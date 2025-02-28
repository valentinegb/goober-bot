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

use chrono::{NaiveDateTime, Utc};
use diesel::{
    Queryable, Selectable,
    pg::Pg,
    prelude::{AsChangeset, Insertable},
};
use poise::serenity_prelude::{ChannelId, GuildId, UserId};
use serenity::all::Mentionable as _;

use crate::schema::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = opendal)]
#[diesel(check_for_backend(Pg))]
pub struct OpenDalEntry {
    pub key: String,
    pub value: Vec<u8>,
}

#[derive(Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = configs)]
#[diesel(check_for_backend(Pg))]
#[diesel(treat_none_as_null = true)]
#[diesel(primary_key(guild))]
pub struct Config {
    pub guild: GuildId,
    pub strikes_enabled: bool,
    pub strikes_log_channel: Option<ChannelId>,
    pub anon_enabled: bool,
    pub anon_channel: Option<ChannelId>,
    pub anon_log_channel: Option<ChannelId>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = strikes)]
#[diesel(check_for_backend(Pg))]
pub struct Strike {
    pub id: i32,
    pub guild: GuildId,
    pub user: UserId,
    pub issuer: UserId,
    pub issued: NaiveDateTime,
    pub rule: Option<String>,
    pub comment: Option<String>,
    pub expiration: Option<NaiveDateTime>,
    pub repealer: Option<UserId>,
}

impl Strike {
    pub fn to_string(&self, with_issuer: bool, with_issued: bool) -> String {
        let on = match with_issued {
            true => format!(" on <t:{}:d>", self.issued.and_utc().timestamp()),
            false => String::new(),
        };
        let for_breaking_rule = match self.rule {
            Some(ref rule) => format!(" for breaking **rule {rule}**"),
            None => String::new(),
        };
        let with_comment = match self.comment {
            Some(ref comment) => format!(" with comment **\"{comment}\"**"),
            None => String::new(),
        };
        let which_expires = match self.expiration {
            Some(expiration) => {
                format!(" which expires <t:{}:R>", expiration.and_utc().timestamp())
            }
            None => String::new(),
        };
        let ave = format!(
            "ave {} a strike{on}{for_breaking_rule}{with_comment}{which_expires}",
            self.user.mention(),
        );
        let message = match with_issuer {
            true => format!("{} g{ave}", self.issuer.mention()),
            false => format!("G{ave}",),
        };

        match self.repealer {
            Some(repealer) => format!("~~{message}~~ **repealed** by {}", repealer.mention()),
            None => message,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.expiration
            .is_some_and(|expiration| expiration.and_utc() <= Utc::now())
    }
}

#[derive(Insertable, Debug)]
#[diesel(table_name = strikes)]
#[diesel(treat_none_as_default_value = false)]
pub struct NewStrike {
    pub guild: GuildId,
    pub user: UserId,
    pub issuer: UserId,
    #[diesel(treat_none_as_default_value = true)]
    pub issued: Option<NaiveDateTime>,
    pub rule: Option<String>,
    pub comment: Option<String>,
    pub expiration: Option<NaiveDateTime>,
    pub repealer: Option<UserId>,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = analytics)]
#[diesel(check_for_backend(Pg))]
pub struct Analytics {
    pub command: String,
    pub invocations: Vec<NaiveDateTime>,
}
