// Goober Bot, Discord bot
// Copyright (C) 2025  Valentine Briese
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// TODO: make general refinements to existing codebase
// TODO: add birthday announcements system
// TODO: replace mentions of specific commands with actual formatted command
//       mentions when https://github.com/serenity-rs/poise/issues/235 is
//       resolved

mod activity;
mod analytics;
mod commands;
mod config;
mod diesel_migration;
mod emoji;
mod models;
mod monetary;
mod schema;

use std::collections::HashSet;

use analytics::analytics;
use config::config;
use diesel_async::{
    AsyncPgConnection,
    async_connection_wrapper::AsyncConnectionWrapper,
    pooled_connection::deadpool::{self, Object},
};
use diesel_migration::migrate_opendal_to_diesel;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use poise::{
    Framework, FrameworkOptions,
    serenity_prelude::{ClientBuilder, GatewayIntents, UserId},
};
use poise_error::{
    anyhow::{Context as _, anyhow},
    on_error,
};
use shuttle_runtime::{CustomError, SecretStore};
use shuttle_serenity::ShuttleSerenity;
use tokio::task::spawn_blocking;
use tracing::{error, info};

use crate::activity::start_activity_loop;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

/// User data, which is stored and accessible in all command invocations
struct Data {
    pool: deadpool::Pool<AsyncPgConnection>,
    #[cfg(not(debug_assertions))]
    topgg_client: topgg::Client,
}

type Context<'a> = poise::Context<'a, Data, poise_error::anyhow::Error>;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
    #[shuttle_shared_db::Postgres] pool: deadpool::Pool<AsyncPgConnection>,
) -> ShuttleSerenity {
    tracing_subscriber::fmt()
        .with_env_filter("goober_bot=debug,info")
        .without_time()
        .init();

    let conn: AsyncPgConnection = { Object::take(pool.get().await.map_err(CustomError::new)?) };
    let mut async_wrapper: AsyncConnectionWrapper<AsyncPgConnection> =
        AsyncConnectionWrapper::from(conn);

    spawn_blocking(move || {
        async_wrapper
            .run_pending_migrations(MIGRATIONS)
            .map_err(|err| anyhow!(err))?;

        Ok::<_, CustomError>(())
    })
    .await
    .map_err(CustomError::new)??;
    migrate_opendal_to_diesel(&pool).await?;

    #[cfg(not(debug_assertions))]
    let topgg_client = {
        let topgg_token = secret_store
            .get("TOPGG_TOKEN")
            .context("`TOPGG_TOKEN` was not found")?;

        topgg::Client::new(topgg_token)
    };
    let client_builder = {
        let discord_token = secret_store
            .get("DISCORD_TOKEN")
            .context("`DISCORD_TOKEN` was not found")?;

        ClientBuilder::new(discord_token, GatewayIntents::GUILDS)
    };
    #[cfg(not(debug_assertions))]
    let client_builder = {
        use std::time::Duration;

        use topgg::Autoposter;

        info!("Bot will post stats to Top.gg");

        let autoposter = Autoposter::serenity(&topgg_client, Duration::from_secs(1800));

        client_builder.event_handler_arc(autoposter.handler())
    };
    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: vec![
                analytics(),
                commands::anon(),
                commands::bap(),
                commands::bite(),
                commands::blow_up(),
                commands::boop(),
                commands::carry(),
                commands::debug(),
                commands::gnaw(),
                commands::hamburger(),
                commands::hug(),
                commands::jumpscare(),
                commands::kiss(),
                commands::meow(),
                commands::murder(),
                commands::pat(),
                commands::poke(),
                commands::revive(),
                commands::rock_paper_scissors(),
                commands::slap(),
                commands::strike(),
                commands::tickle(),
                commands::timestamp(),
                commands::updates(),
                #[cfg(not(debug_assertions))]
                commands::vote(),
                config(),
            ],
            on_error,
            pre_command: |ctx| {
                Box::pin(async move {
                    if let Err(err) = analytics::increment(ctx).await {
                        error!("An error occurred whilst performing analytics: {err:#?}");
                    }

                    info!(
                        "{} invoked `{}`",
                        ctx.author().name,
                        ctx.invocation_string(),
                    );
                })
            },
            post_command: |ctx| {
                Box::pin(async move {
                    info!(
                        "{}'s `{}` invocation finished successfully",
                        ctx.author().name,
                        ctx.invocation_string(),
                    );
                })
            },
            owners: HashSet::from([UserId::new(1016154932354744330)]),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                start_activity_loop(ctx.clone());
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                info!("Commands registered");

                Ok(Data {
                    pool,
                    #[cfg(not(debug_assertions))]
                    topgg_client,
                })
            })
        })
        .build();
    let client_builder = client_builder.framework(framework);
    let client = client_builder.await.map_err(CustomError::new)?;

    Ok(client.into())
}
