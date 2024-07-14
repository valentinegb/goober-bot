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

use poise::{
    command,
    serenity_prelude::{ChannelId, Color, CreateEmbed, Mentionable, Timestamp},
    CreateReply,
};

use crate::{
    config::{get_config_key, Config},
    emoji::*,
    persist::load_or_save_default,
    Context, Error,
};

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
    required_bot_permissions = "USE_EXTERNAL_EMOJIS",
    default_member_permissions = "MANAGE_GUILD"
)]
pub(crate) async fn config(_ctx: Context<'_>) -> Result<(), Error> {
    unreachable!()
}

/// Lists all configuration options for this server
#[command(slash_command, ephemeral)]
async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let config: Config = load_or_save_default(ctx, &get_config_key(ctx)?)?;

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title("Configuration")
                .description("These are the configuration options for this server. Use `/config get <option>` to get more information about an option.")
                .field("Strikes Enabled", config.strikes_enabled.to_string(), false)
                .field("Strikes Log Channel", map_or_none_string(config.strikes_log_channel, |v| v.mention().to_string()), false)
                .field("Anon Enabled", config.anon_enabled.to_string(), false)
                .field("Anon Channel", map_or_none_string(config.anon_channel, |v| v.mention().to_string()), false)
                .field("Anon Log Channel", map_or_none_string(config.anon_log_channel, |v| v.mention().to_string()), false)
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
    subcommands(
        "get_strikes_enabled",
        "get_strikes_log_channel",
        "get_anon_enabled",
        "get_anon_channel",
        "get_anon_log_channel",
    )
)]
async fn get(_ctx: Context<'_>) -> Result<(), Error> {
    unreachable!()
}

/// Gets the Strikes Enabled configuration option
#[command(slash_command, rename = "strikes_enabled", ephemeral)]
async fn get_strikes_enabled(ctx: Context<'_>) -> Result<(), Error> {
    let Config {
        strikes_enabled, ..
    } = load_or_save_default(ctx, &get_config_key(ctx)?)?;

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
    } = load_or_save_default(ctx, &get_config_key(ctx)?)?;

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

/// Gets the Anon Enabled configuration option
#[command(slash_command, rename = "anon_enabled", ephemeral)]
async fn get_anon_enabled(ctx: Context<'_>) -> Result<(), Error> {
    let Config { anon_enabled, .. } = load_or_save_default(ctx, &get_config_key(ctx)?)?;

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title("Anon Enabled")
                .description("Whether to enable the `/anon` command, which allows members to send messages anonymously")
                .field("Value", anon_enabled.to_string(), false)
                .timestamp(Timestamp::now())
                .color(Color::BLUE),
        ),
    )
    .await?;

    Ok(())
}

/// Gets the Anon Channel configuration option
#[command(slash_command, rename = "anon_channel", ephemeral)]
async fn get_anon_channel(ctx: Context<'_>) -> Result<(), Error> {
    let Config { anon_channel, .. } = load_or_save_default(ctx, &get_config_key(ctx)?)?;

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title("Anon Channel")
                .description("Channel to restrict `/anon` to, if anon is enabled")
                .field(
                    "Value",
                    map_or_none_string(anon_channel, |v| v.mention().to_string()),
                    false,
                )
                .timestamp(Timestamp::now())
                .color(Color::BLUE),
        ),
    )
    .await?;

    Ok(())
}

