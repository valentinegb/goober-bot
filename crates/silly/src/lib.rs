// Goober Bot, the Discord bot
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
//
// You may contact me via electronic mail at <valentinegb@icloud.com>.

use early_access::has_early_access;
use emoji::substitute_emojis;
use poise::{
    command,
    serenity_prelude::{Mentionable, UserId},
};
use poise_error::anyhow;
use rand::{rng, seq::IndexedRandom};

struct Responses<'a> {
    self_response: &'a str,
    bot_response: &'a str,
    other_responses: &'a [&'a str],
}

impl<'a> Responses<'a> {
    async fn respond(&self, ctx: poise_error::Context<'_>, target: UserId) -> anyhow::Result<()> {
        let author = ctx.author();
        let response = if target == author.id {
            self.self_response
        } else if target == ctx.framework().bot_id {
            self.bot_response
        } else {
            self.other_responses.choose(&mut rng()).unwrap()
        };

        ctx.reply(
            substitute_emojis(response)
                .replace("{author}", &author.to_string())
                .replace("{target}", &target.mention().to_string()),
        )
        .await?;

        Ok(())
    }
}

include!(concat!(env!("OUT_DIR"), "/sillies.rs"));
