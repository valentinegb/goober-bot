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

use poise::{
    command,
    serenity_prelude::{CreateAllowedMentions, Mentionable, UserId},
    CreateReply,
};
use rand::{seq::IteratorRandom, thread_rng};

use crate::{emoji::*, sponsors::has_early_access, Context, Error};

/// ```
/// silly_command!(
///     /// Command description
///     #[early_access] // Optional line
///     fn command_name("User description") {
///         bot_message = "Message when used on bot, must include {author}";
///         author_message = "Message when used on author, must include {author}";
///         messages = [
///             "Random messages to choose from, must include {author} and {user}",
///             "Messages to randomly choose from, must include {author} and {user}",
///         ];
///     }
/// );
/// ```
macro_rules! silly_command {
    (
        #[$doc:meta]
        $(#[$early_access:ident])?
        fn $name:ident($user_description:literal) {
            bot_message = $bot_message:literal;
            author_message = $author_message:literal;
            messages = [
                $($message:literal),+$(,)?
            ];
        }
    ) => {
        #[$doc]
        #[command(
            slash_command,
            category = "Silly",
            install_context = "Guild|User",
            interaction_context = "Guild|BotDm|PrivateChannel",
            required_bot_permissions = "USE_EXTERNAL_EMOJIS"
        )]
        pub(crate) async fn $name(
            ctx: Context<'_>,
            #[description = $user_description] user: UserId,
        ) -> Result<(), Error> {
            $(
                if stringify!($early_access) == "early_access" {
                    if !has_early_access(ctx).await? {
                        return Ok(())
                    }
                }
            )?

            let content;

            if user == ctx.framework().bot_id {
                content = format!($bot_message, author = ctx.author().mention());
            } else if user == ctx.author().id {
                content = format!($author_message, author = ctx.author().mention());
            } else {
                let mut rng = thread_rng();

                content = [
                    $(format!(
                        $message,
                        author = ctx.author().mention(),
                        user = user.mention(),
                    )),+
                ]
                .into_iter()
                .choose(&mut rng)
                .expect("List of possible message content should not be empty");
            }

            ctx.send(
                CreateReply::default()
                    .content(content)
                    .allowed_mentions(CreateAllowedMentions::new().users([user])),
            )
            .await?;

            Ok(())
        }
    };
}

silly_command! {
    /// Boops a being :3c
    fn boop("Your victim >:3") {
        bot_message = "I have been booped by {author} {FLOOF_OWO}";
        author_message = "{author} just booped themselves... that's a litle sad, won't someone else boop them? {FLOOF_SAD}";
        messages = [
            "{author} booped {user}!!! {FLOOF_OWO}",
            "{user} just got booped by {author}?? {A_FLOOF_LOAD}",
            "Lmao I just saw {author} boop {user} {FLOOF_LOL}",
            "Dear {user},\n\nGet booped, nerd. {FLOOF_SMUG}\n\nSincerely, {author}",
            "{author} booped {user}, I think they're trying to pick a fight {FLOOF_NERVOUS}",
        ];
    }
}

silly_command! {
    /// Embrace the bobin within us all and gnaw on one's bones
    fn gnaw("The subject of today's gnawing") {
        bot_message = "GRAAAHH {author} STOP GNAWING MY BONES GET OFF HELP {FLOOF_SCARED}";
        author_message = "{author}'s gnawing on... their own bones? Are they good...? {A_FLOOF_LOAD}";
        messages = [
            "{author} is gnawing on {user}'s bones {FLOOF_NOM}",
            "{author} craves the bones of {user} {FLOOF_NOM}",
            "{author} hungers for the bones of a {user} {FLOOF_NOM}",
            "Hey uh, {user}, did you know there's a {author} gnawing on your bones? {FLOOF_LURK}",
        ];
    }
}

silly_command! {
    /// Express a wide range of emotions via- your teeth in somebody's skin
    fn bite("The skin-haver in question") {
        bot_message = "Help please {author}'s biting me {FLOOF_OWO}";
        author_message = "{author} bit themselves... why'd they do that? {A_FLOOF_LOAD}";
        messages = [
            "D- did {author} just bite {user}?? {FLOOF_OWO}",
            "Awww, {author} gave {user} a love bite... I think- actually, it's hard to say {FLOOF_TIRED}",
            "The intrusive thoughts won and now {author}'s biting {user} {FLOOF_MISCHIEF}",
            "\\*CHOMP\\*\n{author} bit {user} {FLOOF_NOM}",
        ];
    }
}

