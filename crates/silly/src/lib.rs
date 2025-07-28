use early_access::has_early_access;
use emoji::substitute_emojis;
use poise::{
    command,
    serenity_prelude::{Mentionable, UserId},
};
use poise_error::anyhow;
use rand::{rng, seq::IndexedRandom};

struct Responses<'a> {
    self_response: &'a str,
    bot_response: &'a str,
    other_responses: &'a [&'a str],
}

impl<'a> Responses<'a> {
    async fn respond(&self, ctx: poise_error::Context<'_>, target: UserId) -> anyhow::Result<()> {
        let author = ctx.author();
        let response = if target == author.id {
            self.self_response
        } else if target == ctx.framework().bot_id {
            self.bot_response
        } else {
            self.other_responses.choose(&mut rng()).unwrap()
        };

        ctx.reply(
            substitute_emojis(response)
                .replace("{author}", &author.to_string())
                .replace("{target}", &target.mention().to_string()),
        )
        .await?;

        Ok(())
    }
}

include!(concat!(env!("OUT_DIR"), "/sillies.rs"));
