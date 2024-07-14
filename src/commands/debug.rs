use anyhow::anyhow;
use poise::command;

use crate::{Context, Error};

/// Commands to aid in development of the bot
#[command(slash_command, subcommands("error"))]
pub async fn debug(_ctx: Context<'_>) -> Result<(), Error> {
    unreachable!();
}

/// Fails intentionally
#[command(slash_command)]
async fn error(_ctx: Context<'_>) -> Result<(), Error> {
    Err(anyhow!("This is a test error").context("This is a wrapper test error"))
}
