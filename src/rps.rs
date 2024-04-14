use poise::serenity_prelude::{Mentionable, User};

use crate::{utility::choose_str, Context, Error};

/// Challenges somebody to a game of Rock Paper Scissors
#[poise::command(slash_command)]
pub(super) async fn rps(
    ctx: Context<'_>,
    #[description = "User you're challenging"] user: User,
) -> Result<(), Error> {
    let a = ctx.author().mention();
    let b = user.mention();

    ctx.say(choose_str(&[
        format!("{b}, {a} has challenged you to Rock Paper Scissors!! Do you accept?"),
        format!("{a} wants to fight {b} in a game of Rock Paper Scissors!! {b}, do you accept?"),
        format!(""),
    ])?)
    .await?;

    todo!()
}
