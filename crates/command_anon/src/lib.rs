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

use commands_shared::LogChannelContextualizable;
use config::{Config, get_config_key};
use database::read_or_write_default;
use emoji::*;
use poise::{
    command,
    serenity_prelude::{
        Color, CreateAllowedMentions, CreateEmbed, CreateEmbedAuthor, CreateMessage, CreateWebhook,
        ExecuteWebhook, Timestamp,
    },
};
use poise_error::{
    UserError,
    anyhow::{Context as _, anyhow, bail},
};
use shared::Context;

/// Sends a message anonymously
#[command(
    slash_command,
    category = "Other",
    install_context = "Guild",
    interaction_context = "Guild",
    required_bot_permissions = "MANAGE_WEBHOOKS|SEND_MESSAGES|USE_EXTERNAL_EMOJIS",
    ephemeral
)]
pub async fn anon(
    ctx: Context<'_>,
    #[description = "Message to send anonymously"] message: String,
) -> Result<(), poise_error::anyhow::Error> {
    let Config {
        anon_enabled,
        anon_channel,
        anon_log_channel,
        ..
    } = read_or_write_default(ctx, &get_config_key(ctx)?).await?;

    if !anon_enabled {
        bail!(UserError(anyhow!(
            r#"/anon is not enabled, the "anon_enabled" config option is set to "false""#,
        )));
    }

    if let Some(anon_channel) = anon_channel {
        if anon_channel != ctx.channel_id() {
            bail!(UserError(anyhow!(
                r#"/anon is only allowed in #{} due to the "anon_channel" config option being set"#,
                anon_channel.name(ctx).await?,
            )));
        }
    }

    let channel = ctx.channel_id();
    let webhook = match channel
        .webhooks(ctx)
        .await
        .context("could not get channel webhooks")?
        .into_iter()
        .find(|webhook| {
            webhook
                .application_id
                .is_some_and(|id| id.get() == ctx.framework().bot_id.get())
        }) {
        Some(webhook) => webhook,
        None => channel
            .create_webhook(
                ctx,
                CreateWebhook::new("Anonymous")
                    .audit_log_reason("`/anon` used in channel without existing Anonymous webhook"),
            )
            .await
            .context("could not create webhook")?,
    };

    let author = ctx.author();

    if let Some(log_channel) = anon_log_channel {
        log_channel
            .send_message(
                ctx,
                CreateMessage::new()
                    .embed(
                        CreateEmbed::new()
                            .author(
                                CreateEmbedAuthor::new(&author.name).icon_url(
                                    author
                                        .avatar_url()
                                        .unwrap_or_else(|| author.default_avatar_url()),
                                ),
                            )
                            .title("Anonymous Message Sent")
                            .description(&message)
                            .timestamp(Timestamp::now())
                            .color(Color::BLURPLE),
                    )
                    .allowed_mentions(CreateAllowedMentions::new()),
            )
            .await
            .contextualize_log_channel_errors()?;
    }

    webhook
        .execute(ctx, false, ExecuteWebhook::new().content(message))
        .await
        .context("failed to send anon message")?;

    ctx.say(format!("Message sent anonymously {FLOOF_HAPPY}"))
        .await?;

    Ok(())
}
