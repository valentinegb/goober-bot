use poise::{command, serenity_prelude::UserId};
use poise_error::anyhow;
use rand::{rng, seq::IndexedRandom};

struct Responses<'a> {
    self_response: &'a str,
    bot_response: &'a str,
    other_responses: &'a [&'a str],
}

impl<'a> Responses<'a> {
    async fn respond(&self, ctx: poise_error::Context<'_>, target: UserId) -> anyhow::Result<()> {
        let response = if target == ctx.author().id {
            self.self_response
        } else if target == ctx.framework().bot_id {
            self.bot_response
        } else {
            self.other_responses.choose(&mut rng()).unwrap()
        };

        ctx.reply(response).await?;

        Ok(())
    }
}

include!(concat!(env!("OUT_DIR"), "/sillies.rs"));

/// Boops a being :3c
#[command(
    slash_command,
    install_context = "Guild | User",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
async fn boop(
    ctx: poise_error::Context<'_>,
    #[description = "Your victim >:3"]
    #[rename = "user"]
    target: UserId,
) -> anyhow::Result<()> {
    const RESPONSES: Responses = Responses {
        self_response: "{author} just booped themselves... that's a little sad, won't someone else boop them? {FLOOF_SAD}",
        bot_response: "I have been booped by {author} {FLOOF_OWO}",
        other_responses: &[
            "{author} booped {target}!!! {FLOOF_OWO}",
            "{target} just got booped by {author}?? {FLOOF_LOAD_ANIMATED}",
            "Lmao I just saw {author} boop {target} {FLOOF_LOL}",
            "Dear {target},\n\nGet booped, nerd. {FLOOF_SMUG}\n\nSincerely, {author}",
            "{author} booped {target}, I think they're trying to pick a fight {FLOOF_NERVOUS}",
        ],
    };

    RESPONSES.respond(ctx, target).await
}
