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
use poise::{
    command,
    serenity_prelude::{
        Color, CreateAllowedMentions, CreateEmbed, CreateEmbedAuthor, CreateMessage, CreateWebhook,
        ExecuteWebhook, Mentionable, Timestamp,
    },
};

use crate::{
    config::{get_config_key, Config},
    emoji::*,
    error::UserError,
    persist::load_or_save_default,
    Context, Error,
};

/// Sends a message anonymously
#[command(
    slash_command,
    install_context = "Guild",
    interaction_context = "Guild",
    required_bot_permissions = "MANAGE_WEBHOOKS|SEND_MESSAGES|USE_EXTERNAL_EMOJIS",
    ephemeral
)]
pub(crate) async fn anon(
    ctx: Context<'_>,
    #[description = "Message to send anonymously"] message: String,
) -> Result<(), Error> {
    let Config {
        anon_enabled,
        anon_channel,
        anon_log_channel,
        ..
    } = load_or_save_default(ctx, &get_config_key(ctx)?)?;

    if !anon_enabled {
        bail!(UserError(anyhow!(
            "`/anon` is not enabled, the `anon_enabled` config option is `false`",
        )));
    }

    if let Some(anon_channel) = anon_channel {
        if anon_channel != ctx.channel_id() {
            bail!(UserError(anyhow!(
                "`/anon` is only allowed in {} due to the `anon_channel` config option being set",
                anon_channel.mention(),
            )));
        }
    }

    let channel = ctx.channel_id();
    let webhook = match channel
        .webhooks(ctx)
        .await
        .context("Could not get channel webhooks")?
        .into_iter()
        .find(|webhook| {
            webhook
                .application_id
                .map_or(false, |id| id.get() == ctx.framework().bot_id.get())
        }) {
        Some(webhook) => webhook,
        None => channel
            .create_webhook(
                ctx,
                CreateWebhook::new("Anonymous")
                    .audit_log_reason("`/anon` used in channel without existing Anonymous webhook"),
            )
            .await
            .context("Could not create webhook")?,
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
            .context("Failed to log anonymous message")?;
    }

    webhook
        .execute(ctx, false, ExecuteWebhook::new().content(message))
        .await
        .context("Failed to send message in anon log channel")?;

    ctx.say(format!("Message sent anonymously {FLOOF_HAPPY}"))
        .await?;

    Ok(())
}
