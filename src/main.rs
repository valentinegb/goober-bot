use anyhow::Context as _;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents, GuildId, Mentionable, User};
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;

struct Data {} // User data, which is stored and accessible in all command invocations

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Boops a being :3c
#[poise::command(slash_command, context_menu_command = "Boop")]
async fn boop(
    ctx: Context<'_>,
    #[description = "Your victim >:3"] user: User,
) -> Result<(), Error> {
    ctx.say(format!(
        "{} booped {}!!! <:huh:1226261094818123887>",
        ctx.author().mention(),
        user.mention(),
    ))
    .await?;

    Ok(())
}

/// Embrace the bobin within us all and gnaw on one's bones
#[poise::command(slash_command, context_menu_command = "Gnaw Bones")]
async fn gnaw(
    ctx: Context<'_>,
    #[description = "The subject of today's gnawing"] user: User,
) -> Result<(), Error> {
    ctx.say(format!(
        "{} is gnawing on {}'s bones <:devious:1225988465464705096>",
        ctx.author().mention(),
        user.mention(),
    ))
    .await?;

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
            commands: vec![boop(), gnaw()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(
                    ctx,
                    &framework.options().commands,
                    GuildId::new(1225919005362098176),
                )
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
