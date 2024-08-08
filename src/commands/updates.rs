use commit_history::commit_history;
use poise::{
    command,
    serenity_prelude::{Color, CreateEmbed},
    CreateReply,
};

use crate::{Context, Error};

/// Lists the 10 most recent Goober Bot changes
#[command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel",
    ephemeral
)]
pub(crate) async fn updates(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title("Updates")
                .description(format!("{}\n. . .\n\nSee the [GitHub repository](https://github.com/valentinegb/goober-bot/commits/v1/) for more!", commit_history!()))
                .color(Color::BLUE),
        ),
    )
    .await?;

    Ok(())
}
