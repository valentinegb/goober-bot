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

use poise::serenity_prelude::{
    Attachment, ChannelId, CreateAttachment, CreateMessage, ExecuteWebhook, Mentionable, UserId,
    Webhook,
};

use crate::{utility::choose_str, Context, Error, FloofEmoji};

/// The Collective grows ever stronger...
#[poise::command(slash_command, ephemeral)]
pub(super) async fn collective(
    ctx: Context<'_>,
    #[description = "Your contribution to The Collective"] message: String,
    #[description = "An additional offering to The Collective"] attachment: Option<Attachment>,
) -> Result<(), Error> {
    if ctx.author().id == UserId::new(672258199566417930) {
        ctx.say("Your flesh is now a part of The Collective, but your mind was broken...")
            .await?;

        return Ok(());
    }

    ctx.defer_ephemeral().await?;

    let webhook = Webhook::from_url(ctx, &ctx.data().collective_webhook_url).await?;
    let mut webhook_builder = ExecuteWebhook::new().content(&message);
    let log_channel = ChannelId::new(1229560575030464646);
    let mut message_builder =
        CreateMessage::new().content(format!("{} said: \"{message}\"", ctx.author().mention()));

    if let Some(attachment) = attachment {
        let attachment_builder = CreateAttachment::url(ctx, &attachment.url).await?;

        webhook_builder = webhook_builder.add_file(attachment_builder.clone());
        message_builder = message_builder.add_file(attachment_builder);
    }

    webhook.execute(ctx, false, webhook_builder).await?;
    log_channel.send_message(ctx, message_builder).await?;
    ctx.say(choose_str(&[
        format!(
            "Your work is invaluable to The Collective {}",
            FloofEmoji::Floof,
        ),
        format!("The Collective is appreciative {}", FloofEmoji::FloofHappy),
        format!(
            "The Collective grows ever stronger... thanks to you {}",
            FloofEmoji::FloofHappy,
        ),
        format!("The Collective loves you {}", FloofEmoji::FloofHeart),
        format!(
            "Your dedication to The Collective will be rewarded {}",
            FloofEmoji::FloofHappy,
        ),
        format!(
            "The Collective assures you that you are on the right side {}",
            FloofEmoji::Floof,
        ),
        format!("When The Collective's plan comes to fuition, you will be guaranteed a high position, among The Collective {}", FloofEmoji::Floof),
        format!("All that will remain is The Collective {}", FloofEmoji::Floof),
        format!("My favorite part of The Collective was when The Collective said \"it's collectin' time\" and collected all over the place {}", FloofEmoji::FloofBlep),
    ])?)
    .await?;

    Ok(())
}
