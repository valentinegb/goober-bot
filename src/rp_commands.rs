use std::sync::{atomic::AtomicBool, Arc};

use poise::serenity_prelude::{Mentionable, User};
use rand::{seq::SliceRandom, thread_rng};
use tracing::info;

use super::{boredom::BoredomTracker, Context, Error, FloofEmoji};

/// ```
/// rp_command!(
///     name: ident,
///     description: literal,
///     user_description: literal,
///     [ (message: literal, emoji: expr).. ],
///     (bot_message: literal, bot_emoji: expr),
///     (self_message: literal, self_emoji: expr),
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
        $description:literal,
        $user_description:literal,
        [$(($message:literal, $emoji:expr$(,)?)),+$(,)?],
        ($bot_message:literal, $bot_emoji:expr$(,)?),
        ($self_message:literal, $self_emoji:expr$(,)?)$(,)?
    ) => {
        #[doc = $description]
        #[poise::command(slash_command)]
        pub(super) async fn $name(
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
    ("BAH- {a} JUST K-KILLED THEMSELVES??? NOOOOOOOOOO {e}", FloofEmoji::FloofScared),
);

rp_command!(
    pat,
    "Let them know that they're a good being :>",
    "Good being in question",
    [
        (
            "{a} gave {b} a little pat on the head {e}",
            FloofEmoji::FloofPat,
        ),
        (
            "{a} wants {b} to know they are a good being by giving them a pat on the head {e}",
            FloofEmoji::FloofPat,
        ),
        ("{b} got pat on the head by {a} {e}", FloofEmoji::FloofPat),
        (
            "{b} has been selected to receive a soothing pat on the head from {a} {e}",
            FloofEmoji::FloofPat,
        ),
    ],
    (
        "Awawawawa {a} gave me a pat pat on the head {e}",
        FloofEmoji::FloofPat,
    ),
    (
        "Aw, {a} pat themselves on the head, won't someone else give them a little pat? {e}",
        FloofEmoji::FloofPlead,
    ),
);
