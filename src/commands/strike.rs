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
        ChannelId, Color, CreateAllowedMentions, CreateEmbed, CreateEmbedAuthor, CreateMessage,
        Mentionable, Timestamp, User, UserId,
    },
    CreateReply,
};
use serde::{Deserialize, Serialize};

use crate::{
    config::{get_config_key, Config},
    emoji::*,
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

impl Strike {
    fn to_string(&self, user: impl Mentionable, with_issuer: bool, with_issued: bool) -> String {
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
        let on = match with_issued {
            true => format!(" on <t:{}:d>", self.issued.unix_timestamp()),
            false => String::new(),
        };
        let ave = format!(
            "ave {} a strike{for_breaking_rule}{with_comment}{which_expires}{on}",
            user.mention(),
        );

        match with_issuer {
            true => format!("{} g{ave}", self.issuer.mention()),
            false => format!("G{ave}",),
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
        bail!("Strikes are not enabled, see `/config get strikes_enabled`");
    }

    Ok(strikes_log_channel)
}

/// Subcommands related to the strikes moderation system
#[command(slash_command, subcommands("give", "history"))]
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

    let allowed_mentions = CreateAllowedMentions::new();

    ctx.send(
        CreateReply::default()
            .content(strike.to_string(user, false, false))
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
            .await?;
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
        bail!(
            "You must have the View Audit Log permission to see the strike history of other users"
        );
    }

    let strikes_key = &get_strikes_key(ctx, user.id)?;
    let strikes: Strikes = load_or_save_default(ctx, strikes_key)?;
    let all = all.unwrap_or(false);
    let mut description = String::new();

    if strikes.is_empty() {
        description = format!("All clean! {FLOOF_INNOCENT}");
    }

    for (i, strike) in strikes.iter().enumerate() {
        if strike.is_expired() && !all {
            continue;
        }

        if i != 0 {
            description += "\n";
        }

        description += &format!("- {}", strike.to_string(user.clone(), true, true));
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
