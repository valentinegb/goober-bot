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

use std::{
    fmt,
    sync::{atomic::AtomicBool, Arc},
};

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

/// ```
/// rp_command!(
///     name: ident,
///     context_menu_name: literal,
///     description: literal,
///     user_description: literal,
///     [ (message: literal, emoji: expr).. ],
///     (bot_message: literal, emoji: expr),
///     (self_message: literal, emoji: expr),
/// );
/// ```
///
/// Variables usable in messages:
///
/// - `a`: mention of the user that invoked the command
/// - `b`: mention of the user the command is being used on
/// - `e`: emoji for the message
///
/// Example: `"{a} did something to {b} {e}"`
macro_rules! rp_command {
    (
        $name:ident,
        $context_menu_name:literal,
        $description:literal,
        $user_description:literal,
        [$(($message:literal, $emoji:expr$(,)?)),+$(,)?],
        ($bot_message:literal, $bot_emoji:expr$(,)?),
        ($self_message:literal, $self_emoji:expr$(,)?)$(,)?
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
            let bot_message = format!(
                $bot_message,
                a = author_mention,
                e = $bot_emoji,
            );
            let self_message = format!(
                $self_message,
                a = author_mention,
                e = $self_emoji,
            );
            let messages = [
                $(format!(
                    $message,
                    a = author_mention,
                    b = user.mention(),
                    e = $emoji,
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
        ("{a} booped {b}!!! {e}", FloofEmoji::FloofOwo),
        ("{b} just got booped by {a}?? {e}", FloofEmoji::AFloofLoad),
        ("Lmao I just saw {a} boop {b} {e}", FloofEmoji::FloofLol),
        (
            "Dear {b},\n\nGet booped, nerd. {e}\n\nSincerely, {a}",
            FloofEmoji::FloofSmug
        ),
        (
            "{a} booped {b}, I think they're trying to pick a fight {e}",
            FloofEmoji::FloofNervous
        ),
    ],
    ("I have been booped by {a} {e}", FloofEmoji::FloofOwo),
    (
        "{a} just booped themselves... that's a little sad, won't someone else boop them? {e}",
        FloofEmoji::FloofSad,
    ),
);

rp_command!(
    gnaw,
    "Gnaw Bones",
    "Embrace the bobin within us all and gnaw on one's bones",
    "The subject of today's gnawing",
    [
        ("{a} is gnawing on {b}'s bones {e}", FloofEmoji::FloofNom),
        ("{a} craves the bones of {b} {e}", FloofEmoji::FloofNom),
        (
            "{a} hungers for the bones of a {b} {e}",
            FloofEmoji::FloofNom,
        ),
        (
            "Hey uh, {b}, did you know there's a {a} gnawing on your bones? {e}",
            FloofEmoji::FloofLurk,
        ),
    ],
    (
        "GRAAAHH {a} STOP GNAWING MY BONES GET OFF HELP {e}",
        FloofEmoji::FloofScared,
    ),
    (
        "{a}'s gnawing on... their own bones? Are they good...? {e}",
        FloofEmoji::AFloofLoad,
    ),
);

rp_command!(
    bite,
    "Bite",
    "Express a wide range of emotions via- your teeth in somebody's skin",
    "The skin-haver in question",
    [
        ("D- did {a} just bite {b}?? {e}", FloofEmoji::FloofOwo),
        (
            "Awww, {a} gave {b} a love bite... I think- actually, it's hard to say {e}",
            FloofEmoji::FloofTired,
        ),
        (
            "The intrusive thoughts won and now {a}'s biting {b} {e}",
            FloofEmoji::FloofMischief,
        ),
        ("\\*CHOMP\\*\n{a} bit {b} {e}", FloofEmoji::FloofNom),
    ],
    ("Help please {a}'s biting me {e}", FloofEmoji::FloofOwo),
    (
        "{a} bit themselves... why'd they do that? {e}",
        FloofEmoji::AFloofLoad,
    ),
);

rp_command!(
    meow,
    "Meow At",
    "You know what you are",
    "Get their attention",
    [
        ("Uhh, {a} just meowed at {b} {e}", FloofEmoji::FloofWhat),
        ("{a} is a furry and they want {b} to know it {e}", FloofEmoji::FloofMischief),
        ("{a} is so silly, they think {b} can understand their meowing {e}", FloofEmoji::FloofLol),
        ("{b}, be afraid... {a} is meowing at you {e}", FloofEmoji::FloofPeek),
        ("{b}, {a} is meowing at you, won't you give them what they want? {e}", FloofEmoji::FloofPlead),
        ("{b}, I have a message for you: \"meow meow meow meow meow meow meow meow\"\n{a} gave it to me {e}", FloofEmoji::FloofHappy),
        ("{a} just *nya*-ed all over the place- {b}, clean this up! {e}", FloofEmoji::FloofWhat),
        ("{b}... sire... I have a message for you, from {a}... \\*ahem\\*... \"meow meow meow, meow meow, meow meow meow meow meow, meow!\"\nI'm just the messenger please don't hurt me {e}", FloofEmoji::FloofNervous),
    ],
    ("Hm? What's that {a}? Oh I see... mhm... okay, okay, I understand {e}", FloofEmoji::FloofCat),
    ("{a} is meowing at themselves lol, schizophrenic cat {e}", FloofEmoji::FloofCat),
);

rp_command!(
    murder,
    "Murder",
    "MURRRRRDEERRRRRRRRRRR",
    "KILL THEM KILL THEM KILL THEM >:D",
    [
        ("{a} crept up behind {b} and murdered them!!! {e}", FloofEmoji::FloofOwo),
        ("{a} just pulled out a bazooka and blew {b} up!?!? {e}", FloofEmoji::FloofOwo),
        ("{a} stared directly into {b}'s eyes and shouted \"POMEGRANATE\", triggering the cognitohazard previously planted in {b}'s brain, killing them instantly {e}", FloofEmoji::FloofNervous),
        ("{a} just went \"BOO\", giving {b} a fatal heart attack {e}", FloofEmoji::FloofOwo),
        ("{a} just went \"OOGA BOOGA\", giving {b} a fatal heart attack {e}", FloofEmoji::FloofOwo),
        ("{a} killed {b} when the lights went out so no one would know it was them... {e}", FloofEmoji::FloofSmug),
    ],
    ("GAH {a} HAS A KNIFE AND IS RUNNING AT ME WAAAA {e}", FloofEmoji::FloofScared),
    ("BAH- {a} JUST K-KILLED THEMSELF??? NOOOOOOOOOO {e}", FloofEmoji::FloofScared),
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
