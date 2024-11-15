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

use buy_me_a_coffee::MemberStatus;
use poise::{
    serenity_prelude::{self, UserId},
    CreateReply,
};

use crate::emoji::*;

/// Returns `Ok(true)` or sends a reply and returns `Ok(false)`.
///
/// May return `Err(_)` if it fails to send a reply.
#[allow(unused)]
pub(super) async fn has_early_access(
    ctx: crate::Context<'_>,
) -> Result<bool, serenity_prelude::Error> {
    let author = ctx.author();

    if ctx.framework().options().owners.contains(&author.id)
        || author.id == UserId::new(993768189924229171 /* queerzi */)
        || author.id == UserId::new(354060711732969473 /* woodmanvevo */)
    {
        return Ok(true);
    }

    if let Some(ref email) = author.email {
        let mut i = 1;

        while let Ok(page) = ctx
            .data()
            .buy_me_a_coffee_client
            .members(MemberStatus::Active, i)
            .await
        {
            for membership in page.data {
                if membership.payer_email != *email {
                    continue;
                }

                if membership.id != 218876 {
                    continue;
                }

                return Ok(true);
            }

            i += 1;
        }
    }

    ctx.send(
        CreateReply::default()
            .content(format!(
                "Hark! This command is in **Early Access**- but you're not! You *could* be, though, if you would consider [supporting the developer](https://buymeacoffee.com/im_valentinegb/membership)... {FLOOF_HEART}",
            ))
            .ephemeral(true),
    )
    .await?;

    Ok(false)
}
