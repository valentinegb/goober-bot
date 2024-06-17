use std::time::Duration;

use poise::serenity_prelude::{self, ActivityData};
use rand::{seq::SliceRandom, thread_rng};
use tokio::task::spawn_blocking;

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
            ActivityData::streaming(
                "Goober Bot Coding",
                "https://github.com/valentinegb/goober-bot",
            )
            .expect("URL should be valid"),
            ActivityData::custom("Doing your mom"),
            ActivityData::custom("Goobing"),
        ];
        let mut rng = thread_rng();

        loop {
            ctx.set_activity(Some(
                activities
                    .choose(&mut rng)
                    .expect("`activities` should not be empty")
                    .clone(),
            ));

            std::thread::sleep(Duration::from_secs(10 * 60));
        }
    });
}
