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

mod activity;
mod commands;
mod config;
mod emoji;
mod persist;

use activity::start_activity_loop;
use anyhow::Context as _;
use emoji::*;
use poise::{
    serenity_prelude::{
        self, ClientBuilder, Color, CreateAllowedMentions, CreateEmbed, GatewayIntents,
    },
    CreateReply, Framework, FrameworkError, FrameworkOptions,
};
use shuttle_persist_msgpack::PersistInstance;
use shuttle_runtime::{CustomError, SecretStore};
use shuttle_serenity::ShuttleSerenity;
use tracing::error;

/// User data, which is stored and accessible in all command invocations
struct Data {
    persist: PersistInstance,
}

type Error = anyhow::Error;
type Context<'a> = poise::Context<'a, Data, Error>;

pub async fn on_error<U, E: std::fmt::Display + std::fmt::Debug>(
    error: FrameworkError<'_, U, E>,
) -> Result<(), serenity_prelude::Error> {
    match error {
        FrameworkError::Command { error, ctx, .. } => {
            let error = error.to_string();

            error!("An error occured in a command: {error}");

            ctx.send(
                CreateReply::default()
                    .embed(
                        CreateEmbed::new()
                            .title(format!("Error {A_FLOOF_LOAD}"))
                            .description(error)
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
                            .description("An unexpected internal error has occurred")
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

            error!("An argument parsing error occured{for_input}: {error}");

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
            ctx.send(
                CreateReply::default()
                    .embed(
                        CreateEmbed::new()
                            .title(format!("Missing Bot Permissions {FLOOF_NERVOUS}"))
                            .description(format!("Command cannot be executed because the bot is lacking permissions: {missing_permissions}"))
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
    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;
    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: vec![commands::config(), commands::strike()],
            on_error: |error| {
                Box::pin(async move {
                    if let Err(e) = on_error(error).await {
                        error!("Error while handling error: {e}");
                    }
                })
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                start_activity_loop(ctx.clone());
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

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
