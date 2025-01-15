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

use std::time::Duration;

use poise::serenity_prelude::{self, ActivityData};
use rand::{seq::SliceRandom, thread_rng};
use tokio::task::spawn_blocking;
use tracing::info;

const SLEEP_SECS: u64 = 10 * 60;

pub(super) fn start_activity_loop(ctx: serenity_prelude::Context) {
    spawn_blocking(move || {
        let activities = [
            ActivityData::custom("Testing random activities"),
            ActivityData::playing("Undertale"),
            ActivityData::watching("Markiplier"),
            ActivityData::listening("Daft Punk"),
            ActivityData::playing("ULTRAKILL"),
            ActivityData::custom("Configuring servers"),
            ActivityData::competing("Silliness Competition"),
            ActivityData::custom("Doing your mom"),
            ActivityData::custom("Goobing"),
            ActivityData::playing("with a rhombicosidodecahedron"),
            ActivityData::custom("Reading The Rust Book"),
            ActivityData::watching("cat videos"),
            ActivityData::watching("Gravity Falls"),
            ActivityData::watching("Tessa"),
        ];
        let mut rng = thread_rng();
        let mut last_activity = None;

        loop {
            let chosen_activity = activities
                .choose(&mut rng)
                .expect("`activities` should not be empty");

            // FIXME: This is ridiculous, too much for so little.
            //        Fix this after the PR for Serenity is merged.
            if let Some(last_activity) = last_activity {
                if serde_json::to_string(chosen_activity)
                    .expect("activities should not fail to serialize")
                    == serde_json::to_string(last_activity)
                        .expect("activities should not fail to serialize")
                {
                    continue;
                }
            }

            ctx.set_activity(Some(chosen_activity.clone()));

            last_activity = Some(chosen_activity);

            info!(?chosen_activity, "Set activity");
            std::thread::sleep(Duration::from_secs(SLEEP_SECS));
        }
    });
}
