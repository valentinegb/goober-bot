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

use anyhow::{bail, Context as _};
use poise::{
    command,
    serenity_prelude::{Color, CreateEmbed},
    CreateReply,
};
use serde::{Deserialize, Serialize};
use shuttle_persist::PersistError;

use crate::{Context, Error};

#[derive(Deserialize, Serialize, Default)]
#[non_exhaustive]
struct Config {
    strikes_enabled: bool,
}

/// Attempts to load the config for the server in `ctx`. If a configuration is
/// not found, will attempt to save the default configuration once and try to
/// load the configuration again.
fn load_or_save_default_config(ctx: Context<'_>) -> Result<Config, Error> {
    let data = ctx.data();
    let config_key = format!(
        "config_{}",
        ctx.guild_id().context("Expected context to be in guild")?
    );

    Ok(match data.persist.load(&config_key) {
        Ok(config) => config,
        Err(err) => match err {
            PersistError::Open(ref io_err) => match io_err.kind() {
                std::io::ErrorKind::NotFound => {
                    data.persist.save(&config_key, Config::default())?;

                    data.persist.load(&config_key)?
                }
                _ => bail!(err),
            },
            _ => bail!(err),
        },
    })
}

/// Subcommands related to getting and setting server configuration
#[command(
    slash_command,
    subcommands("list"/*, "get", "set"*/),
    install_context = "Guild",
    interaction_context = "Guild",
    default_member_permissions = "MANAGE_GUILD"
)]
pub(crate) async fn config(_ctx: Context<'_>) -> Result<(), Error> {
    unreachable!()
}

#[command(slash_command)]
async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let config = load_or_save_default_config(ctx)?;

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title("Configuration")
                .description("These are the configuration options for this server.")
                .field("Strikes Enabled", config.strikes_enabled.to_string(), false)
                .color(Color::BLUE),
        ),
    )
    .await?;

    Ok(())
}