silly_command! {
    /// You know what you are
    fn meow("Get their attention") {
        bot_message = "Hm? What's that {author}? Oh I see... mhm... okay, okay, I understand {FLOOF_CAT}";
        author_message = "{author} is meowing at themselves lol, schizophrenic cat {FLOOF_CAT}";
        messages = [
            "Uhh, {author} just meowed at {user} {FLOOF_WHAT}",
            "{author} is a furry and they want {user} to know it {FLOOF_MISCHIEF}",
            "{author} is so silly, they think {user} can understand their meowing {FLOOF_LOL}",
            "{user}, be afraid... {author} is meowing at you {FLOOF_PEEK}",
            "{user}, {author} is meowing at you, won't you give them what they want? {FLOOF_PLEAD}",
            "{user}, I have a message for you: \"meow meow meow meow meow meow meow meow\"\n{author} gave it to me {FLOOF_HAPPY}",
            "{author} just *nya*-ed all over the place- {user}, clean this up! {FLOOF_WHAT}",
            "{user}... sire... I have a message for you, from {author}... \\*ahem\\*... \"meow meow meow, meow meow, meow meow meow meow meow, meow!\"\nI'm just the messenger please don't hurt me {FLOOF_NERVOUS}"
        ];
    }
}

silly_command! {
    /// MURRRRRDEERRRRRRRRRRR
    fn murder("KILL THEM KILL THEM KILL THEM >:D") {
        bot_message = "GAH {author} HAS A KNIFE AND IS RUNNING AT ME WAAAA {FLOOF_SCARED}";
        author_message = "BAH- {author} JUST K-KILLED THEMSELVES??? NOOOOOOOOOO {FLOOF_SCARED}";
        messages = [
            "{author} crept up behind {user} and murdered them!!! {FLOOF_OWO}",
            "{author} just pulled out a bazooka and blew {user} up!?!? {FLOOF_OWO}",
            "{author} stared directly into {user}'s eyes and shouted \"POMEGRANATE\", triggering the cognitohazard previously planted in {user}'s brain, killing them instantly {FLOOF_NERVOUS}",
            "{author} just went \"BOO\", giving {user} a fatal heart attack {FLOOF_OWO}",
            "{author} just went \"OOGA BOOGA\", giving {user} a fatal heart attack {FLOOF_OWO}",
            "{author} killed {user} when the lights went out so no one would know it was them... {FLOOF_SMUG}",
        ];
    }
}

silly_command! {
    /// Let them know that they're a good being :>
    fn pat("Good being in question") {
        bot_message = "Awawawawa {author} gave me a pat pat on the head {FLOOF_PAT}";
        author_message = "Aw, {author} pat themselves on the head, won't someone else give them a little pat? {FLOOF_PLEAD}";
        messages = [
            "{author} gave {user} a little pat on the head {FLOOF_PAT}",
            "{author} wants {user} to know they are a good being by giving them a pat on the head {FLOOF_PAT}",
            "{user} got pat on the head by {author} {FLOOF_PAT}",
            "{user} has been selected to receive a soothing pat on the head from {author} {FLOOF_PAT}",
        ];
    }
}

silly_command! {
    /// 😳
    fn kiss("Omg who is it who is it???") {
        bot_message = "\\*gasp* oh- oh my goodness- {author} kissed me!!! {FLOOF_WOOZY}";
        author_message = "{author} kissed theirselves? ...how? {FLOOF_WHAT}";
        messages = [
            "AWWWWWWWWW- {author} gave {user} a kiss!!!! {FLOOF_PLEAD}",
            "Hehehehe, {author} gave {user} a little smooooch {FLOOF_HAPPY}",
            "OMG- GUYS- {author} JUST KISSED {user}!!! {FLOOF_PLEAD}",
            "Hehehe {author} and {user} are so cute, they just kissed each other {FLOOF_HAPPY}",
            "{author} **VIOLENTLY** pulled {user} to them and **SMOOCHED** them on the **LIPS**, not letting **ANYONE ELSE** in {FLOOF_MISCHIEF}",
        ];
    }
}

silly_command! {
    /// Doesn't this count as necromancy?
    fn revive("The deceased") {
        bot_message = "What- I- {author}, I'm not dead- {FLOOF_WHAT}";
        author_message = "... Oh my god *{author}'S **IMMORTAL**-* {FLOOF_SCARED}";
        messages = [
            "{author} performed necromancy on {user}, now they're a *zOoOmBiIeE oOoOo* {FLOOF_SMUG}",
            "{author} crouched by {user} and held `E` for a few seconds {FLOOF}",
            "{author} graciously donated a health pack to {user} {FLOOF_HAPPY}",
            "{author} performed a ritual and sacrificed a lamb to bring {user} back to life {FLOOF_OWO}",
            "In a flash of light, {author} descended upon {user} and gave them the gift of another life {FLOOF_INNOCENT}",
        ];
    }
}

