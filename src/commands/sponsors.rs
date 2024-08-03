use anyhow::Context as _;
use poise::command;

use crate::{
    emoji::*,
    sponsors::{self, Sponsor},
    Context, Error,
};

/// Lists current GitHub sponsors ❤️
#[command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel",
    required_bot_permissions = "USE_EXTERNAL_EMOJIS",
    ephemeral
)]
pub(crate) async fn sponsors(ctx: Context<'_>) -> Result<(), Error> {
    let sponsors = sponsors::get().await.context("Failed to get sponsors")?;
    let sponsors_page = "https://github.com/sponsors/valentinegb";

    if sponsors.is_empty() {
        ctx.say(format!("Woah, hey, new command! Hmm... I don't have any sponsors to show yet, but you could be the first!\nYour name could [be here]({sponsors_page}) for **$5/month**, a little goes a long way! {FLOOF_MUG}")).await?;

        return Ok(());
    }

    let mut message = format!(
        "This project is made possible by these absolutely *lovely* sponsors {FLOOF_HEART}\n",
    );

    for Sponsor { login, name } in sponsors {
        message += &format!("\n- {name} ([{login}](https://github.com/{login}))");
    }

    message += &format!("\n\nYour name could [be here too]({sponsors_page}) for **$5/month**, a little goes a long way! {FLOOF_MUG}");

    ctx.say(message).await?;

    Ok(())
}
