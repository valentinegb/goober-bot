use poise::{
    command,
    serenity_prelude::{CreateAllowedMentions, Mentionable, UserId},
    CreateReply,
};
use rand::{seq::IteratorRandom, thread_rng};

use crate::{emoji::*, Context, Error};

/// ```
/// fun_command!(
///     /// Command description
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
macro_rules! fun_command {
    (
        #[$doc:meta]
        fn $name:ident($user_description:literal) {
            bot_message = $bot_message:literal;
            author_message = $author_message:literal;
            messages = [
                $($message:literal),+$(,)?
            ];
        }
    ) => {
        #[$doc]
        #[command(slash_command)]
        pub async fn $name(
            ctx: Context<'_>,
            #[description = $user_description] user: UserId,
        ) -> Result<(), Error> {
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

fun_command!(
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
);

fun_command!(
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
);

fun_command!(
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
);
