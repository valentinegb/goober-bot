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

use anyhow::Result;
use poise::{serenity_prelude::UserId, CreateReply};
use serde::Deserialize;

use crate::{emoji::*, Context};

#[derive(Deserialize)]
struct Response {
    data: Data,
}

#[derive(Deserialize)]
struct Data {
    viewer: Viewer,
}

#[derive(Deserialize)]
struct Viewer {
    sponsors: Sponsors,
}

#[derive(Deserialize)]
struct Sponsors {
    nodes: Vec<Sponsor>,
}

#[derive(Deserialize)]
pub(super) struct Sponsor {
    pub(super) login: String,
    pub(super) name: String,
}

// TODO: make this work with more than 100 sponsors
pub(super) async fn get() -> Result<Vec<Sponsor>> {
    let tier = "ST_kwDOAiT5_84ABlqV";
    let response: Response = octocrab::instance()
        .graphql(
            &serde_json::json!({
                "query": format!("{{ viewer {{ sponsors(first: 100, tierId: \"{tier}\") {{ nodes {{ ... on User {{ login name }} ... on Organization {{ login name }} }} }} }} }}"),
            }),
        )
        .await?;

    Ok(response.data.viewer.sponsors.nodes)
}

pub(super) async fn has_early_access(ctx: Context<'_>) -> Result<bool> {
    if ctx.author().id == UserId::new(1016154932354744330 /* valentinegb */)
        || ctx.author().id == UserId::new(993768189924229171 /* queerzi */)
    {
        Ok(true)
    } else {
        ctx.send(
            CreateReply::default()
                .content(format!(
                    "This command is in **early access**, only users with early access can use it. Currently, that's... one person, because they did a favor for me and asked to be part of early access before I've finished implementing it {FLOOF_MUG}\n\nDon't worry, all early access commands fall out of early access eventually! But if you are interested in joining early access yourself, stay tuned! {FLOOF_HAPPY}",
                ))
                .ephemeral(true),
        )
        .await?;

        Ok(false)
    }
}
