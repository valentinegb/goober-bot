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
