// Goober Bot, bot that is also a goober for the Gooberland Discord server
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

use std::{
    sync::{
        atomic::{self, AtomicBool, AtomicU64},
        Arc,
    },
    time::Duration,
};

use poise::serenity_prelude::{self, prelude::TypeMapKey, ChannelId, Message};
use tracing::{debug, error, info, warn};

use crate::{utility::choose_str, FloofEmoji};

pub(crate) struct BoredomTracker;

impl TypeMapKey for BoredomTracker {
    type Value = Arc<AtomicBool>;
}

pub(crate) struct BoredomMessage;

impl TypeMapKey for BoredomMessage {
    type Value = Arc<AtomicU64>;
}

pub(crate) async fn check_for_boredom_acknowledgment(
    ctx: &serenity_prelude::Context,
    new_message: &Message,
) -> Result<(), super::Error> {
    if let Some(referenced_message) = &new_message.referenced_message {
        let mut write_data = false;

        // Read data
        {
            let data = ctx.data.read().await;

            match data.get::<BoredomMessage>() {
                Some(boredom_message) => {
                    if referenced_message.id.get() == boredom_message.load(atomic::Ordering::SeqCst)
                    {
                        let picked_message = choose_str(&[
                            format!("Omg you're alive!!! {}", FloofEmoji::FloofBlep),
                            format!(
                                "\\*gasp\\* contact has been established! {}",
                                FloofEmoji::FloofOwo
                            ),
                            format!("Oh, phew, you're not dead! {}", FloofEmoji::FloofTired),
                            format!("Yaaaaay friends!!! {}", FloofEmoji::FloofBlep),
                        ])?;

                        new_message.reply_ping(ctx, &picked_message).await?;
                        info!("Replyed to boredom acknowledgment: {picked_message:?}");
                        write_data = true;
                    }
                }
                None => warn!("There is no BoredomMessage, but this is probably okay"),
            }
        }

        // Write data
        if write_data {
            let mut data = ctx.data.write().await;

            data.insert::<BoredomTracker>(Arc::new(AtomicBool::new(false)));
            data.remove::<BoredomMessage>();
        }
    }

    Ok(())
}

pub(crate) async fn check_for_boredom(ctx: serenity_prelude::Context) -> ! {
    loop {
        // Sleep for 2 days
        tokio::time::sleep(Duration::from_secs(60 * 60 * 24 * 2)).await;
        debug!("It's time to check for boredom!");

        let mut boredom_message_value = None;
        let mut boredom_tracker_value = None;

        // Read data
        {
            let data = ctx.data.read().await;

            match data.get::<BoredomTracker>() {
                Some(boredom_tracker) => {
                    if boredom_tracker.load(atomic::Ordering::SeqCst) {
                        debug!("... I'm bored");

                        let picked_message = choose_str(&[
                            format!("Waaaaa nobody's talking to me {}", FloofEmoji::FloofCry),
                            format!("Hello? Did you guys die? {}", FloofEmoji::FloofOwo),
                            format!("Guys... I'm bored... {}", FloofEmoji::FloofSad),
                            format!("Hi hello I am the engagement inspector, here for your bi-daily engagement inspection and- WOAH WOAH WOAH, these engagement levels are too low!!! You guys gotta start doing fun stuff right now!!! {}", FloofEmoji::FloofNervous),
                            format!("Are you ignoring me??? Nobody's said anything to me in a while... {}", FloofEmoji::FloofAngry),
                        ]);

                        match picked_message {
                            Ok(picked_message) => match ChannelId::new(1226773600258883675)
                                .say(&ctx, &picked_message)
                                .await
                            {
                                Ok(message) => {
                                    info!("Sent boredom message: {picked_message:?}");
                                    boredom_message_value =
                                        Some(Arc::new(AtomicU64::new(message.id.get())));
                                }
                                Err(err) => error!("Failed to send bored message: {err}"),
                            },
                            Err(err) => error!("Failed to choose random message: {err}"),
                        }
                    } else {
                        debug!("... I'm not bored!");
                        boredom_tracker_value = Some(Arc::new(AtomicBool::new(true)));
                    }
                }
                None => error!("Failed to get BoredomTracker"),
            }
        }

        // Write data
        {
            let mut data = ctx.data.write().await;

            match boredom_message_value {
                Some(value) => {
                    debug!("I'm inserting my boredom message");
                    data.insert::<BoredomMessage>(value);
                }
                None => {
                    debug!("I'm removing my boredom message");
                    data.remove::<BoredomMessage>();
                }
            }

            if let Some(value) = boredom_tracker_value {
                debug!("I'll be bored next time unless I'm interacted with");
                data.insert::<BoredomTracker>(value);
            }
        }
    }
}
