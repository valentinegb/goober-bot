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
    serenity_prelude::{
        ChannelId, ChannelType, Color, CreateEmbed, GuildChannel, Mentionable, Timestamp,
    },
    CreateReply,
};
use serde::{Deserialize, Serialize};
use shuttle_persist_msgpack::PersistError;

use crate::{Context, Error};

#[derive(Deserialize, Serialize, Default)]
#[non_exhaustive]
#[serde(default)]
struct Config {
    strikes_enabled: bool,
    strikes_log_channel: Option<ChannelId>,
}

/// Gets the config key for the server in `ctx`.
fn get_config_key(ctx: Context<'_>) -> Result<String, Error> {
    Ok(format!(
        "config_{}",
        ctx.guild_id().context("Expected context to be in guild")?
    ))
}

/// Saves a config for the server in `ctx`.
fn save_config(ctx: Context<'_>, config: Config) -> Result<(), Error> {
    Ok(ctx.data().persist.save(&get_config_key(ctx)?, config)?)
}

/// Attempts to load the config for the server in `ctx`. If a configuration is
/// not found, will attempt to save the default configuration once and try to
/// load the configuration again.
fn load_or_save_default_config(ctx: Context<'_>) -> Result<Config, Error> {
    let data = ctx.data();
    let config_key = get_config_key(ctx)?;

    Ok(match data.persist.load(&config_key) {
        Ok(config) => config,
        Err(err) => match err {
            PersistError::Open(ref io_err) => match io_err.kind() {
                std::io::ErrorKind::NotFound => {
                    save_config(ctx, Config::default())?;

                    data.persist.load(&config_key)?
                }
                _ => bail!(err),
            },
            _ => bail!(err),
        },
    })
}

/// Returns "None" as a [`String`] (if none), or applies a function to the
/// contained value (if any).
///
/// See [`Option::map_or_else`].
fn map_or_none_string<T, F>(option: Option<T>, f: F) -> String
where
    F: FnOnce(T) -> String,
{
    option.map_or_else(|| "None".to_string(), f)
}

/// Subcommands related to getting and setting server configuration
#[command(
    slash_command,
    subcommands("list", "get", "set", "unset"),
    install_context = "Guild",
    interaction_context = "Guild",
    default_member_permissions = "MANAGE_GUILD"
)]
pub(crate) async fn config(_ctx: Context<'_>) -> Result<(), Error> {
    unreachable!()
}

/// Lists all configuration options for this server
#[command(slash_command, ephemeral)]
async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let config = load_or_save_default_config(ctx)?;

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title("Configuration")
                .description("These are the configuration options for this server. Use `/config get <option>` to get more information about an option.")
                .field("Strikes Enabled", config.strikes_enabled.to_string(), false)
                .field("Strikes Log Channel", map_or_none_string(config.strikes_log_channel, |v| v.mention().to_string()), false)
                .timestamp(Timestamp::now())
                .color(Color::BLUE),
        ),
    )
    .await?;

    Ok(())
}

/// Gets a specific configuration option
#[command(
    slash_command,
    subcommands("get_strikes_enabled", "get_strikes_log_channel")
)]
async fn get(_ctx: Context<'_>) -> Result<(), Error> {
    unreachable!()
}

/// Gets the Strikes Enabled configuration option
#[command(slash_command, rename = "strikes_enabled", ephemeral)]
async fn get_strikes_enabled(ctx: Context<'_>) -> Result<(), Error> {
    let Config {
        strikes_enabled, ..
    } = load_or_save_default_config(ctx)?;

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title("Strikes Enabled")
                .description("Whether to enable the strikes moderation system, `/strike`, and its subcommands")
                .field("Value", strikes_enabled.to_string(), false)
                .timestamp(Timestamp::now())
                .color(Color::BLUE),
        ),
    )
    .await?;

    Ok(())
}

/// Gets the Strikes Log Channel configuration option
#[command(slash_command, rename = "strikes_log_channel", ephemeral)]
async fn get_strikes_log_channel(ctx: Context<'_>) -> Result<(), Error> {
    let Config {
        strikes_log_channel,
        ..
    } = load_or_save_default_config(ctx)?;

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title("Strikes Log Channel")
                .description("Channel to log strike events in")
                .field(
                    "Value",
                    map_or_none_string(strikes_log_channel, |v| v.mention().to_string()),
                    false,
                )
                .timestamp(Timestamp::now())
                .color(Color::BLUE),
        ),
    )
    .await?;

    Ok(())
}

/// Sets a specific configuration option
#[command(
    slash_command,
    subcommands("set_strikes_enabled", "set_strikes_log_channel")
)]
async fn set(_ctx: Context<'_>) -> Result<(), Error> {
    unreachable!()
}

/// Sets the Strikes Enabled configuration option
#[command(slash_command, rename = "strikes_enabled", ephemeral)]
async fn set_strikes_enabled(
    ctx: Context<'_>,
    #[description = "The value to set Strikes Enabled to"] value: bool,
) -> Result<(), Error> {
    let mut config = load_or_save_default_config(ctx)?;

    config.strikes_enabled = value;
    save_config(ctx, config)?;
    ctx.say(format!("**Strikes Enabled** has been set to **{value}**"))
        .await?;

    Ok(())
}

/// Sets the Strikes Log Channel configuration option
#[command(slash_command, rename = "strikes_log_channel", ephemeral)]
async fn set_strikes_log_channel(
    ctx: Context<'_>,
    #[description = "The value to set Strikes Log Channel to"] value: GuildChannel,
) -> Result<(), Error> {
    match value.kind {
        ChannelType::Text => (),
        _ => bail!("Value must be a text channel"),
    }

    let mut config = load_or_save_default_config(ctx)?;

    config.strikes_log_channel = Some(value.id);
    save_config(ctx, config)?;
    ctx.say(format!(
        "**Strikes Log Channel** has been set to **{}**",
        value.mention(),
    ))
    .await?;

    Ok(())
}

/// Unsets a specific configuration option
#[command(slash_command, subcommands("unset_strikes_log_channel"))]
async fn unset(_ctx: Context<'_>) -> Result<(), Error> {
    unreachable!()
}

/// Unsets the Strikes Log Channel configuration option
#[command(slash_command, rename = "strikes_log_channel", ephemeral)]
async fn unset_strikes_log_channel(ctx: Context<'_>) -> Result<(), Error> {
    let mut config = load_or_save_default_config(ctx)?;

    config.strikes_log_channel = None;
    save_config(ctx, config)?;
    ctx.say("**Strikes Log Channel** has been **unset**")
        .await?;

    Ok(())
}
