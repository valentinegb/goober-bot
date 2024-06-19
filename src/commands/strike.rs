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
use chrono::Months;
use poise::{
    command,
    serenity_prelude::{
        ChannelId, Color, CreateAllowedMentions, CreateEmbed, CreateMessage, Mentionable,
        Timestamp, UserId,
    },
    CreateReply,
};
use serde::{Deserialize, Serialize};

use crate::{
    config::{get_config_key, Config},
    persist::load_or_save_default,
    Context, Error,
};

type Strikes = Vec<Strike>;

#[derive(Deserialize, Serialize, Clone)]
#[non_exhaustive]
struct Strike {
    issuer: UserId,
    issued: Timestamp,
    #[serde(default)]
    rule: Option<u8>,
    #[serde(default)]
    comment: Option<String>,
    #[serde(default)]
    expiration: Option<Timestamp>,
}

/// Gets the strikes key for `user` for the server in `ctx`.
pub(crate) fn get_strikes_key(ctx: Context<'_>, user: UserId) -> Result<String, Error> {
    Ok(format!(
        "strikes_{}_{user}",
        ctx.guild_id().context("Expected context to be in guild")?
    ))
}

/// Returns an error if strikes are not enabled, otherwise returns
/// `strikes_log_channel`, which may be [`None`].
fn pre_strike_command(ctx: Context<'_>) -> Result<Option<ChannelId>, Error> {
    let Config {
        strikes_enabled,
        strikes_log_channel,
        ..
    } = load_or_save_default(ctx, &get_config_key(ctx)?)?;

    if !strikes_enabled {
        bail!("Strikes are not enabled, see `/config get strikes_enabled`");
    }

    Ok(strikes_log_channel)
}

/// Subcommands related to the strikes moderation system
#[command(slash_command, subcommands("give"))]
pub(crate) async fn strike(_ctx: Context<'_>) -> Result<(), Error> {
    unreachable!()
}

/// Give a strike to a server member
#[command(
    slash_command,
    required_permissions = "KICK_MEMBERS|BAN_MEMBERS|MODERATE_MEMBERS",
    required_bot_permissions = "KICK_MEMBERS|BAN_MEMBERS|MODERATE_MEMBERS",
    ephemeral
)]
async fn give(
    ctx: Context<'_>,
    #[description = "User to give strike to"] user: UserId,
    #[description = "Infracted rule that strike is being given in response to"] rule: Option<u8>,
    #[description = "Any comment on the strike, such as explanation of a specific circumstance"]
    comment: Option<String>,
    #[description = "When the strike should expire, in months. If not specified, strike will never expire"]
    expiration: Option<u32>,
) -> Result<(), Error> {
    let log_channel = pre_strike_command(ctx)?;

    let strikes_key = &get_strikes_key(ctx, user)?;
    let mut strikes: Strikes = load_or_save_default(ctx, strikes_key)?;
    let strike = Strike {
        issuer: ctx.author().id,
        issued: Timestamp::now(),
        rule,
        comment,
        expiration: match expiration {
            Some(expiration) => Some(
                Timestamp::now()
                    .checked_add_months(Months::new(expiration))
                    .context("Failed to create timestamp from months")?
                    .into(),
            ),
            None => None,
        },
    };

    strikes.push(strike.clone());
    ctx.data().persist.save(strikes_key, strikes)?;

    let for_breaking_rule = match strike.rule {
        Some(rule) => format!(" for breaking rule {rule}"),
        None => String::new(),
    };
    let with_comment = match strike.comment {
        Some(comment) => format!(" with comment \"{comment}\""),
        None => String::new(),
    };
    let which_expires = match strike.expiration {
        Some(expiration) => format!(" which expires <t:{}:d>", expiration.unix_timestamp()),
        None => String::new(),
    };
    let message = format!(
        "ave {} a strike{for_breaking_rule}{with_comment}{which_expires}",
        user.mention(),
    );
    let allowed_mentions = CreateAllowedMentions::new();

    ctx.send(
        CreateReply::default()
            .content(format!("G{message}"))
            .allowed_mentions(allowed_mentions.clone()),
    )
    .await?;

    if let Some(log_channel) = log_channel {
        log_channel
            .send_message(
                ctx,
                CreateMessage::new()
                    .embed(
                        CreateEmbed::new()
                            .title("Strike Given")
                            .description(format!("{} g{message}", strike.issuer.mention()))
                            .timestamp(strike.issued)
                            .color(Color::RED),
                    )
                    .allowed_mentions(allowed_mentions),
            )
            .await?;
    }

    Ok(())
}