silly_command! {
    /// Does somebody want a huuuug :3
    fn hug("No seriously, who wants a hug? I need to know-") {
        bot_message = "Awawawa, thanks for the hug {author} {FLOOF_HEART}";
        author_message = "{author} gave themselves a hug {FLOOF_PLEAD}";
        messages = [
            "{author} gave {user} a much needed hug {FLOOF_HEART}",
            "{author} wrapped their arms around {user} for a hug {FLOOF_HEART}",
            "Awww, {author} and {user} are hugging, so wholesome {FLOOF_PLEAD}",
            "{author} and {user} are hugging and uhh, me too, I'm also a part of the hug {FLOOF_PLEAD}",
            "Before {user} could say anything, {author} had them trapped in an embrace {FLOOF_HEART}",
        ];
    }
}

silly_command! {
    /// Knock some sense into somebody
    fn slap("Senseless somebody in question") {
        bot_message = "OW- HEY- {author} just *slapped* me, what the heck!? {FLOOF_SCARED}";
        author_message = "{author} must think they're dreaming or something, they just slapped themselves {FLOOF_OWO}";
        messages = [
            "{author} slapped {user} and shouted \"SNAP OUT OF IT\" {FLOOF_LOL}",
            "{author} tried to knock some sense into {user} by slapping them {FLOOF_LOL}",
            "{author} decided to slap {user} across the face {FLOOF_OWO}",
            "{author} slapped {user}, just cuz they felt like it {FLOOF_BLEP}",
            "{user} find themeselves facing the opposite direction after {author}'s slap turned them around {FLOOF_OWO}",
            "In slapstick fashion, {author} slapped {user} causing them to comically spin in circles {FLOOF_LOL}",
        ];
    }
}

silly_command! {
    /// *bap* *bap*
    fn bap("Bap receiver") {
        bot_message = "WHAT- oh, {author} just bapped me. What do you want buddy? {FLOOF_TIRED}";
        author_message = "{author}'s bapping themselves, they seem a little confused... {FLOOF_TIRED}";
        messages = [
            "{author} bapped {user} and {user} jumped {FLOOF_LOL}",
            "{user} was startled for a moment when {author} snuck up and bapped them {FLOOF_LOL}",
            "{author} sloooowly reached out... and then bapped {user} {FLOOF_LURK}",
            "{author} swat at and bapped {user} like a cat {FLOOF_CAT}",
            "LOOK OUT {user}, {author}'S GONNA- oh, they only bapped you {FLOOF_TIRED}",
            "{author} bapped {user} before retreating into the shadows... {FLOOF_PEEK}",
        ];
    }
}

silly_command! {
    /// FNAF style
    fn jumpscare("Night guard") {
        bot_message = "# *AAAAAAAA-*\n*{author}... \\*gasp* **jumpscared** me... jeez...* {FLOOF_SCARED}";
        author_message = "{author} looked in a mirror and went \"raaahhhh!\"\nVery scary {FLOOF_SMUG}";
        messages = [
            "{author} did the bite of '87 on {user} {FLOOF_NOM}",
            "{user} almost got to 6 AM but {author} got to them first {FLOOF_PEEK}",
            "{author} ran down the hall and before {user} could shut the door, it was game over {FLOOF_PEEK}",
            "{user} didn't catch {author} in time and they leapt from the shadows {FLOOF_LURK}",
            "{user} forgot to wind the music box and {author} went straight to them {FLOOF_MISCHIEF}",
            "{user} just realized that {author} had snuck up to them, but by then it was already too late {FLOOF_PEEK}",
            "***RAAAHHHH!!!***\n{author} jumpscared {user} {FLOOF_MISCHIEF}",
        ];
    }
}

silly_command! {
    /// Yeah you know, just, carry a person
    fn carry("Who are you carrying away?") {
        bot_message = "Oh- {author} picked me up- okay, where're we going? {FLOOF_HAPPY}";
        author_message = "{author} discovered a physics glitch and carried themselves into the sky... {FLOOF_WHAT}";
        messages = [
            "{author} just, grabbed {user} and started carrying them over their shoulder {FLOOF_OWO}",
            "Oh my- {author} is now carrying {user}! {FLOOF_OWO}",
            "{author} must be strong- they just picked {user} right up! {FLOOF_OWO}",
            "Woah! {author} picked up {user}- I wonder where {author}'s gonna take them? {FLOOF_HAPPY}",
            "Hey! {author} just stole something and took off! They stole... {user}!! {FLOOF_OWO}",
            "Awww {author}'s giving {user} a piggy back ride {FLOOF_HAPPY}",
            "Pfft, {author} just started carrying {user} under {author}'s arm as if they were luggage {FLOOF_LOL}",
        ];
    }
}

// TODO: add `/kick`?
// TODO: add `/lick`?
// TODO: add `/punch`?
// TODO: add `/stab`?
// TODO: add `/throw`?
