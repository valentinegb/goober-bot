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

use poise::serenity_prelude::{ChannelId, ExecuteWebhook, Mentionable, Webhook};

use crate::{Context, Error, FloofEmoji};

/// Sends an anonymous message in the #confessional channel
#[poise::command(slash_command)]
pub(super) async fn confess(
    ctx: Context<'_>,
    #[description = "Your message to #confessional"] message: String,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let webhook = Webhook::from_url(ctx, &ctx.data().confessions_webhook_url).await?;

    webhook
        .execute(
            ctx,
            false,
            ExecuteWebhook::new()
                .content(&message)
                .username("Anonymous"),
        )
        .await?;

    let log_channel = ChannelId::new(1228943791285866606);

    log_channel
        .say(
            ctx,
            format!("{} said: \"{message}\"", ctx.author().mention()),
        )
        .await?;

    ctx.say(format!(
        "Your confession has been sent! {}",
        FloofEmoji::FloofHappy,
    ))
    .await?;

    Ok(())
}
