use poise::serenity_prelude::User;

use crate::{utility::choose_str, Context, Error};

/// Challenges somebody to a game of Rock Paper Scissors
#[poise::command(slash_command)]
pub(super) async fn rps(
    ctx: Context<'_>,
    #[description = "User you're challenging"] user: User,
) -> Result<(), Error> {
    choose_str(&["This is a test...", "... of the new utility function"])?;

    todo!()
}
