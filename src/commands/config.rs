use anyhow::{bail, Context as _};
use poise::{
    serenity_prelude::{Color, CreateEmbed, Timestamp},
    CreateReply,
};
use sqlx::{Column, Row, TypeInfo};

use crate::{Context, Error};

#[poise::command(
    slash_command,
    subcommands("list"),
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
    let row = match sqlx::query("SELECT * FROM configs WHERE guild_id = ?")
        .bind(ctx.guild_id().context("Not in guild")?.get())
        .fetch_one(&ctx.data().pool)
        .await
    {
        Ok(query) => query,
        Err(err) => match err {
            sqlx::Error::RowNotFound => {
                sqlx::query("INSERT INTO configs (guild_id) VALUES (?) RETURNING *")
                    .bind(ctx.guild_id().context("Not in guild")?.get())
                    .fetch_one(&ctx.data().pool)
                    .await?
            }
            other => bail!(other),
        },
    };

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
