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

use chrono::{Datelike, FixedOffset, TimeZone, Timelike, Utc};
use poise::{
    command,
    serenity_prelude::{FormattedTimestamp, FormattedTimestampStyle},
    ChoiceParameter,
};
use poise_error::{
    anyhow::{anyhow, Context as _},
    UserError,
};

use crate::{emoji::*, Context};

const SECONDS_PER_HOUR: i32 = 3600;

#[derive(ChoiceParameter)]
enum FormattedTimestampStyleChoice {
    #[name = "Short time"]
    ShortTime,
    #[name = "Long time"]
    LongTime,
    #[name = "Short date"]
    ShortDate,
    #[name = "Long date"]
    LongDate,
    #[name = "Short date time"]
    ShortDateTime,
    #[name = "Long date time"]
    LongDateTime,
    #[name = "Relative time"]
    RelativeTime,
}

impl From<FormattedTimestampStyleChoice> for FormattedTimestampStyle {
    fn from(value: FormattedTimestampStyleChoice) -> Self {
        match value {
            FormattedTimestampStyleChoice::ShortTime => FormattedTimestampStyle::ShortTime,
            FormattedTimestampStyleChoice::LongTime => FormattedTimestampStyle::LongTime,
            FormattedTimestampStyleChoice::ShortDate => FormattedTimestampStyle::ShortDate,
            FormattedTimestampStyleChoice::LongDate => FormattedTimestampStyle::LongDate,
            FormattedTimestampStyleChoice::ShortDateTime => FormattedTimestampStyle::ShortDateTime,
            FormattedTimestampStyleChoice::LongDateTime => FormattedTimestampStyle::LongDateTime,
            FormattedTimestampStyleChoice::RelativeTime => FormattedTimestampStyle::RelativeTime,
        }
    }
}

/// Generates a Unix timestamp for your use in Discord styled messages
#[allow(clippy::too_many_arguments)]
#[command(
    slash_command,
    category = "Other",
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel",
    ephemeral
)]
pub(crate) async fn timestamp(
    ctx: Context<'_>,
    #[description = "Default is current year"]
    // Where the heck did these big ol' numbers come from?
    #[min = -262142]
    #[max = 262141]
    year: Option<i32>,
    #[description = "Default is current month"]
    #[min = 1]
    #[max = 12]
    month: Option<u32>,
    #[description = "Default is current day"]
    #[min = 1]
    #[max = 31]
    day: Option<u32>,
    #[description = "Default is current hour"]
    #[max = 23]
    hour: Option<u32>,
    #[description = "Default is current minute"]
    #[max = 59]
    minute: Option<u32>,
    #[description = "Default is current second"]
    #[max = 59]
    second: Option<u32>,
    #[description = "Offset from UTC (default is 0)"]
    #[min = -12]
    #[max = 14]
    timezone: Option<i32>,
    #[description = "Default is short date time"] style: Option<FormattedTimestampStyleChoice>,
) -> Result<(), poise_error::anyhow::Error> {
    let timezone = timezone.unwrap_or(0);
    let fixed_offset = FixedOffset::east_opt(timezone * SECONDS_PER_HOUR)
        .context(UserError(anyhow!("entered timezone difference is invalid")))?;
    let now = Utc::now().with_timezone(&fixed_offset);
    let year = year.unwrap_or_else(|| now.year());
    let month = month.unwrap_or_else(|| now.month());
    let day = day.unwrap_or_else(|| now.day());
    let hour = hour.unwrap_or_else(|| now.hour());
    let minute = minute.unwrap_or_else(|| now.minute());
    let second = second.unwrap_or_else(|| now.second());
    let datetime = fixed_offset
        .with_ymd_and_hms(year, month, day, hour, minute, second)
        .earliest()
        .context(UserError(anyhow!("entered date/time is invalid")))?;
    let formatted_timestamp = FormattedTimestamp::new(datetime.into(), style.map(|s| s.into()));

    ctx.say(format!("Copy this and use it anywhere that supports Discord formatting {FLOOF_HAPPY}\n```\n{formatted_timestamp}\n```\nLooks like this btw: {formatted_timestamp}\n*If this isn't the timestamp you expected, make sure you set `timezone` to your timezone!*")).await?;

    Ok(())
}
