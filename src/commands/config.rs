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

use paste::paste;
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

// TODO: replace `$name_str` with `stringify!($name)` when it works

/// Remember to add the subcommands to `get` and `set` and add config option to
/// `list`
macro_rules! config_bool {
    (
        #[doc = $desc:literal]
        let $name:ident = ($name_str:literal, $title:literal);
    ) => {
        paste! {
            #[doc = "Gets the " $title " configuration option"]
            #[command(slash_command, rename = $name_str, ephemeral)]
            async fn [<get_ $name>](ctx: Context<'_>) -> Result<(), Error> {
                let Config { $name, .. } = load_or_save_default(ctx, &get_config_key(ctx)?)?;

                ctx.send(
                    CreateReply::default().embed(
                        CreateEmbed::new()
                            .title($title)
                            .description($desc)
                            .field("Current Value", $name.to_string(), false)
                            .timestamp(Timestamp::now())
                            .color(Color::BLUE),
                    ),
                )
                .await?;

                Ok(())
            }

            #[doc = "Sets the " $title " configuration option"]
            #[command(slash_command, rename = $name_str, ephemeral)]
            async fn [<set_ $name>](
                ctx: Context<'_>,
                #[description = "The value to set " $title " to"] value: bool,
            ) -> Result<(), Error> {
                let config_key = get_config_key(ctx)?;
                let mut config: Config = load_or_save_default(ctx, &config_key)?;

                config.$name = value;
                ctx.data().persist.save(&config_key, config)?;
                ctx.say(format!(
                    "**{}** has been set to **{value}** {FLOOF_HAPPY}",
                    $title,
                ))
                .await?;

                Ok(())
            }
        }
    };
}

/// Remember to add the subcommands to `get`, `set`, and `unset` and add config
/// option to `list`
macro_rules! config_channel {
    (
        #[doc = $desc:literal]
        let $name:ident = ($name_str:literal, $title:literal);
    ) => {
        paste! {
            #[doc = "Gets the " $title " configuration option"]
            #[command(slash_command, rename = $name_str, ephemeral)]
            async fn [<get_ $name>](ctx: Context<'_>) -> Result<(), Error> {
                let Config {
                    $name,
                    ..
                } = load_or_save_default(ctx, &get_config_key(ctx)?)?;

                ctx.send(
                    CreateReply::default().embed(
                        CreateEmbed::new()
                            .title($title)
                            .description($desc)
                            .field(
                                "Current Value",
                                map_or_none_string($name, |v| v.mention().to_string()),
                                false,
                            )
                            .timestamp(Timestamp::now())
                            .color(Color::BLUE),
                    ),
                )
                .await?;

                Ok(())
            }

            #[doc = "Sets the " $title " configuration option"]
            #[command(slash_command, rename = $name_str, ephemeral)]
            async fn [<set_ $name>](
                ctx: Context<'_>,
                #[description = "The value to set " $title " to"]
                #[channel_types("Text")]
                value: ChannelId,
            ) -> Result<(), Error> {
                let config_key = get_config_key(ctx)?;
                let mut config: Config = load_or_save_default(ctx, &config_key)?;

                config.$name = Some(value);
                ctx.data().persist.save(&config_key, config)?;
                ctx.say(format!(
                    "**{}** has been set to **{}** {FLOOF_HAPPY}",
                    $title,
                    value.mention(),
                ))
                .await?;

                Ok(())
            }

            #[doc = "Unsets the " $title " configuration option"]
            #[command(slash_command, rename = $name_str, ephemeral)]
            async fn [<unset_ $name>](ctx: Context<'_>) -> Result<(), Error> {
                let config_key = get_config_key(ctx)?;
                let mut config: Config = load_or_save_default(ctx, &config_key)?;

                config.$name = None;
                ctx.data().persist.save(&config_key, config)?;
                ctx.say(format!(
                    "**{}** has been **unset** {FLOOF_HAPPY}",
                    $title,
                ))
                .await?;

                Ok(())
            }
        }
    };
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
    let mut embed = CreateEmbed::new()
        .title("Configuration")
        .description("These are the configuration options for this server. Use `/config get <option>` to get more information about an option.")
        .timestamp(Timestamp::now())
        .color(Color::BLUE);

    macro_rules! list_bool {
        ($title:literal, $name:ident) => {
            embed = embed.field($title, config.$name.to_string(), false);
        };
    }

    macro_rules! list_channel {
        ($title:literal, $name:ident) => {
            embed = embed.field(
                $title,
                map_or_none_string(config.$name, |v| v.mention().to_string()),
                false,
            );
        };
    }

    list_bool!("Strikes Enabled", strikes_enabled);
    list_channel!("Strikes Log Channel", strikes_log_channel);
    list_bool!("Anon Enabled", anon_enabled);
    list_channel!("Anon Channel", anon_channel);
    list_channel!("Anon Log Channel", anon_log_channel);

    ctx.send(CreateReply::default().embed(embed)).await?;

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

config_bool! {
    /// Whether to enable the strikes moderation system, `/strike`, and its subcommands
    let strikes_enabled = ("strikes_enabled", "Strikes Enabled");
}

config_channel! {
    /// Channel to log strike events in
    let strikes_log_channel = ("strikes_log_channel", "Strikes Log Channel");
}

config_bool! {
    /// Whether to enable the `/anon` command, which allows members to send messages anonymously
    let anon_enabled = ("anon_enabled", "Anon Enabled");
}

config_channel! {
    /// Channel to restrict `/anon` to, if anon is enabled
    let anon_channel = ("anon_channel", "Anon Channel");
}

config_channel! {
    /// Channel to log `/anon` uses to, if anon is enabled
    let anon_log_channel = ("anon_log_channel", "Anon Log Channel");
}
