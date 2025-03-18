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
    CreateReply,
    serenity_prelude::{CreateActionRow, CreateButton, GuildId, RoleId, SkuId},
};

use crate::emoji::*;

const EARLY_ACCESS_SKU_ID: u64 = 1351234259867926671;

/// Returns `Ok(true)` or sends a reply and returns `Ok(false)`.
///
/// May return `Err(_)` if it fails to send a reply.
#[allow(unused)]
pub(super) async fn has_early_access(
    ctx: crate::Context<'_>,
) -> Result<bool, poise_error::anyhow::Error> {
    let author_id = ctx.author().id;
    let goober_bot_dev_guild = GuildId::new(1250948547403055114);
    let og_early_access_role = RoleId::new(1337229578472652846);

    if goober_bot_dev_guild
        .member(ctx, author_id)
        .await
        .is_ok_and(|member| member.roles.contains(&og_early_access_role))
    {
        return Ok(true);
    }

    let entitlements = ctx
        .http()
        .get_entitlements(
            Some(author_id),
            Some(vec![SkuId::new(EARLY_ACCESS_SKU_ID)]),
            None,
            None,
            None,
            None,
            Some(true),
        )
        .await?;

    if entitlements.is_empty() {
        ctx.send(
            CreateReply::default()
                .content(format!(
                    "Hark! This command is in **Early Access**- but you're not! You *could* be, though, if you would consider **supporting the developer**... {FLOOF_HEART}",
                ))
                .components(vec![CreateActionRow::Buttons(vec![CreateButton::new_premium(EARLY_ACCESS_SKU_ID)])])
                .ephemeral(true),
        )
        .await?;

        return Ok(false);
    }

    Ok(true)
}
