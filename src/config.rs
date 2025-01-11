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

use anyhow::Context as _;
use paste::paste;
use poise::{
    command,
    serenity_prelude::{ChannelId, Color, CreateEmbed, Mentionable, Timestamp},
    CreateReply, FrameworkError,
};
use serde::{Deserialize, Serialize};

use crate::{database::read_or_write_default, emoji::*, Context, Data, Error};

trait ToConfigString {
    fn to_config_string(&self) -> String;
}

impl ToConfigString for bool {
    fn to_config_string(&self) -> String {
        self.to_string()
    }
}

impl<T: ToConfigString> ToConfigString for Option<T> {
    fn to_config_string(&self) -> String {
        match self {
            Some(t) => t.to_config_string(),
            None => "none".to_string(),
        }
    }
}

impl ToConfigString for ChannelId {
    fn to_config_string(&self) -> String {
        self.mention().to_string()
    }
}

/// Gets the config key for the server in `ctx`.
pub(crate) fn get_config_key(ctx: Context<'_>) -> Result<String, Error> {
    Ok(format!(
        "config_{}",
        ctx.guild_id().context("Expected context to be in guild")?
    ))
}

/// Subcommands related to getting and setting server configuration
#[command(
    slash_command,
    subcommands("list", "get", "set"),
    category = "Config",
    install_context = "Guild",
    interaction_context = "Guild",
    required_bot_permissions = "USE_EXTERNAL_EMOJIS",
    default_member_permissions = "MANAGE_GUILD"
)]
pub(crate) async fn config(_ctx: Context<'_>) -> Result<(), Error> {
    unreachable!()
}

macro_rules! config {
    ($(
        #[doc = $desc:literal]
        let $name:ident: $type:ty = ($name_str:literal, $title:literal);
    )+) => {
        paste! {
            #[derive(Deserialize, Serialize, Default)]
            #[non_exhaustive]
            #[serde(default)]
            pub(crate) struct Config {
                $(pub(crate) $name: $type),+
            }

            /// Lists all configuration options for this server
            #[command(slash_command, ephemeral)]
            async fn list(ctx: Context<'_>) -> Result<(), Error> {
                let config: Config = read_or_write_default(ctx, &get_config_key(ctx)?).await?;

                ctx.send(CreateReply::default().embed(
                    CreateEmbed::new()
                        .title("Configuration")
                        .description("These are the configuration options for this server. Use `/config get <option>` to get more information about an option.")
                        $(.field($title, config.$name.to_config_string(), false))+
                        .timestamp(Timestamp::now())
                        .color(Color::BLUE)
                )).await?;

                Ok(())
            }

            fn get() -> poise::Command<Data, Error> {
                async fn inner(_ctx: Context<'_>) -> Result<(), FrameworkError<'_, Data, Error>> {
                    unreachable!();
                }

                poise::Command {
                    slash_action: Some(|ctx| Box::pin(async move {
                        inner(ctx.into()).await
                    })),
                    subcommands: vec![$([<get_ $name>]()),+],
                    name: "get".to_string(),
                    description: Some("Gets a specific configuration option".to_string()),
                    ..Default::default()
                }
            }

            $(
                #[doc = "Gets the " $title " configuration option"]
                #[command(slash_command, rename = $name_str, ephemeral)]
                async fn [<get_ $name>](ctx: Context<'_>) -> Result<(), Error> {
                    let Config { $name, .. } = read_or_write_default(ctx, &get_config_key(ctx)?).await?;

                    ctx.send(
                        CreateReply::default().embed(
                            CreateEmbed::new()
                                .title($title)
                                .description($desc)
                                .field("Current Value", $name.to_config_string(), false)
                                .timestamp(Timestamp::now())
                                .color(Color::BLUE),
                        ),
                    )
                    .await?;

                    Ok(())
                }
            )+

            fn set() -> poise::Command<Data, Error> {
                async fn inner(_ctx: Context<'_>) -> Result<(), FrameworkError<'_, Data, Error>> {
                    unreachable!();
                }

                poise::Command {
                    slash_action: Some(|ctx| Box::pin(async move {
                        inner(ctx.into()).await
                    })),
                    subcommands: vec![$([<set_ $name>]()),+],
                    name: "set".to_string(),
                    description: Some("Sets a specific configuration option".to_string()),
                    ..Default::default()
                }
            }

            $(

                #[doc = "Sets the " $title " configuration option"]
                #[command(slash_command, rename = $name_str, ephemeral)]
                async fn [<set_ $name>](
                    ctx: Context<'_>,
                    #[description = "The value to set " $title " to"] value: $type,
                ) -> Result<(), Error> {
                    let config_key = get_config_key(ctx)?;
                    let mut config: Config = read_or_write_default(ctx, &config_key).await?;

                    config.$name = value;
                    ctx.data().op.write_serialized(&config_key, &config).await?;
                    ctx.say(format!(
                        "**{}** has been set to **{}** {FLOOF_HAPPY}",
                        $title,
                        value.to_config_string(),
                    ))
                    .await?;

                    Ok(())
                }
            )+
        }
    };
}

config! {
    /// Whether to enable the strikes moderation system, `/strike`, and its subcommands
    let strikes_enabled: bool = ("strikes_enabled", "Strikes Enabled");

    /// Channel to log strike events in
    let strikes_log_channel: Option<ChannelId> = ("strikes_log_channel", "Strikes Log Channel");

    /// Whether to enable the `/anon` command, which allows members to send messages anonymously
    let anon_enabled: bool = ("anon_enabled", "Anon Enabled");

    /// Channel to restrict `/anon` to, if anon is enabled
    let anon_channel: Option<ChannelId> = ("anon_channel", "Anon Channel");

    /// Channel to log `/anon` uses to, if anon is enabled
    let anon_log_channel: Option<ChannelId> = ("anon_log_channel", "Anon Log Channel");
}
