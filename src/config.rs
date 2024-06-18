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
use poise::serenity_prelude::ChannelId;
use serde::{Deserialize, Serialize};

use crate::{Context, Error};

#[derive(Deserialize, Serialize, Default)]
#[non_exhaustive]
#[serde(default)]
pub(crate) struct Config {
    pub(crate) strikes_enabled: bool,
    pub(crate) strikes_log_channel: Option<ChannelId>,
}

/// Gets the config key for the server in `ctx`.
pub(crate) fn get_config_key(ctx: Context<'_>) -> Result<String, Error> {
    Ok(format!(
        "config_{}",
        ctx.guild_id().context("Expected context to be in guild")?
    ))
}
