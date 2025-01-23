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

use std::collections::BTreeMap;

use chrono::Utc;
use poise::{command, ChoiceParameter};
use poise_error::{
    anyhow::{anyhow, bail},
    UserError,
};

use crate::{config::get_config_key, emoji::*, Context, Data};

#[derive(ChoiceParameter)]
enum ErrorKind {
    User,
    Internal,
    Panic,
}

/// Commands to aid in development of the bot
#[command(slash_command, subcommands("error", "delete_config", "commands"))]
pub(crate) async fn debug(_ctx: Context<'_>) -> Result<(), poise_error::anyhow::Error> {
    unreachable!();
}

/// Fails intentionally
#[command(slash_command)]
async fn error(
    _ctx: Context<'_>,
    #[description = "Kind of error to return"] kind: ErrorKind,
) -> Result<(), poise_error::anyhow::Error> {
    match kind {
        ErrorKind::User => bail!(UserError(
            anyhow!("This is an example of a user error")
                .context("This is an example of extra context")
        )),
        ErrorKind::Internal => Err(anyhow!("This is an example of an internal error")
            .context("This is an example of extra context")),
        ErrorKind::Panic => panic!("This is an example of a panic"),
    }
}

/// Deletes the config file for the current server
#[command(
    slash_command,
    required_permissions = "MANAGE_GUILD",
    required_bot_permissions = "USE_EXTERNAL_EMOJIS",
    ephemeral
)]
async fn delete_config(ctx: Context<'_>) -> Result<(), poise_error::anyhow::Error> {
    ctx.data().op.0.delete(&get_config_key(ctx)?).await?;
    ctx.say(format!("Server config file deleted {FLOOF_MUG}"))
        .await?;

    Ok(())
}

/// Prints the list of commands that goes in the bot's GitHub README
#[command(slash_command, ephemeral)]
async fn commands(ctx: Context<'_>) -> Result<(), poise_error::anyhow::Error> {
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

            string += &parameter.name;

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
    let mut categories: BTreeMap<&String, Vec<&poise::Command<Data, poise_error::anyhow::Error>>> =
        BTreeMap::new();

    for command in &ctx.framework().options.commands {
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

    ctx.say(format!("```md\n{}\n```", string.trim_end()))
        .await?;

    Ok(())
}
