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

// TODO: make general refinements to existing codebase
// TODO: add birthday announcements system
// TODO: replace mentions of specific commands with actual formatted command
//       mentions when https://github.com/serenity-rs/poise/issues/235 is
//       resolved

mod activity;
mod analytics;
mod commands;
mod config;
mod emoji;
mod error;
mod persist;
mod sponsors;

pub(crate) use crate::error::Error;

#[cfg(not(debug_assertions))]
use std::time::Duration;
use std::{
    collections::{BTreeMap, HashSet},
    fmt::Debug,
};

use analytics::analytics;
use anyhow::Context as _;
use chrono::Utc;
use config::config;
use octocrab::Octocrab;
use poise::{
    serenity_prelude::{ClientBuilder, GatewayIntents, UserId},
    Framework, FrameworkOptions,
};
use shuttle_persist_msgpack::PersistInstance;
use shuttle_runtime::{CustomError, SecretStore};
use shuttle_serenity::ShuttleSerenity;
#[cfg(not(debug_assertions))]
use tokio::spawn;
#[cfg(not(debug_assertions))]
use topgg::Autoposter;
use tracing::{error, info};

use crate::activity::start_activity_loop;

/// User data, which is stored and accessible in all command invocations
#[derive(Debug)]
struct Data {
    persist: PersistInstance,
    #[cfg(not(debug_assertions))]
    topgg_client: topgg::Client,
}

type Context<'a> = poise::Context<'a, Data, Error>;

fn print_commands<U, E>(commands: &[poise::Command<U, E>]) {
    fn print_command<U, E>(command: &poise::Command<U, E>) {
        print!("- /{}", command.qualified_name);

        for parameter in &command.parameters {
            print!(" ");

            if parameter.required {
                print!("<");
            } else {
                print!("[");
            }

            print!("{}", parameter.name);

            if parameter.required {
                print!(">");
            } else {
                print!("]");
            }
        }

        if command.name == "sponsors" {
            print!(" 💖");
        } else if command.name == "vote" {
            print!(" ❤️");
        }

        println!();
    }

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
    println!("## Commands\n");
    println!("*Last updated {}*", Utc::now().format("%b %e, %Y"));

    for category in category_keys {
        let category_commands = &categories[category];

        println!("\n### {category}\n");

        for command in category_commands {
            for subcommand in &command.subcommands {
                print_command(subcommand);
            }

            if command.subcommands.is_empty() {
                print_command(command);
            }
        }
    }
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
    #[shuttle_persist_msgpack::Persist] persist: PersistInstance,
) -> ShuttleSerenity {
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("`DISCORD_TOKEN` was not found")?;
    let github_pat = secret_store
        .get("GITHUB_PAT")
        .context("`GITHUB_PAT` was not found")?;
    #[cfg(not(debug_assertions))]
    let topgg_token = secret_store
        .get("TOPGG_TOKEN")
        .context("`TOPGG_TOKEN` was not found")?;
    #[cfg(not(debug_assertions))]
    let topgg_client = topgg::Client::new(topgg_token);
    #[cfg(not(debug_assertions))]
    let mut autoposter = Autoposter::serenity(&topgg_client, Duration::from_secs(1800));
    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: vec![
                analytics(),
                commands::anon(),
                commands::bap(),
                commands::bite(),
                commands::boop(),
                commands::carry(),
                commands::debug(),
                commands::gnaw(),
                commands::hug(),
                commands::jumpscare(),
                commands::kiss(),
                commands::meow(),
                commands::murder(),
                commands::pat(),
                commands::revive(),
                commands::rock_paper_scissors(),
                commands::slap(),
                commands::sponsors(),
                commands::strike(),
                commands::timestamp(),
                commands::updates(),
                #[cfg(not(debug_assertions))]
                commands::vote(),
                config(),
            ],
            on_error: |error| {
                Box::pin(async move {
                    if let Err(e) = error::on_error(error).await {
                        error!("Error while handling error: {e}");
                    }
                })
            },
            pre_command: |ctx| {
                Box::pin(async move {
                    if let Err(err) = analytics::increment(ctx) {
                        error!("An error occured whilst performing analytics: {err:#?}");
                    }

                    info!(
                        "{} invoked `{}`",
                        ctx.author().name,
                        ctx.invocation_string(),
                    );
                })
            },
            post_command: |ctx| {
                Box::pin(async move {
                    info!(
                        "{}'s `{}` invocation finished successfully",
                        ctx.author().name,
                        ctx.invocation_string(),
                    );
                })
            },
            owners: HashSet::from([UserId::new(1016154932354744330)]),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                start_activity_loop(ctx.clone());
                info!("Activity loop started");
                // Omit `category` argument on a command to hide from list
                print_commands(&framework.options().commands);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                info!("Commands registered");
                octocrab::initialise(Octocrab::builder().personal_token(github_pat).build()?);
                info!("GitHub authenticated");

                Ok(Data {
                    persist,
                    #[cfg(not(debug_assertions))]
                    topgg_client,
                })
            })
        })
        .build();
    #[allow(unused_mut)]
    let mut client_builder =
        ClientBuilder::new(discord_token, GatewayIntents::GUILDS).framework(framework);

    #[cfg(not(debug_assertions))]
    {
        client_builder = client_builder.event_handler_arc(autoposter.handler());

        info!("Top.gg autoposter handler passed to client builder");
        spawn(async move {
            loop {
                if let Some(result) = autoposter.recv().await {
                    match result {
                        Ok(_) => info!("Autoposter posted stats successfully"),
                        Err(err) => error!("Autoposter returned an error: {err:#?}"),
                    }
                }
            }
        });
        info!("Began awaiting Top.gg autoposter responses");
    }

    let client = client_builder.await.map_err(CustomError::new)?;

    Ok(client.into())
}
