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
mod database;
mod emoji;
mod error;
mod monetary;

#[cfg(not(debug_assertions))]
use std::time::Duration;
use std::{collections::HashSet, fmt::Debug};

use analytics::analytics;
use anyhow::Context as _;
use config::config;
use poise::{
    serenity_prelude::{ClientBuilder, GatewayIntents, UserId},
    Framework, FrameworkOptions,
};
use shuttle_runtime::{CustomError, SecretStore};
use shuttle_serenity::ShuttleSerenity;
use shuttle_shared_db::SerdeJsonOperator;
#[cfg(not(debug_assertions))]
use tokio::spawn;
#[cfg(not(debug_assertions))]
use topgg::Autoposter;
use tracing::{error, info};

use crate::activity::start_activity_loop;

/// User data, which is stored and accessible in all command invocations
#[derive(Debug)]
struct Data {
    op: SerdeJsonOperator,
    #[cfg(not(debug_assertions))]
    topgg_client: topgg::Client,
    buy_me_a_coffee_client: buy_me_a_coffee::Client,
}

type Context<'a> = poise::Context<'a, Data, anyhow::Error>;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
    #[shuttle_shared_db::Postgres] op: SerdeJsonOperator,
) -> ShuttleSerenity {
    tracing_subscriber::fmt()
        .with_env_filter("goober_bot=debug,info")
        .without_time()
        .init();

    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("`DISCORD_TOKEN` was not found")?;
    #[cfg(not(debug_assertions))]
    let topgg_token = secret_store
        .get("TOPGG_TOKEN")
        .context("`TOPGG_TOKEN` was not found")?;
    let buy_me_a_coffee_pat = secret_store
        .get("BUY_ME_A_COFFEE_PAT")
        .context("`BUY_ME_A_COFFEE_PAT` was not found")?;
    #[cfg(not(debug_assertions))]
    let topgg_client = topgg::Client::new(topgg_token);
    #[cfg(not(debug_assertions))]
    let mut autoposter = Autoposter::serenity(&topgg_client, Duration::from_secs(1800));
    let buy_me_a_coffee_client = buy_me_a_coffee::Client::new(buy_me_a_coffee_pat);
    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: vec![
                analytics(),
                commands::anon(),
                commands::bap(),
                commands::bite(),
                commands::boop(),
                commands::carry(),
                commands::debug(),
                commands::gnaw(),
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
            on_error: |error| {
                Box::pin(async move {
                    if let Err(e) = error::on_error(error).await {
                        error!("Error while handling error: {e}");
                    }
                })
            },
            pre_command: |ctx| {
                Box::pin(async move {
                    if let Err(err) = analytics::increment(ctx).await {
                        error!("An error occured whilst performing analytics: {err:#?}");
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
                    op,
                    #[cfg(not(debug_assertions))]
                    topgg_client,
                    buy_me_a_coffee_client,
                })
            })
        })
        .build();
    #[allow(unused_mut)]
    let mut client_builder =
        ClientBuilder::new(discord_token, GatewayIntents::GUILDS).framework(framework);

    #[cfg(not(debug_assertions))]
    {
        client_builder = client_builder.event_handler_arc(autoposter.handler());

        info!("Top.gg autoposter handler passed to client builder");
        spawn(async move {
            loop {
                if let Some(result) = autoposter.recv().await {
                    match result {
                        Ok(_) => info!("Autoposter posted stats successfully"),
                        Err(err) => error!("Autoposter returned an error: {err:#?}"),
                    }
                }
            }
        });
        info!("Began awaiting Top.gg autoposter responses");
    }

    let client = client_builder.await.map_err(CustomError::new)?;

    Ok(client.into())
}
