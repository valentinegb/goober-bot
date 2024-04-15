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

mod boredom;
mod confess;
mod rp_commands;
mod rps;
mod utility;

use std::{
    fmt,
    sync::{atomic::AtomicBool, Arc},
};

use anyhow::Context as _;
use poise::serenity_prelude::{ClientBuilder, FullEvent, GatewayIntents, GuildId};
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;
use tracing::info;

use crate::{
    boredom::{check_for_boredom, check_for_boredom_acknowledgment, BoredomTracker},
    confess::confess,
    rp_commands::{bite, boop, gnaw, meow, murder, pat},
};

/// User data, which is stored and accessible in all command invocations
struct UserData {
    confessions_webhook_url: String,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, UserData, Error>;

#[allow(dead_code)]
enum FloofEmoji {
    AFloofLoad,
    Floof,
    FloofAngry,
    FloofBlep,
    FloofCat,
    FloofCool,
    FloofCry,
    FloofDrool,
    FloofHappy,
    FloofHeart,
    FloofInnocent,
    FloofLoad,
    FloofLol,
    FloofLurk,
    FloofMischief,
    FloofMug,
    FloofNervous,
    FloofNom,
    FloofOwo,
    FloofPat,
    FloofPeek,
    FloofPlead,
    FloofSad,
    FloofScared,
    FloofSmug,
    FloofTeehee,
    FloofTired,
    FloofWhat,
    FloofWoozy,
}

impl fmt::Display for FloofEmoji {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FloofEmoji::AFloofLoad => write!(f, "<a:afloofLoad:1229166166292041758>"),
            FloofEmoji::Floof => write!(f, "<:floof:1229166168724738119>"),
            FloofEmoji::FloofAngry => write!(f, "<:floofAngry:1229166171379466290>"),
            FloofEmoji::FloofBlep => write!(f, "<:floofBlep:1229166174051373136>"),
            FloofEmoji::FloofCat => write!(f, "<:floofCat:1229166176144326737>"),
            FloofEmoji::FloofCool => write!(f, "<:floofCool:1229166178707177502>"),
            FloofEmoji::FloofCry => write!(f, "<:floofCry:1229166181265444946>"),
            FloofEmoji::FloofDrool => write!(f, "<:floofDrool:1229166183757119518>"),
            FloofEmoji::FloofHappy => write!(f, "<:floofHappy:1229166186084958239>"),
            FloofEmoji::FloofHeart => write!(f, "<:floofHeart:1229166188240699403>"),
            FloofEmoji::FloofInnocent => write!(f, "<:floofInnocent:1229166190794899547>"),
            FloofEmoji::FloofLoad => write!(f, "<:floofLoad:1229166192816685127>"),
            FloofEmoji::FloofLol => write!(f, "<:floofLol:1229166195761221683>"),
            FloofEmoji::FloofLurk => write!(f, "<:floofLurk:1229166198017495090>"),
            FloofEmoji::FloofMischief => write!(f, "<:floofMischief:1229166200576282686>"),
            FloofEmoji::FloofMug => write!(f, "<:floofMug:1229166203029819482>"),
            FloofEmoji::FloofNervous => write!(f, "<:floofNervous:1229166205579821176>"),
            FloofEmoji::FloofNom => write!(f, "<:floofNom:1229166330473873539>"),
            FloofEmoji::FloofOwo => write!(f, "<:floofOwO:1229166210382434335>"),
            FloofEmoji::FloofPat => write!(f, "<:floofPat:1229166213268246580>"),
            FloofEmoji::FloofPeek => write!(f, "<:floofPeek:1229166216669696131>"),
            FloofEmoji::FloofPlead => write!(f, "<:floofPlead:1229166219131621387>"),
            FloofEmoji::FloofSad => write!(f, "<:floofSad:1229166333090861177>"),
            FloofEmoji::FloofScared => write!(f, "<:floofScared:1229166224722759680>"),
            FloofEmoji::FloofSmug => write!(f, "<:floofSmug:1229166227038011434>"),
            FloofEmoji::FloofTeehee => write!(f, "<:floofTeehee:1229166230527803423>"),
            FloofEmoji::FloofTired => write!(f, "<:floofTired:1229166232960503910>"),
            FloofEmoji::FloofWhat => write!(f, "<:floofWhat:1229166340036886599>"),
            FloofEmoji::FloofWoozy => write!(f, "<:floofWoozy:1229166238106914990>"),
        }
    }
}

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;
    let confessions_webhook_url = secret_store
        .get("CONFESSIONS_WEBHOOK_URL")
        .context("'CONFESSIONS_WEBHOOK_URL' was not found")?;
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![boop(), gnaw(), bite(), meow(), murder(), pat(), confess()],
            event_handler: |ctx, event, _framework, _data| {
                Box::pin(async move {
                    if let FullEvent::Message { new_message } = event {
                        check_for_boredom_acknowledgment(ctx, new_message).await?;
                    }

                    Ok(())
                })
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(
                    ctx,
                    &framework.options().commands,
                    GuildId::new(1225919005362098176),
                )
                .await?;
                info!("Registered commands");

                let mut data = ctx.data.write().await;

                data.insert::<BoredomTracker>(Arc::new(AtomicBool::new(true)));
                info!("Initialized BoredomTracker");
                tokio::spawn(check_for_boredom(ctx.clone()));
                info!("Started checking for boredom");

                Ok(UserData {
                    confessions_webhook_url,
                })
            })
        })
        .build();
    let client = ClientBuilder::new(discord_token, GatewayIntents::non_privileged())
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    info!("Constructed client");

    Ok(client.into())
}
