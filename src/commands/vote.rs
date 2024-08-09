use anyhow::Context as _;
use poise::command;

use crate::{emoji::*, Context, Error};

/// Vote for Goober Bot on Top.gg!
#[command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel",
    ephemeral
)]
pub(crate) async fn vote(ctx: Context<'_>) -> Result<(), Error> {
    let has_voted = ctx
        .data()
        .topgg_client
        .has_voted(ctx.author().id)
        .await
        .context("Could not check if user has voted")
        .context("Top.gg dun goofed")?;
    let message = if has_voted {
        format!("You've already voted today, thank you so much! ily {FLOOF_HEART}")
    } else {
        format!(
            "You're able to vote for <@{bot_id}> on Top.gg today still! You can [do so here](https://top.gg/bot/{bot_id}/vote). Thank you for your consideration! {FLOOF_HAPPY}",
            bot_id = ctx.framework().bot_id,
        )
    };

    ctx.say(message).await?;

    Ok(())
}
