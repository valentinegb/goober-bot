// Goober Bot, bot that is also a goober for the Gooberland Discord server
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
use chrono::Months;
use poise::{
    serenity_prelude::{
        Color, CreateEmbed, CreateEmbedAuthor, FormattedTimestamp, FormattedTimestampStyle,
        Timestamp, UserId,
    },
    CreateReply,
};
use serde::{Deserialize, Serialize};

use crate::{Context, Error};

#[derive(Debug, Deserialize, Serialize)]
struct Strike {
    issuer_id: u64,
    issuing: Timestamp,
    expiration: Timestamp,
    rule: Option<u8>,
    comment: Option<String>,
}

#[repr(u8)]
enum Rule {
    Nsfw = 1,
    Hate = 2,
    Discourse = 3,
    Conflict = 4,
}

impl Rule {
    fn get_expiration(&self) -> Months {
        match self {
            Rule::Nsfw => Months::new(4),
            Rule::Hate => Months::new(5),
            Rule::Discourse => Months::new(2),
            Rule::Conflict => Months::new(3),
        }
    }
}

impl TryFrom<u8> for Rule {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Nsfw),
            2 => Ok(Self::Hate),
            3 => Ok(Self::Discourse),
            4 => Ok(Self::Conflict),
            other => Err(format!("There is no rule {other}")),
        }
    }
}

#[poise::command(slash_command, subcommands("add", "history", "remove", "total"))]
pub(super) async fn strike(_ctx: Context<'_>) -> Result<(), Error> {
    unreachable!();
}

/// Add a strike to a user's record
#[poise::command(slash_command, required_permissions = "BAN_MEMBERS", ephemeral)]
async fn add(
    ctx: Context<'_>,
    #[description = "User to give a strike to"] user: UserId,
    #[description = "Rule in violation"] rule: Option<u8>,
    #[description = "Any additional comments"] comment: Option<String>,
    #[description = "Override the expiration time (in months)"] expiration: Option<u32>,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let mut users_strikes: Vec<Strike> = ctx
        .data()
        .persist
        .load(&format!("strikes_{user}"))
        .unwrap_or_default();
    let expiration_months = match expiration {
        Some(expiration) => Months::new(expiration),
        None => match rule {
            Some(rule) => Rule::try_from(rule)?.get_expiration(),
            None => Months::new(1),
        },
    };

    users_strikes.push(Strike {
        issuer_id: ctx.author().id.get(),
        issuing: Timestamp::now(),
        expiration: Timestamp::now()
            .checked_add_months(expiration_months)
            .ok_or(anyhow!("Could not add months to current timestamp"))?
            .into(),
        rule,
        comment,
    });

    let users_active_strikes: Vec<&Strike> = users_strikes
        .iter()
        .filter(|strike| strike.expiration > Timestamp::now())
        .collect();
    let action_suggestion = match users_active_strikes.len() {
        1 => Some("give them a warning".to_string()),
        2 => Some("put them on timeout for an hour".to_string()),
        3 => Some("put them on timeout for a day".to_string()),
        4 => Some("ban them for a week".to_string()),
        5 => Some("ban them permanently".to_string()),
        _ => None,
    };
    let action_suggestion_message = match action_suggestion {
        Some(suggestion) => format!(", you should **{suggestion}**"),
        None => "".to_string(),
    };
    let message = format!(
        "<@{user}> has been given a strike. This is strike {}{}.",
        users_active_strikes.len(),
        action_suggestion_message,
    );

    ctx.data()
        .persist
        .save(&format!("strikes_{user}"), users_strikes)?;
    ctx.say(message).await?;

    Ok(())
}

