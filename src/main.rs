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

use std::sync::{atomic::AtomicBool, Arc};

use anyhow::Context as _;
use boredom::{check_for_boredom, check_for_boredom_acknowledgment, BoredomTracker};
use poise::serenity_prelude::{
    ClientBuilder, FullEvent, GatewayIntents, GuildId, Mentionable, User,
};
use rand::{seq::SliceRandom, thread_rng};
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;
use tracing::info;

struct UserData {} // User data, which is stored and accessible in all command invocations

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, UserData, Error>;

/// ```
/// rp_command!(
///     name: ident,
///     context_menu_name: literal,
///     description: literal,
///     user_description: literal,
///     [ message..: literal ],
///     bot_message: literal,
///     self_message: literal,
/// );
/// ```
macro_rules! rp_command {
    (
        $name:ident,
        $context_menu_name:literal,
        $description:literal,
        $user_description:literal,
        [$($message:literal),+$(,)?],
        $bot_message:literal,
        $self_message:literal$(,)?
    ) => {
        #[doc = $description]
        #[poise::command(slash_command, context_menu_command = $context_menu_name)]
        async fn $name(
            ctx: Context<'_>,
            #[description = $user_description] user: User,
        ) -> Result<(), Error> {
            let mut data = ctx.serenity_context().data.write().await;

            data.insert::<BoredomTracker>(Arc::new(AtomicBool::new(false)));

            let author_mention = ctx.author().mention();
            let bot_message = format!($bot_message, a = author_mention);
            let self_message = format!($self_message, a = author_mention);
            let messages = [
                $(format!(
                    $message,
                    a = author_mention,
                    b = user.mention(),
                )),+
            ];
            let picked_message;

            if user.id == ctx.framework().bot_id {
                picked_message = &bot_message;
            } else if user.id == ctx.author().id {
                picked_message = &self_message;
            } else {
                let mut rng = thread_rng();

                picked_message = messages
                    .choose(&mut rng)
                    .ok_or("Failed to choose random message")?;
            }

            ctx.say(picked_message).await?;
            info!("Responded to a command: {picked_message:?}");

            Ok(())
        }
    };
}

rp_command!(
    boop,
    "Boop",
    "Boops a being :3c",
    "Your victim >:3",
    [
        "{a} booped {b}!!! <:floofOwO:1226944711768412280>",
        "{b} just got booped by {a}?? <a:afloofLoad:1227015489792905360>",
        "Lmao I just saw {a} boop {b} <:floofLol:1226944692541980692>",
        "Dear {b},\n\nGet booped, nerd. <:floofSmug:1226944728734629970>\n\nSincerely, {a}",
        "{a} booped {b}, I think they're trying to pick a fight <:floofNervous:1226944704541622394>",
    ],
    "I have been booped by {a} <:floofOwO:1226944711768412280>",
    "{a} just booped themselves... that's a little sad, won't someone else boop them? <:floofSad:1226944722908483665>",
);

rp_command!(
    gnaw,
    "Gnaw Bones",
    "Embrace the bobin within us all and gnaw on one's bones",
    "The subject of today's gnawing",
    [
        "{a} is gnawing on {b}'s bones <:floofNom:1226944708366831637>",
        "{a} craves the bones of {b} <:floofNom:1226944708366831637>",
        "{a} hungers for the bones of a {b} <:floofNom:1226944708366831637>",
        "Hey uh, {b}, did you know there's a {a} gnawing on your bones? <:floofLurk:1226944909446090922>",
    ],
    "GRAAAHH {a} STOP GNAWING MY BONES GET OFF HELP <:floofScared:1226944726096285777>",
    "{a}'s gnawing on... their own bones? Are they good...? <a:afloofLoad:1227015489792905360>",
);

rp_command!(
    bite,
    "Bite",
    "Express a wide range of emotions via- your teeth in somebody's skin",
    "The skin-haver in question",
    [
        "D- did {a} just bite {b}?? <:floofOwO:1226944711768412280>",
        "Awww, {a} gave {b} a love bite... I think- actually, it's hard to say <:floofTired:1226944734640078878>",
        "The intrusive thoughts won and now {a}'s biting {b} <:floofMischief:1226944697579077692>",
        "\\*CHOMP\\*\n{a} bit {b} <:floofNom:1226944708366831637>",
    ],
    "Help please {a}'s biting me <:floofOwO:1226944711768412280>",
    "{a} bit themselves... why'd they do that? <a:afloofLoad:1227015489792905360>",
);

rp_command!(
    meow,
    "Meow At",
    "You know what you are",
    "Get their attention",
    [
        "Uhh, {a} just meowed at {b} <:floofWhat:1226944914315804683>",
        "{a} is a furry and they want {b} to know it <:floofMischief:1226944697579077692>",
        "{a} is so silly, they think {b} can understand their meowing <:floofLol:1226944692541980692>",
        "{b}, be afraid... {a} is meowing at you <:floofPeek:1226944911857815594>",
        "{b}, {a} is meowing at you, won't you give them what they want? <:floofPlead:1226944718735151266>",
        "{b}, I have a message for you: \"meow meow meow meow meow meow meow meow\"\n{a} gave it to me <:floofHappy:1226944682815258755>",
        "{a} just *nya*-ed all over the place- {b}, clean this up! <:floofWhat:1226944914315804683>",
        "{b}... sire... I have a message for you, from {a}... \\*ahem\\*... \"meow meow meow, meow meow, meow meow meow meow meow, meow!\"\nI'm just the messenger please don't hurt me <:floofNervous:1226944704541622394>",
    ],
    "Hm? What's that {a}? Oh I see... mhm... okay, okay, I understand <:floofCat:1226944674988687491>",
    "{a} is meowing at themselves lol, schizophrenic cat <:floofCat:1226944674988687491>",
);

rp_command!(
    murder,
    "Murder",
    "MURRRRRDEERRRRRRRRRRR",
    "KILL THEM KILL THEM KILL THEM >:D",
    [
        "{a} crept up behind {b} and murdered them!!! <:floofOwO:1226944711768412280>",
        "{a} just pulled out a bazooka and blew {b} up!?!? <:floofOwO:1226944711768412280>",
        "{a} stared directly into {b}'s eyes and shouted \"POMEGRANATE\", triggering the cognitohazard previously planted in {b}'s brain, killing them instantly <:floofNervous:1226944704541622394>",
        "{a} just went \"BOO\", giving {b} a fatal heart attack <:floofOwO:1226944711768412280>",
        "{a} just went \"OOGA BOOGA\", giving {b} a fatal heart attack <:floofOwO:1226944711768412280>",
        "{a} killed {b} when the lights went out so no one would know it was them... <:floofSmug:1226944728734629970>",
    ],
    "GAH {a} HAS A KNIFE AND IS RUNNING AT ME WAAAA <:floofScared:1226944726096285777>",
    "BAH- {a} JUST K-KILLED THEMSELF??? NOOOOOOOOOO <:floofScared:1226944726096285777>",
);

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![boop(), gnaw(), bite(), meow(), murder()],
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

    Ok(client.into())
}
