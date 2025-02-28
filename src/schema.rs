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

use diesel::table;

table! {
    opendal (key) {
        key -> Text,
        value -> Bytea,
    }
}

table! {
    configs (guild) {
        guild -> BigInt,
        strikes_enabled -> Bool,
        strikes_log_channel -> Nullable<BigInt>,
        anon_enabled -> Bool,
        anon_channel -> Nullable<BigInt>,
        anon_log_channel -> Nullable<BigInt>,
    }
}

table! {
    strikes {
        id -> Integer,
        guild -> BigInt,
        user -> BigInt,
        issuer -> BigInt,
        issued -> Timestamp,
        rule -> Nullable<Text>,
        comment -> Nullable<Text>,
        expiration -> Nullable<Timestamp>,
        repealer -> Nullable<BigInt>,
    }
}

table! {
    analytics (command) {
        command -> Text,
        invocations -> Array<Timestamp>,
    }
}
