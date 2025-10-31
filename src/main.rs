// Goober Bot, the Discord bot
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
//
// You may contact me via electronic mail at <valentinegb@icloud.com>.

use std::env;

use dotenvy::dotenv;
use poise::{
    Framework, FrameworkOptions,
    samples::register_globally,
    serenity_prelude::{ClientBuilder, GatewayIntents},
};
use poise_error::{
    anyhow::{self, Context},
    dedup_error_chain,
};
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt as _, util::SubscriberInitExt as _};

#[tokio::main]
async fn main() {
    let registry = tracing_subscriber::registry();

    match tracing_journald::layer() {
        Ok(journald_layer) => {
            registry.with(journald_layer).init();
        }
        Err(_) => {
            let fmt_layer = tracing_subscriber::fmt::layer().with_target(false);
            let filter_layer = EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("info"))
                .unwrap();

            registry.with(fmt_layer).with(filter_layer).init();
        }
    }

    if let Err(err) = try_main().await {
        error!("A fatal error occurred: {err:#}");
    }
}

async fn try_main() -> anyhow::Result<()> {
    info!("Starting up...");
    dotenv().ok();

    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: silly::commands(),
            on_error: |error| {
                Box::pin(async move {
                    if let Err(mut err) =
                        early_access::try_handle_error_or(error, poise_error::try_handle_error)
                            .await
                    {
                        dedup_error_chain(&mut err);
                        error!("Failed to handle error: {err:#}");
                    }
                })
            },
            pre_command: |ctx| {
                Box::pin(async move {
                    info!("{} invoked {}", ctx.author().tag(), ctx.invocation_string())
                })
            },
            ..Default::default()
        })
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                info!("Online as {}", ready.user.tag());
                register_globally(ctx, &framework.options().commands).await?;
                info!("Finished setup");

                Ok(())
            })
        })
        .build();
    let mut client = ClientBuilder::new(
        env::var("GOOBER_BOT_DISCORD_TOKEN").context("couldn't fetch $GOOBER_BOT_DISCORD_TOKEN")?,
        GatewayIntents::empty(),
    )
    .framework(framework)
    .await?;

    client.start().await?;

    Ok(())
}
