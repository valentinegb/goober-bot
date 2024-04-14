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
    rp_commands::{bite, boop, gnaw, meow, murder, pat},
};

struct UserData {} // User data, which is stored and accessible in all command invocations

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
            FloofEmoji::AFloofLoad => write!(f, "<a:afloofLoad:1227015489792905360>"),
            FloofEmoji::Floof => write!(f, "<:floof:1226944669448147044>"),
            FloofEmoji::FloofAngry => write!(f, "<:floofAngry:1226944671423660133>"),
            FloofEmoji::FloofBlep => write!(f, "<:floofBlep:1226944673281609788>"),
            FloofEmoji::FloofCat => write!(f, "<:floofCat:1226944674988687491>"),
            FloofEmoji::FloofCool => write!(f, "<:floofCool:1226944677387698226>"),
            FloofEmoji::FloofCry => write!(f, "<:floofCry:1226944679833112598>"),
            FloofEmoji::FloofDrool => write!(f, "<:floofDrool:1226944681477406801>"),
            FloofEmoji::FloofHappy => write!(f, "<:floofHappy:1226944682815258755>"),
            FloofEmoji::FloofHeart => write!(f, "<:floofHeart:1226944685210341467>"),
            FloofEmoji::FloofInnocent => write!(f, "<:floofInnocent:1226944687412215828>"),
            FloofEmoji::FloofLoad => write!(f, "<:floofLoad:1226944689546989710>"),
            FloofEmoji::FloofLol => write!(f, "<:floofLol:1226944692541980692>"),
            FloofEmoji::FloofLurk => write!(f, "<:floofLurk:1226944909446090922>"),
            FloofEmoji::FloofMischief => write!(f, "<:floofMischief:1226944697579077692>"),
            FloofEmoji::FloofMug => write!(f, "<:floofMug:1226944701345828904>"),
            FloofEmoji::FloofNervous => write!(f, "<:floofNervous:1226944704541622394>"),
            FloofEmoji::FloofNom => write!(f, "<:floofNom:1226944708366831637>"),
            FloofEmoji::FloofOwo => write!(f, "<:floofOwO:1226944711768412280>"),
            FloofEmoji::FloofPat => write!(f, "<:floofPat:1226944714234794044>"),
            FloofEmoji::FloofPeek => write!(f, "<:floofPeek:1226944911857815594>"),
            FloofEmoji::FloofPlead => write!(f, "<:floofPlead:1226944718735151266>"),
            FloofEmoji::FloofSad => write!(f, "<:floofSad:1226944722908483665>"),
            FloofEmoji::FloofScared => write!(f, "<:floofScared:1226944726096285777>"),
            FloofEmoji::FloofSmug => write!(f, "<:floofSmug:1226944728734629970>"),
            FloofEmoji::FloofTeehee => write!(f, "<:floofTeehee:1226944732169502761>"),
            FloofEmoji::FloofTired => write!(f, "<:floofTired:1226944734640078878>"),
            FloofEmoji::FloofWhat => write!(f, "<:floofWhat:1226944914315804683>"),
            FloofEmoji::FloofWoozy => write!(f, "<:floofWoozy:1226944739593424957>"),
        }
    }
}

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![boop(), gnaw(), bite(), meow(), murder(), pat()],
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

                Ok(UserData {})
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
