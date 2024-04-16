use poise::serenity_prelude::Message;
use tracing::debug;

use crate::{Context, Error};

#[poise::command(context_menu_command = "Debug Embeds", owners_only, ephemeral)]
pub(super) async fn embeds(ctx: Context<'_>, message: Message) -> Result<(), Error> {
    debug!("{:#?}", message.embeds);
    ctx.say(format!("```rs\n{:#?}\n```", message.embeds))
        .await?;

    Ok(())
}
