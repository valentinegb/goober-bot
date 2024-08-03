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
use serde::Deserialize;

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
