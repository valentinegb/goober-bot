mod commands;

use anyhow::Context as _;
use poise::{
    serenity_prelude::{ClientBuilder, GatewayIntents},
    Framework, FrameworkOptions,
};
use shuttle_runtime::{CustomError, SecretStore};
use shuttle_serenity::ShuttleSerenity;
use sqlx::MySqlPool;

/// User data, which is stored and accessible in all command invocations
struct Data {
    pool: MySqlPool,
}

type Error = anyhow::Error;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Responds with "world!"
#[poise::command(slash_command)]
async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("world!").await?;

    Ok(())
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
    #[shuttle_aws_rds::MariaDB] pool: MySqlPool,
) -> ShuttleSerenity {
    sqlx::migrate!()
        .run(&pool)
        .await
        .map_err(CustomError::new)?;

    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;
    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: vec![commands::config()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                Ok(Data { pool })
            })
        })
        .build();
    let client = ClientBuilder::new(discord_token, GatewayIntents::non_privileged())
        .framework(framework)
        .await
        .map_err(CustomError::new)?;

    Ok(client.into())
}
