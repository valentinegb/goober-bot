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

use poise::{
    command,
    serenity_prelude::{Color, CreateEmbed, CreateMessage, Timestamp, UserId},
};
use serde::{Deserialize, Serialize};

use crate::{
    config::{get_config_key, Config},
    persist::load_or_save_default,
    Context, Error,
};

#[derive(Deserialize, Serialize)]
#[non_exhaustive]
struct Strike {
    issuer: UserId,
    issued: Timestamp,
    #[serde(default)]
    expiration: Option<Timestamp>,
    #[serde(default)]
    rule: Option<u8>,
    #[serde(default)]
    comment: Option<String>,
}

enum StrikeEvent {
    Give(UserId, Strike),
}

async fn log_strike_event(ctx: Context<'_>, event: StrikeEvent) -> Result<(), Error> {
    let Config {
        strikes_log_channel,
        ..
    } = load_or_save_default(ctx, &get_config_key(ctx)?)?;

    if let Some(strikes_log_channel) = strikes_log_channel {
        match event {
            StrikeEvent::Give(user, strike) => {
                let for_breaking_rule = match strike.rule {
                    Some(rule) => format!(" for breaking rule {rule}"),
                    None => String::new(),
                };
                let with_comment = match strike.comment {
                    Some(comment) => format!(" with comment \"{comment}\""),
                    None => String::new(),
                };
                let which_expires = match strike.expiration {
                    Some(expiration) => format!(" which expires {expiration}"),
                    None => String::new(),
                };

                strikes_log_channel
                    .send_message(
                        ctx,
                        CreateMessage::new().embed(
                            CreateEmbed::new()
                                .title("Strike Given")
                                .description(format!(
                                    "{} gave {user} a strike{for_breaking_rule}{with_comment}{which_expires}",
                                    strike.issuer,
                                ))
                                .timestamp(strike.issued)
                                .color(Color::RED),
                        ),
                    )
                    .await?;
            }
        }
    }

    Ok(())
}

#[command(slash_command, subcommands("give"))]
pub(crate) async fn strike(_ctx: Context<'_>) -> Result<(), Error> {
    unreachable!()
}

#[command(slash_command)]
async fn give(ctx: Context<'_>) -> Result<(), Error> {
    todo!()
}
