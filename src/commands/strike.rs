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

use anyhow::{anyhow, bail, Context as _};
use chrono::Months;
use poise::{
    command,
    serenity_prelude::{
        ChannelId, Color, CreateAllowedMentions, CreateEmbed, CreateEmbedAuthor, CreateMessage,
        Mentionable, Timestamp, User, UserId,
    },
    CreateReply,
};
use serde::{Deserialize, Serialize};

use crate::{
    config::{get_config_key, Config},
    emoji::*,
    error::UserError,
    persist::load_or_save_default,
    Context, Error,
};

const SEND_STRIKE_LOG_CHANNEL_MESSAGE_ERROR: &str = "Failed to send message in strike log channel";

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
    #[serde(default)]
    repealer: Option<UserId>,
}

impl Strike {
    fn to_string(&self, user: impl Mentionable, with_issuer: bool, with_issued: bool) -> String {
        let on = match with_issued {
            true => format!(" on <t:{}:d>", self.issued.unix_timestamp()),
            false => String::new(),
        };
        let for_breaking_rule = match self.rule {
            Some(rule) => format!(" for breaking **rule {rule}**"),
            None => String::new(),
        };
        let with_comment = match self.comment {
            Some(ref comment) => format!(" with comment **\"{comment}\"**"),
            None => String::new(),
        };
        let which_expires = match self.expiration {
            Some(expiration) => format!(" which expires <t:{}:R>", expiration.unix_timestamp()),
            None => String::new(),
        };
        let ave = format!(
            "ave {} a strike{on}{for_breaking_rule}{with_comment}{which_expires}",
            user.mention(),
        );
        let message = match with_issuer {
            true => format!("{} g{ave}", self.issuer.mention()),
            false => format!("G{ave}",),
        };

        match self.repealer {
            Some(repealer) => format!("~~{message}~~ **repealed** by {}", repealer.mention()),
            None => message,
        }
    }

    fn is_expired(&self) -> bool {
        self.expiration
            .map_or(false, |expiration| expiration <= Timestamp::now())
    }
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
        bail!(UserError(anyhow!(
            "Strikes are not enabled, see `/config get strikes_enabled`",
        )));
    }

    Ok(strikes_log_channel)
}

/// Subcommands related to the strikes moderation system
#[command(
    slash_command,
    subcommands("give", "history", "repeal"),
    install_context = "Guild",
    interaction_context = "Guild",
    required_bot_permissions = "USE_EXTERNAL_EMOJIS"
)]
pub(crate) async fn strike(_ctx: Context<'_>) -> Result<(), Error> {
    unreachable!()
}

/// Give a strike to a server member
#[command(
    slash_command,
    required_permissions = "MODERATE_MEMBERS",
    required_bot_permissions = "SEND_MESSAGES",
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
        repealer: None,
    };

    strikes.push(strike.clone());
    ctx.data().persist.save(strikes_key, strikes)?;

    let allowed_mentions = CreateAllowedMentions::new();

    ctx.send(
        CreateReply::default()
            .content(format!(
                "{} {FLOOF_SAD}",
                strike.to_string(user, false, false)
            ))
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
                            .description(strike.to_string(user, true, false))
                            .timestamp(strike.issued)
                            .color(Color::RED),
                    )
                    .allowed_mentions(allowed_mentions),
            )
            .await
            .context(SEND_STRIKE_LOG_CHANNEL_MESSAGE_ERROR)?;
    }

    Ok(())
}

/// Get the strike history of a user (yourself by default)
#[command(slash_command, ephemeral)]
async fn history(
    ctx: Context<'_>,
    #[description = "User to get the strike history of"] user: Option<User>,
    #[description = "Show even expired strikes"] all: Option<bool>,
) -> Result<(), Error> {
    pre_strike_command(ctx)?;

    let user = user.as_ref().unwrap_or(ctx.author());

    if user.id != ctx.author().id
        && !ctx
            .author_member()
            .await
            .context("Expected author to be member")?
            .permissions
            .map_or(false, |permissions| permissions.view_audit_log())
    {
        bail!(UserError(anyhow!(
            "You must have the View Audit Log permission to see the strike history of other users",
        )));
    }

    let strikes_key = &get_strikes_key(ctx, user.id)?;
    let strikes: Strikes = load_or_save_default(ctx, strikes_key)?;
    let all = all.unwrap_or(false);
    let mut description = String::new();

    if strikes.is_empty() {
        description = format!("All clean! {FLOOF_INNOCENT}");
    }

    for (i, strike) in strikes.iter().enumerate() {
        if strike.is_expired() || strike.repealer.is_some() && !all {
            continue;
        }

        if i != 0 {
            description += "\n";
        }

        description += &format!(
            "- #{}: {}",
            i + 1,
            strike.to_string(user.clone(), true, true)
        );
    }

    ctx.send(
        CreateReply::default()
            .embed(
                CreateEmbed::new()
                    .author(
                        CreateEmbedAuthor::new(&user.name).icon_url(
                            user.avatar_url()
                                .unwrap_or_else(|| user.default_avatar_url()),
                        ),
                    )
                    .title("Strike History")
                    .description(description)
                    .timestamp(Timestamp::now())
                    .color(if strikes.is_empty() {
                        Color::FOOYOO
                    } else {
                        Color::RED
                    }),
            )
            .allowed_mentions(CreateAllowedMentions::new()),
    )
    .await?;

    Ok(())
}

/// Repeal a strike that was previously given
#[command(
    slash_command,
    required_permissions = "MODERATE_MEMBERS",
    required_bot_permissions = "SEND_MESSAGES",
    ephemeral
)]
async fn repeal(
    ctx: Context<'_>,
    #[description = "User to repeal a strike from"] user: UserId,
    #[description = "Strike to repeal (most recent by default)"]
    #[rename = "strike"]
    strike_i: Option<usize>,
) -> Result<(), Error> {
    if user == ctx.author().id {
        bail!(UserError(anyhow!(
            "You cannot repeal one of your own strikes",
        )));
    }

    let log_channel = pre_strike_command(ctx)?;
    let strikes_key = &get_strikes_key(ctx, user)?;
    let mut strikes: Strikes = load_or_save_default(ctx, strikes_key)?;
    let strike_i = strike_i.unwrap_or(strikes.len());
    let repealer = &mut strikes
        .get_mut(strike_i - 1)
        .context(UserError(anyhow!(
            "User does not have a strike #{strike_i}",
        )))?
        .repealer;

    if repealer.is_some() {
        bail!(UserError(anyhow!(
            "{}'s strike #{strike_i} has already been repealed",
            user.mention(),
        )));
    }

    *repealer = Some(ctx.author().id);
    ctx.data().persist.save(strikes_key, strikes)?;

    let allowed_mentions = CreateAllowedMentions::new();

    ctx.send(
        CreateReply::default()
            .content(format!(
                "{}'s strike #{strike_i} has been repealed {FLOOF_HAPPY}",
                user.mention(),
            ))
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
                            .title("Strike Repealed")
                            .description(format!(
                                "{}'s strike #{strike_i} was repealed by {}",
                                user.mention(),
                                ctx.author().mention(),
                            ))
                            .timestamp(Timestamp::now())
                            .color(Color::FOOYOO),
                    )
                    .allowed_mentions(allowed_mentions),
            )
            .await
            .context(SEND_STRIKE_LOG_CHANNEL_MESSAGE_ERROR)?;
    }

    Ok(())
}
