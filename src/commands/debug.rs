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

use anyhow::anyhow;
use poise::command;

use crate::{config::get_config_key, emoji::*, Context, Error};

/// Commands to aid in development of the bot
#[command(slash_command, subcommands("error", "delete_config"))]
pub async fn debug(_ctx: Context<'_>) -> Result<(), Error> {
    unreachable!();
}

/// Fails intentionally
#[command(slash_command)]
async fn error(_ctx: Context<'_>) -> Result<(), Error> {
    Err(anyhow!("This is a test error").context("This is a wrapper test error"))
}

/// Deletes the config file for the current server
#[command(
    slash_command,
    required_permissions = "MANAGE_GUILD",
    required_bot_permissions = "USE_EXTERNAL_EMOJIS",
    ephemeral
)]
async fn delete_config(ctx: Context<'_>) -> Result<(), Error> {
    ctx.data().persist.remove(&get_config_key(ctx)?)?;
    ctx.say(format!("Server config file deleted {FLOOF_MUG}"))
        .await?;

    Ok(())
}