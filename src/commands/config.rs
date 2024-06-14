use anyhow::{bail, Context as _};
use paste::paste;
use poise::{
    serenity_prelude::{Color, CreateEmbed, Timestamp},
    CreateReply,
};
use sqlx::{mysql::MySqlQueryResult, Column, Row, TypeInfo};

use crate::{Context, Error};

/// Returns the inner representation of the ID of the guild the context is in.
fn get_current_guild_id(ctx: Context<'_>) -> Result<u64, Error> {
    Ok(ctx.guild_id().context("Not in a guild")?.get())
}

/// Ensures that a config exists for the current guild.
async fn ensure_config_exists(ctx: Context<'_>) -> Result<MySqlQueryResult, Error> {
    let guild_id = get_current_guild_id(ctx)?;

    Ok(
        sqlx::query(
            "INSERT INTO configs (guild_id) VALUE (?) ON DUPLICATE KEY UPDATE guild_id = ?",
        )
        .bind(guild_id)
        .bind(guild_id)
        .execute(&ctx.data().pool)
        .await?,
    )
}

/// Subcommands related to getting and setting server configuration
#[poise::command(
    slash_command,
    subcommands("list", "get", "set"),
    install_context = "Guild",
    interaction_context = "Guild",
    default_member_permissions = "MANAGE_GUILD"
)]
pub(crate) async fn config(_ctx: Context<'_>) -> Result<(), Error> {
    unreachable!()
}

/// List all server configuration options and their current values
#[poise::command(slash_command, ephemeral)]
async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let mut embed = CreateEmbed::new()
        .title("Configuration")
        .description("Options and their values for this server.")
        .timestamp(Timestamp::now())
        .color(Color::ORANGE);

    ensure_config_exists(ctx).await?;

    let row = sqlx::query("SELECT * FROM configs WHERE guild_id = ?")
        .bind(get_current_guild_id(ctx)?)
        .fetch_one(&ctx.data().pool)
        .await?;

    for column in row.columns() {
        let name = column.name();

        if name == "guild_id" {
            continue;
        }

        let value: String = match column.type_info().name() {
            "BOOLEAN" => row.try_get::<bool, _>(name)?.to_string(),
            other => bail!("Unable to display column `{name}` with type `{other}`"),
        };

        embed = embed.field(format!("`{name}`"), format!("`{value}`"), false);
    }

    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}

/// Get a specific configuration option
#[poise::command(slash_command, subcommands("get_strikes_enabled"))]
async fn get(_ctx: Context<'_>) -> Result<(), Error> {
    unreachable!()
}

/// Set a specific configuration option
#[poise::command(slash_command, subcommands("set_strikes_enabled"))]
async fn set(_ctx: Context<'_>) -> Result<(), Error> {
    unreachable!()
}

macro_rules! get_set {
    ($ident:ident, $literal:literal, $ty:ty) => {
        paste! {
            #[doc = "Get the value of " $ident]
            #[poise::command(slash_command, rename = $literal, ephemeral)]
            async fn [<get_ $ident>](ctx: Context<'_>) -> Result<(), Error> {
                ensure_config_exists(ctx).await?;

                let option = $literal;
                let query = sqlx::query(
                    &format!("SELECT {option} FROM configs WHERE guild_id = ?"),
                )
                .bind(get_current_guild_id(ctx)?)
                .fetch_one(&ctx.data().pool)
                .await?;
                let value: bool = query.try_get($literal)?;

                ctx.say(format!("`{option}` is currently set to `{value}`."))
                    .await?;

                Ok(())
            }

            #[doc = "Set the value of " $ident]
            #[poise::command(slash_command, rename = $literal, ephemeral)]
            async fn [<set_ $ident>](ctx: Context<'_>, value: $ty) -> Result<(), Error> {
                ensure_config_exists(ctx).await?;

                let option = $literal;

                sqlx::query(&format!(
                    "UPDATE configs SET {option} = ? WHERE guild_id = ?"
                ))
                .bind(value)
                .bind(get_current_guild_id(ctx)?)
                .execute(&ctx.data().pool)
                .await?;

                ctx.say(format!("`{option}` has been set to `{value}`."))
                    .await?;

                Ok(())
            }
        }
    };
}

get_set!(strikes_enabled, "strikes_enabled", bool);
