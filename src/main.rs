// Goober Bot, Discord bot
// Copyright (C) 2024  Valentine Briese
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
mod commands;
mod config;
mod emoji;
mod error;
mod persist;
mod sponsors;

pub(crate) use crate::error::Error;

#[cfg(not(debug_assertions))]
use std::time::Duration;
use std::{collections::HashSet, fmt::Debug};

use anyhow::Context as _;
use config::config;
use octocrab::Octocrab;
use poise::{
    serenity_prelude::{ClientBuilder, GatewayIntents, UserId},
    Framework, FrameworkOptions,
};
use shuttle_persist_msgpack::PersistInstance;
use shuttle_runtime::{CustomError, SecretStore};
use shuttle_serenity::ShuttleSerenity;
#[cfg(not(debug_assertions))]
use tokio::spawn;
#[cfg(not(debug_assertions))]
use topgg::Autoposter;
use tracing::{error, info};

use crate::activity::start_activity_loop;

/// User data, which is stored and accessible in all command invocations
#[derive(Debug)]
struct Data {
    persist: PersistInstance,
    #[cfg(not(debug_assertions))]
    topgg_client: topgg::Client,
}

type Context<'a> = poise::Context<'a, Data, Error>;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
    #[shuttle_persist_msgpack::Persist] persist: PersistInstance,
) -> ShuttleSerenity {
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("`DISCORD_TOKEN` was not found")?;
    let github_pat = secret_store
        .get("GITHUB_PAT")
        .context("`GITHUB_PAT` was not found")?;
    #[cfg(not(debug_assertions))]
    let topgg_token = secret_store
        .get("TOPGG_TOKEN")
        .context("`TOPGG_TOKEN` was not found")?;
    #[cfg(not(debug_assertions))]
    let topgg_client = topgg::Client::new(topgg_token);
    #[cfg(not(debug_assertions))]
    let mut autoposter = Autoposter::serenity(&topgg_client, Duration::from_secs(1800));
    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: vec![
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
                commands::revive(),
                commands::rock_paper_scissors(),
                commands::slap(),
                commands::sponsors(),
                commands::strike(),
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
                info!("Activity loop started");
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                info!("Commands registered");
                octocrab::initialise(Octocrab::builder().personal_token(github_pat).build()?);
                info!("GitHub authenticated");

                Ok(Data {
                    persist,
                    #[cfg(not(debug_assertions))]
                    topgg_client,
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
