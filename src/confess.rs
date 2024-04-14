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
