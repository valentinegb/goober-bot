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

use std::fmt::{self, Debug};

use poise::{
    serenity_prelude::{self, Color, CreateAllowedMentions, CreateEmbed, Mentionable},
    CreateReply, FrameworkError,
};
use tracing::{error, warn};

use crate::emoji::*;

#[derive(Debug)]
pub(crate) struct UserError(pub(crate) anyhow::Error);

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "User performed an action improperly")
    }
}

impl std::error::Error for UserError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.0.as_ref())
    }
}

pub(super) async fn on_error(
    error: FrameworkError<'_, impl Debug, anyhow::Error>,
) -> Result<(), serenity_prelude::Error> {
    match error {
        FrameworkError::Command { error, ctx, .. } => {
            if let Some(downcasted_error) = error.downcast_ref() {
                let user_error: &UserError = downcasted_error;

                warn!("{error:#}");

                ctx.send(
                    CreateReply::default()
                        .embed(
                            CreateEmbed::new()
                                .title(format!("User Error {FLOOF_LOAD_ANIMATED}"))
                                .description(format!("{:?}", user_error.0))
                                .color(Color::GOLD),
                        )
                        .allowed_mentions(CreateAllowedMentions::new())
                        .ephemeral(true),
                )
                .await?;

                return Ok(());
            }

            error!("An error occured in a command: {error:#?}");

            ctx.send(
                CreateReply::default()
                    .embed(
                        CreateEmbed::new()
                            .title(format!("Command Error {FLOOF_LOAD_ANIMATED}"))
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
                            .description("Something went *seriously* wrong- please [join the support server](https://discord.gg/7v2aY2YzJU) and let a developer know!")
                            .color(Color::DARK_RED),
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
                            .title(format!("Argument Parsing Error {FLOOF_LOAD_ANIMATED}"))
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
            warn!(
                "{} invoked `{}` but the bot is missing permissions: {missing_permissions}",
                ctx.author().name,
                ctx.invocation_string(),
            );

            ctx.send(
                CreateReply::default()
                    .embed(
                        CreateEmbed::new()
                            .title(format!("Missing Bot Permissions {FLOOF_NERVOUS}"))
                            .description(format!("I can't execute this command because I don't have these permissions: {missing_permissions}"))
                            .color(Color::GOLD),
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
                                    warn!(
                                        "{} invoked `{}` but the user is missing permissions: {missing_permissions}",
                                        ctx.author().name,
                                        ctx.invocation_string(),
                                    );

                                    format!("You need these permissions to use this command: {missing_permissions}")
                                },
                                None => {
                                    warn!(
                                        "{} invoked `{}` but the user is missing permissions",
                                        ctx.author().name,
                                        ctx.invocation_string(),
                                    );

                                    "I'm not sure what exactly you're missing, but you're missing some permission you need for this command, so I can't let you continue. Sorry!".to_string()
                                },
                            })
                            .color(Color::GOLD),
                    )
                    .ephemeral(true),
            )
            .await?;
        }
        FrameworkError::NotAnOwner { ctx, .. } => {
            warn!(
                "{} invoked `{}` but the user is not an owner",
                ctx.author().name,
                ctx.invocation_string(),
            );

            ctx.send(
                CreateReply::default()
                    .embed(
                        CreateEmbed::new()
                            .title(format!("Not an Owner {FLOOF_PAT}"))
                            .description(format!(
                                "You must be a developer of {} to use this command.",
                                ctx.framework().bot_id.mention(),
                            ))
                            .color(Color::GOLD),
                    )
                    .ephemeral(true),
            )
            .await?;
        }
        other => poise::builtins::on_error(other).await?,
    }

    Ok(())
}
