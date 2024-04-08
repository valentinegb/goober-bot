use std::env;

use anyhow::Context as _;
use poise::serenity_prelude::{self, ClientBuilder, GatewayIntents, GuildId};
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Responds with "world!"
#[poise::command(slash_command)]
async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("world!").await?;
    Ok(())
}

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![hello()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                // Register commands globally when deployed, otherwise only in
                // test server
                let test_server = GuildId::new(1226752252593573898);

                if let Ok(shuttle) = env::var("SHUTTLE") {
                    if shuttle == "true" {
                        poise::builtins::register_globally(ctx, &framework.options().commands)
                            .await?;
                        test_server.set_commands(ctx, vec![]).await?;

                        return Ok(Data {});
                    }
                }

                serenity_prelude::Command::set_global_commands(ctx, vec![]).await?;
                poise::builtins::register_in_guild(ctx, &framework.options().commands, test_server)
                    .await?;

                Ok(Data {})
            })
        })
        .build();

    let client = ClientBuilder::new(discord_token, GatewayIntents::non_privileged())
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}
