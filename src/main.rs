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

mod activity;
mod commands;
mod config;
mod emoji;
mod persist;

use std::fmt::Debug;

use activity::start_activity_loop;
use anyhow::Context as _;
use emoji::*;
use octocrab::Octocrab;
use poise::{
    serenity_prelude::{
        self, ClientBuilder, Color, CreateAllowedMentions, CreateEmbed, GatewayIntents,
    },
    CreateReply, Framework, FrameworkError, FrameworkOptions,
};
use shuttle_persist_msgpack::PersistInstance;
use shuttle_runtime::{CustomError, SecretStore};
use shuttle_serenity::ShuttleSerenity;
use tracing::{error, info, warn};

/// User data, which is stored and accessible in all command invocations
#[derive(Debug)]
struct Data {
    persist: PersistInstance,
}

type Error = anyhow::Error;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn on_error<U: Debug, E: std::fmt::Display + std::fmt::Debug>(
    error: FrameworkError<'_, U, E>,
) -> Result<(), serenity_prelude::Error> {
    match error {
        FrameworkError::Command { error, ctx, .. } => {
            error!("An error occured in a command: {error:#?}");

            ctx.send(
                CreateReply::default()
                    .embed(
                        CreateEmbed::new()
                            .title(format!("Command Error {A_FLOOF_LOAD}"))
                            .description(format!("{error:?}"))
                            .color(Color::RED),
                    )
                    .allowed_mentions(CreateAllowedMentions::new())
                    .ephemeral(true),
            )
            .await?;
        }
        FrameworkError::CommandPanic {
            payload: _, ctx, ..
        } => {
            // Not showing the payload to the user because it may contain sensitive info
            ctx.send(
                CreateReply::default()
                    .embed(
                        CreateEmbed::new()
                            .title(format!("Internal Error {FLOOF_NERVOUS}"))
                            .description("Something went *seriously* wrong- please join the support server and let a developer know!")
                            .color(Color::RED),
                    )
                    .ephemeral(true),
            )
            .await?;
        }
        FrameworkError::ArgumentParse {
            error, input, ctx, ..
        } => {
            let for_input = match input {
                Some(input) => format!(" for input \"{input}\""),
                None => String::new(),
            };

            error!("An argument parsing error occured{for_input}: {error}: {ctx:#?}");

            ctx.send(
                CreateReply::default()
                    .embed(
                        CreateEmbed::new()
                            .title(format!("Argument Parsing Error {A_FLOOF_LOAD}"))
                            .description("There's probably been an update to this command recently. Please try running it again in a few seconds.")
                            .color(Color::RED),
                    )
                    .ephemeral(true),
            )
            .await?;
        }
        FrameworkError::MissingBotPermissions {
            missing_permissions,
            ctx,
            ..
        } => {
            warn!("Missing bot permissions: {missing_permissions}: {ctx:#?}");

            ctx.send(
                CreateReply::default()
                    .embed(
                        CreateEmbed::new()
                            .title(format!("Missing Bot Permissions {FLOOF_NERVOUS}"))
                            .description(format!("I can't execute this command because I don't have these permissions: {missing_permissions}"))
                            .color(Color::RED),
                    )
                    .ephemeral(true),
            )
            .await?;
        }
        FrameworkError::MissingUserPermissions {
            missing_permissions,
            ctx,
            ..
        } => {
            ctx.send(
                CreateReply::default()
                    .embed(
                        CreateEmbed::new()
                            .title(format!("Missing User Permissions {FLOOF_NERVOUS}"))
                            .description(match missing_permissions {
                                Some(missing_permissions) => {
                                    warn!("Missing user permissions: {missing_permissions}: {ctx:#?}");

                                    format!("You need these permissions to use this command: {missing_permissions}")
                                },
                                None => {
                                    warn!("Missing user permissions: {ctx:#?}");

                                    "I'm not sure what exactly you're missing, but you're missing some permission you need for this command, so I can't let you continue. Sorry!".to_string()
                                },
                            })
                            .color(Color::RED),
                    )
                    .ephemeral(true),
            )
            .await?;
        }
        other => poise::builtins::on_error(other).await?,
    }

    Ok(())
}

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
    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: vec![
                commands::anon(),
                commands::bite(),
                commands::boop(),
                commands::config(),
                commands::debug(),
                commands::gnaw(),
                commands::kiss(),
                commands::meow(),
                commands::murder(),
                commands::pat(),
                commands::rock_paper_scissors(),
                commands::sponsors(),
                commands::strike(),
            ],
            on_error: |error| {
                Box::pin(async move {
                    if let Err(e) = on_error(error).await {
                        error!("Error while handling error: {e}");
                    }
                })
            },
            pre_command: |ctx| {
                Box::pin(async move {
                    info!("Command initiated: {ctx:#?}");
                })
            },
            post_command: |ctx| {
                Box::pin(async move {
                    info!("Command finished successfully: {ctx:#?}");
                })
            },
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

                Ok(Data { persist })
            })
        })
        .build();
    let client = ClientBuilder::new(discord_token, GatewayIntents::non_privileged())
        .framework(framework)
        .await
        .map_err(CustomError::new)?;

    Ok(client.into())
}
