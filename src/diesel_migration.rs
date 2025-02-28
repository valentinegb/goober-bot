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

use std::{collections::HashMap, str::FromStr};

use chrono::{DateTime, Utc};
use diesel::{ExpressionMethods as _, upsert::excluded};
use diesel_async::{AsyncPgConnection, RunQueryDsl as _, pooled_connection::deadpool};
use poise_error::anyhow::{self, anyhow};
use serenity::all::{GuildId, UserId};

use crate::{
    models::{Analytics, Config, NewStrike, OpenDalEntry},
    schema::{self, opendal::dsl::*},
};

pub async fn migrate_opendal_to_diesel(
    pool: &deadpool::Pool<AsyncPgConnection>,
) -> Result<(), anyhow::Error> {
    let mut conn = pool.get().await?;
    let entries: Vec<OpenDalEntry> = opendal.load(&mut conn).await?;

    for entry in entries {
        if entry.key == "analytics" {
            let analytics: HashMap<String, Vec<DateTime<Utc>>> =
                serde_json::from_slice(&entry.value)?;
            let analytics: Vec<Analytics> = analytics
                .into_iter()
                .map(|(command, invocations)| Analytics {
                    command,
                    invocations: invocations
                        .into_iter()
                        .map(|date_time| date_time.naive_utc())
                        .collect(),
                })
                .collect();

            diesel::insert_into(schema::analytics::table)
                .values(analytics)
                .on_conflict(schema::analytics::columns::command)
                .do_update()
                .set(
                    schema::analytics::columns::invocations
                        .eq(excluded(schema::analytics::columns::invocations)),
                )
                .execute(&mut conn)
                .await?;
        } else if entry.key.starts_with("strikes") {
            let (guild, user) = entry
                .key
                .trim_start_matches("strikes_")
                .split_once('_')
                .ok_or(anyhow!("expected strikes key to have guild ID and user ID"))?;
            let guild = GuildId::from_str(guild)?;
            let user = UserId::from_str(user)?;
            let array: Vec<serde_json::Map<String, serde_json::Value>> =
                serde_json::from_slice(&entry.value)?;
            let strikes =
                array
                    .into_iter()
                    .map(|object| {
                        Ok::<_, anyhow::Error>(NewStrike {
                            guild,
                            user,
                            issuer: object["issuer"]
                                .as_str()
                                .ok_or(anyhow!("expected \"issuer\" property to be a string"))?
                                .parse()?,
                            issued: Some(
                                serenity::all::Timestamp::parse(object["issued"].as_str().ok_or(
                                    anyhow!("expected \"issued\" property to be a string"),
                                )?)?
                                .naive_utc(),
                            ),
                            rule: object["rule"]
                                .as_str()
                                .map(|rule| rule.to_string())
                                .or_else(|| object["rule"].as_u64().map(|rule| rule.to_string())),
                            comment: object["comment"]
                                .as_str()
                                .map(|comment| comment.to_string()),
                            expiration: object["expiration"].as_str().and_then(|expiration| {
                                serenity::all::Timestamp::parse(expiration)
                                    .ok()
                                    .map(|timestamp| timestamp.naive_utc())
                            }),
                            repealer: object["repealer"]
                                .as_str()
                                .and_then(|repealer| repealer.parse().ok()),
                        })
                    })
                    .collect::<Result<Vec<NewStrike>, _>>()?;

            diesel::insert_into(schema::strikes::table)
                .values(strikes)
                .execute(&mut conn)
                .await?;
        } else if entry.key.starts_with("config") {
            let guild = GuildId::from_str(entry.key.trim_start_matches("config_"))?;
            let object: serde_json::Map<String, serde_json::Value> =
                serde_json::from_slice(&entry.value)?;
            let config = Config {
                guild,
                strikes_enabled: object["strikes_enabled"].as_bool().ok_or(anyhow!(
                    "expected \"strikes_enabled\" property to be a bool"
                ))?,
                strikes_log_channel: object["strikes_log_channel"]
                    .as_str()
                    .and_then(|strikes_log_channel| strikes_log_channel.parse().ok()),
                anon_enabled: object["anon_enabled"]
                    .as_bool()
                    .ok_or(anyhow!("expected \"anon_enabled\" property to be a bool"))?,
                anon_channel: object["anon_channel"]
                    .as_str()
                    .and_then(|anon_channel| anon_channel.parse().ok()),
                anon_log_channel: object["anon_log_channel"]
                    .as_str()
                    .and_then(|anon_log_channel| anon_log_channel.parse().ok()),
            };

            diesel::insert_into(schema::configs::table)
                .values(&config)
                .on_conflict(schema::configs::columns::guild)
                .do_update()
                .set(&config)
                .execute(&mut conn)
                .await?;
        }
    }

    Ok(())
}
