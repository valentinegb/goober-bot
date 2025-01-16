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

mod anon;
mod debug;
mod rock_paper_scissors;
mod silly;
mod strike;
mod timestamp;
mod updates;
#[cfg(not(debug_assertions))]
mod vote;

use std::collections::BTreeMap;

pub(super) use anon::*;
use chrono::Utc;
pub(super) use debug::*;
pub(super) use rock_paper_scissors::*;
pub(super) use silly::*;
pub(super) use strike::*;
pub(super) use timestamp::*;
pub(super) use updates::*;
#[cfg(not(debug_assertions))]
pub(super) use vote::*;

/// Omit `category` argument on a command to hide from list.
pub(super) fn print_all<U, E>(commands: &[poise::Command<U, E>]) {
    #[must_use]
    fn command_string<U, E>(command: &poise::Command<U, E>) -> String {
        let mut string = String::new();

        for subcommand in &command.subcommands {
            string += &command_string(subcommand);
        }

        if !command.subcommands.is_empty() {
            return string;
        }

        string += &format!("- `/{}", command.qualified_name);

        for parameter in &command.parameters {
            string += " ";

            if parameter.required {
                string += "<";
            } else {
                string += "[";
            }

            string += &format!("{}", parameter.name);

            if parameter.required {
                string += ">";
            } else {
                string += "]";
            }
        }

        string += "`";

        if command.name == "vote" {
            string += " ❤️";
        }

        string += "\n";

        string
    }

    let mut string = String::new();
    let mut category_keys = Vec::new();
    let mut categories: BTreeMap<&String, Vec<&poise::Command<U, E>>> = BTreeMap::new();

    for command in commands {
        if let Some(category) = &command.category {
            if !category_keys.contains(&category) {
                category_keys.push(category);
            }

            let category_commands = categories.entry(category).or_default();

            category_commands.push(command);
        }
    }

    category_keys.sort_by(|a, b| categories[b].len().cmp(&categories[a].len()));

    let other_category_key_index = category_keys
        .binary_search(&&String::from("Other"))
        .expect("there should be a command category called \"Other\"");
    let other_category_key = category_keys.remove(other_category_key_index);

    category_keys.push(other_category_key);

    string += &format!(
        "## Commands\n\n*Last updated {}*\n",
        Utc::now().format("%b %e, %Y")
    );

    for category in category_keys {
        let category_commands = &categories[category];

        string += &format!("\n### {category}\n\n");

        for command in category_commands {
            string += &command_string(command);
        }
    }

    println!("{}", string.trim_end());
}
