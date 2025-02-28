// Goober Bot, Discord bot
// Copyright (C) 2025  Valentine Briese
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

use chrono::{Months, Utc};
use poise::{
    CreateReply, command,
    serenity_prelude::{
        ChannelId, Color, CreateAllowedMentions, CreateEmbed, CreateEmbedAuthor, CreateMessage,
        Mentionable, Timestamp, User, UserId,
    },
};
use poise_error::{
    UserError,
    anyhow::{Context as _, anyhow, bail},
};

use crate::{
    Context,
    emoji::*,
    models::{Config, NewStrike, Strike},
};

const SEND_STRIKE_LOG_CHANNEL_MESSAGE_ERROR: &str = "failed to send message in strike log channel";

/// Returns an error if strikes are not enabled, otherwise returns
/// `strikes_log_channel`, which may be [`None`].
async fn pre_strike_command(
    ctx: Context<'_>,
) -> Result<Option<ChannelId>, poise_error::anyhow::Error> {
    let Config {
        strikes_enabled,
        strikes_log_channel,
        ..
    } = {
        use diesel::QueryDsl as _;
        use diesel_async::RunQueryDsl as _;

        use crate::schema::configs::dsl::*;

        let mut conn = ctx.data().pool.get().await?;

        configs
            .find(ctx.guild_id().unwrap())
            .first(&mut conn)
            .await?
    };

    if !strikes_enabled {
        bail!(UserError(anyhow!(
            r#"strikes are not enabled, see "/config get strikes_enabled""#,
        )));
    }

    Ok(strikes_log_channel)
}

/// Subcommands related to the strikes moderation system
#[command(
    slash_command,
    subcommands("give", "history", "repeal"),
    category = "Strikes",
    install_context = "Guild",
    interaction_context = "Guild",
    required_bot_permissions = "USE_EXTERNAL_EMOJIS"
)]
pub(crate) async fn strike(_ctx: Context<'_>) -> Result<(), poise_error::anyhow::Error> {
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
    #[description = "Infracted rule that strike is being given in response to"]
    #[max_length = 7]
    rule: Option<String>,
    #[description = "Any comment on the strike, such as explanation of a specific circumstance"]
    comment: Option<String>,
    #[description = "When the strike should expire, in months. If not specified, strike will never expire"]
    expiration: Option<u32>,
) -> Result<(), poise_error::anyhow::Error> {
    let log_channel = pre_strike_command(ctx).await?;
    let strike = NewStrike {
        guild: ctx.guild_id().unwrap(),
        user,
        issuer: ctx.author().id,
        issued: None,
        rule,
        comment,
        expiration: match expiration {
            Some(expiration) => Some(
                Utc::now()
                    .checked_add_months(Months::new(expiration))
                    .context("failed to create timestamp from months")?
                    .naive_utc(),
            ),
            None => None,
        },
        repealer: None,
    };
    let strike: Strike = {
        use diesel_async::RunQueryDsl as _;

        use crate::schema::strikes::dsl::*;

        let mut conn = ctx.data().pool.get().await?;

        diesel::insert_into(strikes)
            .values(strike)
            .get_result(&mut conn)
            .await?
    };
    let allowed_mentions = CreateAllowedMentions::new();

    ctx.send(
        CreateReply::default()
            .content(format!("{} {FLOOF_SAD}", strike.to_string(false, false)))
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
                            .description(strike.to_string(true, false))
                            .timestamp(strike.issued.and_utc())
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
) -> Result<(), poise_error::anyhow::Error> {
    pre_strike_command(ctx).await?;

    let user = user.as_ref().unwrap_or(ctx.author());

    if user.id != ctx.author().id
        && !ctx
            .author_member()
            .await
            .context("expected author to be member")?
            .permissions
            .is_some_and(|permissions| permissions.view_audit_log())
    {
        bail!(UserError(anyhow!(
            "you must have the View Audit Log permission to see the strike history of other users",
        )));
    }

    let strikes: Vec<Strike> = {
        use diesel::{ExpressionMethods as _, QueryDsl as _};
        use diesel_async::RunQueryDsl as _;

        let mut conn = ctx.data().pool.get().await?;

        crate::schema::strikes::table
            .filter(crate::schema::strikes::columns::guild.eq(ctx.guild_id().unwrap()))
            .filter(crate::schema::strikes::columns::user.eq(user.id))
            .load(&mut conn)
            .await?
    };
    let all = all.unwrap_or(false);
    let mut description = String::new();
    let mut clean = true;

    for (i, strike) in strikes.iter().enumerate() {
        if strike.is_expired() || strike.repealer.is_some() && !all {
            continue;
        }

        clean = false;

        if i != 0 {
            description += "\n";
        }

        description += &format!("- #{}: {}", i + 1, strike.to_string(true, true));
    }

    if clean {
        description = format!("All clean! {FLOOF_INNOCENT}");
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
                    .color(if clean { Color::FOOYOO } else { Color::RED }),
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
) -> Result<(), poise_error::anyhow::Error> {
    if user == ctx.author().id {
        bail!(UserError(anyhow!(
            "you cannot repeal one of your own strikes",
        )));
    }

    let log_channel = pre_strike_command(ctx).await?;
    let strikes: Vec<(i32, Option<UserId>)> = {
        use diesel::{ExpressionMethods as _, QueryDsl as _};
        use diesel_async::RunQueryDsl as _;

        let mut conn = ctx.data().pool.get().await?;

        crate::schema::strikes::table
            .filter(crate::schema::strikes::columns::guild.eq(ctx.guild_id().unwrap()))
            .filter(crate::schema::strikes::columns::user.eq(user))
            .select((
                crate::schema::strikes::columns::id,
                crate::schema::strikes::columns::repealer,
            ))
            .load(&mut conn)
            .await?
    };
    let strike_i = strike_i.unwrap_or(strikes.len());
    let (id, repealer) = strikes.get(strike_i - 1).context(UserError(anyhow!(
        "user does not have a strike #{strike_i}",
    )))?;

    if repealer.is_some() {
        bail!(UserError(anyhow!(
            "{}'s strike #{strike_i} has already been repealed",
            user.to_user(ctx).await?.name,
        )));
    }

    {
        use diesel::{ExpressionMethods as _, QueryDsl as _};
        use diesel_async::RunQueryDsl as _;

        let mut conn = ctx.data().pool.get().await?;

        diesel::update(
            crate::schema::strikes::table.filter(crate::schema::strikes::columns::id.eq(id)),
        )
        .set(crate::schema::strikes::columns::repealer.eq(Some(ctx.author().id)))
        .execute(&mut conn)
        .await?;
    }

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
