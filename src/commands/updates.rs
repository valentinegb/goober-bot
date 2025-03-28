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

use poise::{
    CreateReply, command,
    serenity_prelude::{Color, CreateEmbed},
};

use crate::Context;

/// Lists the 10 most recent Goober Bot changes
#[command(
    slash_command,
    category = "Other",
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel",
    ephemeral
)]
pub(crate) async fn updates(ctx: Context<'_>) -> Result<(), poise_error::anyhow::Error> {
    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title("Updates")
                .description(format!(
                    "{}\n. . .\n\nSee the [GitHub repository](https://github.com/valentinegb/goober-bot/commits/v1/) for more!",
                    crabtime::eval! {
                        #![dependency(git2 = "0.20.1")]
                        use git2::Repository;

                        let repo = Repository::open(crabtime::WORKSPACE_PATH).unwrap();
                        let mut revwalk = repo.revwalk().unwrap();
                        let mut string = String::new();

                        revwalk.push_head().unwrap();

                        for (i, oid) in revwalk.take(10).enumerate() {
                            let oid = oid.unwrap();
                            let commit = repo.find_commit(oid).unwrap();

                            if i == 0 {
                                string += &format!("The last change was <t:{}:R>.\n", commit.time().seconds());
                            }

                            string += &format!(
                                "\n[{}](https://github.com/valentinegb/goober-bot/commit/{}): {}",
                                &oid.to_string()[..7],
                                oid,
                                commit.message().unwrap().lines().next().unwrap(),
                            );
                        }

                        format!("{string:?}")
                    }
                ))
                .color(Color::BLUE),
        ),
    )
    .await?;

    Ok(())
}