/// Gets the Anon Log Channel configuration option
#[command(slash_command, rename = "anon_log_channel", ephemeral)]
async fn get_anon_log_channel(ctx: Context<'_>) -> Result<(), Error> {
    let Config {
        anon_log_channel, ..
    } = load_or_save_default(ctx, &get_config_key(ctx)?)?;

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title("Anon Log Channel")
                .description("Channel to log `/anon` uses to, if anon is enabled")
                .field(
                    "Value",
                    map_or_none_string(anon_log_channel, |v| v.mention().to_string()),
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
    subcommands(
        "set_strikes_enabled",
        "set_strikes_log_channel",
        "set_anon_enabled",
        "set_anon_channel",
        "set_anon_log_channel",
    )
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
    let config_key = get_config_key(ctx)?;
    let mut config: Config = load_or_save_default(ctx, &config_key)?;

    config.strikes_enabled = value;
    ctx.data().persist.save(&config_key, config)?;
    ctx.say(format!(
        "**Strikes Enabled** has been set to **{value}** {FLOOF_HAPPY}"
    ))
    .await?;

    Ok(())
}

/// Sets the Strikes Log Channel configuration option
#[command(slash_command, rename = "strikes_log_channel", ephemeral)]
async fn set_strikes_log_channel(
    ctx: Context<'_>,
    #[description = "The value to set Strikes Log Channel to"]
    #[channel_types("Text")]
    value: ChannelId,
) -> Result<(), Error> {
    let config_key = get_config_key(ctx)?;
    let mut config: Config = load_or_save_default(ctx, &config_key)?;

    config.strikes_log_channel = Some(value);
    ctx.data().persist.save(&config_key, config)?;
    ctx.say(format!(
        "**Strikes Log Channel** has been set to **{}** {FLOOF_HAPPY}",
        value.mention(),
    ))
    .await?;

    Ok(())
}

/// Sets the Anon Enabled configuration option
#[command(slash_command, rename = "anon_enabled", ephemeral)]
async fn set_anon_enabled(
    ctx: Context<'_>,
    #[description = "The value to set Anon Enabled to"] value: bool,
) -> Result<(), Error> {
    let config_key = get_config_key(ctx)?;
    let mut config: Config = load_or_save_default(ctx, &config_key)?;

    config.anon_enabled = value;
    ctx.data().persist.save(&config_key, config)?;
    ctx.say(format!(
        "**Anon Enabled** has been set to **{value}** {FLOOF_HAPPY}"
    ))
    .await?;

    Ok(())
}

/// Sets the Anon Channel configuration option
#[command(slash_command, rename = "anon_channel", ephemeral)]
async fn set_anon_channel(
    ctx: Context<'_>,
    #[description = "The value to set Anon Channel to"]
    #[channel_types("Text")]
    value: ChannelId,
) -> Result<(), Error> {
    let config_key = get_config_key(ctx)?;
    let mut config: Config = load_or_save_default(ctx, &config_key)?;

    config.anon_channel = Some(value);
    ctx.data().persist.save(&config_key, config)?;
    ctx.say(format!(
        "**Anon Channel** has been set to **{}** {FLOOF_HAPPY}",
        value.mention(),
    ))
    .await?;

    Ok(())
}

/// Sets the Anon Log Channel configuration option
#[command(slash_command, rename = "anon_log_channel", ephemeral)]
async fn set_anon_log_channel(
    ctx: Context<'_>,
    #[description = "The value to set Anon Log Channel to"]
    #[channel_types("Text")]
    value: ChannelId,
) -> Result<(), Error> {
    let config_key = get_config_key(ctx)?;
    let mut config: Config = load_or_save_default(ctx, &config_key)?;

    config.anon_log_channel = Some(value);
    ctx.data().persist.save(&config_key, config)?;
    ctx.say(format!(
        "**Anon Log Channel** has been set to **{}** {FLOOF_HAPPY}",
        value.mention(),
    ))
    .await?;

    Ok(())
}

/// Unsets a specific configuration option
#[command(
    slash_command,
    subcommands(
        "unset_strikes_log_channel",
        "unset_anon_channel",
        "unset_anon_log_channel",
    )
)]
async fn unset(_ctx: Context<'_>) -> Result<(), Error> {
    unreachable!()
}

/// Unsets the Strikes Log Channel configuration option
#[command(slash_command, rename = "strikes_log_channel", ephemeral)]
async fn unset_strikes_log_channel(ctx: Context<'_>) -> Result<(), Error> {
    let config_key = get_config_key(ctx)?;
    let mut config: Config = load_or_save_default(ctx, &config_key)?;

    config.strikes_log_channel = None;
    ctx.data().persist.save(&config_key, config)?;
    ctx.say("**Strikes Log Channel** has been **unset** {FLOOF_HAPPY}")
        .await?;

    Ok(())
}

/// Unsets the Anon Channel configuration option
#[command(slash_command, rename = "anon_channel", ephemeral)]
async fn unset_anon_channel(ctx: Context<'_>) -> Result<(), Error> {
    let config_key = get_config_key(ctx)?;
    let mut config: Config = load_or_save_default(ctx, &config_key)?;

    config.anon_channel = None;
    ctx.data().persist.save(&config_key, config)?;
    ctx.say("**Anon Channel** has been **unset** {FLOOF_HAPPY}")
        .await?;

    Ok(())
}

/// Unsets the Anon Log Channel configuration option
#[command(slash_command, rename = "anon_log_channel", ephemeral)]
async fn unset_anon_log_channel(ctx: Context<'_>) -> Result<(), Error> {
    let config_key = get_config_key(ctx)?;
    let mut config: Config = load_or_save_default(ctx, &config_key)?;

    config.anon_log_channel = None;
    ctx.data().persist.save(&config_key, config)?;
    ctx.say("**Anon Log Channel** has been **unset** {FLOOF_HAPPY}")
        .await?;

    Ok(())
}
