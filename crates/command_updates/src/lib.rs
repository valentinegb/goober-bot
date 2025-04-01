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

use command_updates_proc_macro::commits_string;
use poise::{
    CreateReply, command,
    serenity_prelude::{Color, CreateEmbed},
};
use shared::Context;

/// Lists the 10 most recent Goober Bot changes
#[command(
    slash_command,
    category = "Other",
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel",
    ephemeral
)]
pub async fn updates(ctx: Context<'_>) -> Result<(), poise_error::anyhow::Error> {
    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title("Updates")
                .description(format!(
                    "{}\n. . .\n\nSee the [GitHub repository](https://github.com/valentinegb/goober-bot/commits/v1/) for more!",
                    commits_string!(),
                ))
                .color(Color::BLUE),
        ),
    )
    .await?;

    Ok(())
}
