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

use std::collections::HashSet;

use activity::start_activity_loop;
use analytics::analytics;
use commands::CustomData;
use config::config;
use monetary::has_early_access;
use poise::{
    Framework, FrameworkOptions,
    serenity_prelude::{ClientBuilder, GatewayIntents, UserId},
};
use poise_error::{anyhow::Context as _, on_error};
use shared::Data;
use shuttle_runtime::{CustomError, SecretStore};
use shuttle_serenity::ShuttleSerenity;
use shuttle_shared_db::SerdeJsonOperator;
use tracing::{error, info};

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
    #[shuttle_shared_db::Postgres] op: SerdeJsonOperator,
) -> ShuttleSerenity {
    tracing_subscriber::fmt()
        // If making a new crate, make sure to add it here.
        .with_env_filter("goober_bot=debug,activity=debug,analytics=debug,command_anon=debug,command_debug=debug,command_rock_paper_scissors=debug,command_silly=debug,command_strike=debug,command_timestamp=debug,command_updates=debug,command_updates_proc_macro=debug,command_vote=debug,commands=debug,commands_shared=debug,config=debug,database=debug,emoji=debug,monetary=debug,shared=debug,info")
        .without_time()
        .init();

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
    let autoposter = {
        use std::time::Duration;

        use topgg::Autoposter;

        info!("Bot will post stats to Top.gg");

        Autoposter::serenity(&topgg_client, Duration::from_secs(1800))
    };
    #[cfg(not(debug_assertions))]
    let client_builder = client_builder.event_handler_arc(autoposter.handler());
    let mut commands = vec![
        analytics(),
        commands::anon(),
        commands::arrest(),
        commands::bap(),
        commands::bite(),
        commands::blow_up(),
        commands::boop(),
        commands::carry(),
        commands::debug(),
        commands::defenestrate(),
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
    ];

    for command in commands.iter_mut() {
        if let Some(custom_data) = command.custom_data.downcast_ref::<CustomData>() {
            if custom_data.early_access {
                command.checks.push(|ctx| Box::pin(has_early_access(ctx)));
            }
        }
    }

    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands,
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
                    op,
                    #[cfg(not(debug_assertions))]
                    topgg_client,
                    #[cfg(not(debug_assertions))]
                    _autoposter: autoposter,
                })
            })
        })
        .build();
    let client_builder = client_builder.framework(framework);
    let client = client_builder.await.map_err(CustomError::new)?;

    Ok(client.into())
}