/// View a user's strike history
#[poise::command(slash_command, ephemeral)]
async fn history(
    ctx: Context<'_>,
    #[description = "User to view strike history of"] user: Option<UserId>,
    #[description = "Whether to show expired strikes, too"] all: Option<bool>,
) -> Result<(), Error> {
    let user_id = user.unwrap_or(ctx.author().id);

    if !ctx
        .author_member()
        .await
        .ok_or(anyhow!("Expected command caller to be a member"))?
        .permissions
        .ok_or(anyhow!("Expected command caller to have permissions"))?
        .ban_members()
        && user_id != ctx.author().id
    {
        ctx.say(
            "You must have the `BAN_MEMBERS` permission to view other members' strike history.",
        )
        .await?;
    }

    ctx.defer_ephemeral().await?;

    let user = user_id.to_user(ctx).await?;
    let all = all.unwrap_or_default();
    let users_strikes: Vec<Strike> = ctx
        .data()
        .persist
        .load(&format!("strikes_{user_id}"))
        .unwrap_or_default();
    let mut list = String::new();

    for (i, strike) in users_strikes.iter().enumerate() {
        let expired = strike.expiration <= Timestamp::now();

        if !all && expired {
            continue;
        }

        list += &format!(
            "- **Strike {}**: issued {} by <@{}>",
            i + 1,
            FormattedTimestamp::new(strike.issuing, Some(FormattedTimestampStyle::ShortDate)),
            strike.issuer_id,
        );

        if let Some(rule) = strike.rule {
            list += &format!(" for breaking rule {rule}");
        }

        if let Some(comment) = &strike.comment {
            list += &format!(" with the comment \"{comment}\"");
        }

        list += ". ";
        list += match expired {
            true => "Expired ",
            false => "Expires ",
        };
        list += &FormattedTimestamp::new(
            strike.expiration,
            Some(FormattedTimestampStyle::RelativeTime),
        )
        .to_string();
        list += ".\n";
    }

    if list.is_empty() {
        list = "All clean!".to_string();
    }

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .author(
                    CreateEmbedAuthor::new(&user.name)
                        .icon_url(user.avatar_url().unwrap_or(user.default_avatar_url())),
                )
                .title("Strike History")
                .description(list)
                .timestamp(Timestamp::now())
                .color(Color::RED),
        ),
    )
    .await?;

    Ok(())
}

/// Remove the most recent strike from a user's record
#[poise::command(slash_command, required_permissions = "BAN_MEMBERS", ephemeral)]
async fn remove(
    ctx: Context<'_>,
    #[description = "User to remove the strike from"] user: UserId,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let mut users_strikes: Vec<Strike> = ctx
        .data()
        .persist
        .load(&format!("strikes_{user}"))
        .unwrap_or_default();

    users_strikes.pop();

    let users_active_strikes: Vec<&Strike> = users_strikes
        .iter()
        .filter(|strike| strike.expiration > Timestamp::now())
        .collect();
    let message = format!(
        "<@{user}>'s last strike has been removed, they now have {} active strike(s).",
        users_active_strikes.len()
    );

    ctx.data()
        .persist
        .save(&format!("strikes_{user}"), users_strikes)?;
    ctx.say(message).await?;

    Ok(())
}

/// View a user's strike total
#[poise::command(slash_command, ephemeral)]
async fn total(
    ctx: Context<'_>,
    #[description = "User to view total strikes of"] user: Option<UserId>,
) -> Result<(), Error> {
    let user = user.unwrap_or(ctx.author().id);

    if !ctx
        .author_member()
        .await
        .ok_or(anyhow!("Expected command caller to be a member"))?
        .permissions
        .ok_or(anyhow!("Expected command caller to have permissions"))?
        .ban_members()
        && user != ctx.author().id
    {
        ctx.say("You must have the `BAN_MEMBERS` permission to view other members' strike total.")
            .await?;
    }

    //   /\
    //  /  \
    // / __ \
    // | 🦐 |
    // ------

    ctx.defer_ephemeral().await?;

    let users_strikes: Vec<Strike> = ctx
        .data()
        .persist
        .load(&format!("strikes_{user}"))
        .unwrap_or_default();
    let users_active_strikes: Vec<&Strike> = users_strikes
        .iter()
        .filter(|strike| strike.expiration > Timestamp::now())
        .collect();
    let users_expired_strikes: Vec<&Strike> = users_strikes
        .iter()
        .filter(|strike| strike.expiration <= Timestamp::now())
        .collect();

    ctx.say(format!(
        "<@{user}> has {} active strike(s) and {} expired strike(s), totaling at {} strike(s).",
        users_active_strikes.len(),
        users_expired_strikes.len(),
        users_strikes.len(),
    ))
    .await?;

    Ok(())
}
