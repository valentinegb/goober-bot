use anyhow::Context as _;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents, GuildId, Mentionable, User};
use rand::{seq::SliceRandom, thread_rng};
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;
use tracing::info;

struct Data {} // User data, which is stored and accessible in all command invocations

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Boops a being :3c
#[poise::command(slash_command, context_menu_command = "Boop")]
async fn boop(
    ctx: Context<'_>,
    #[description = "Your victim >:3"] user: User,
) -> Result<(), Error> {
    let messages = [
        format!(
            "{} booped {}!!! <:floofOwO:1226944711768412280>",
            ctx.author().mention(),
            user.mention(),
        ),
        format!(
            "{} just got booped by {}?? <:floofLoad:1226944689546989710>",
            user.mention(),
            ctx.author().mention(),
        ),
        format!(
            "Lmao I just saw {} boop {} <:floofLol:1226944692541980692>",
            ctx.author().mention(),
            user.mention(),
        ),
        format!(
            "Dear {},\n\nGet booped, nerd. <:floofSmug:1226944728734629970>\n\nSincerely, {}",
            user.mention(),
            ctx.author().mention(),
        ),
        format!(
            "{} booped {}, I think they're trying to pick a fight <:floofNervous:1226944704541622394>",
            ctx.author().mention(),
            user.mention(),
        ),
    ];
    let picked_message;

    {
        let mut rng = thread_rng();

        picked_message = messages
            .choose(&mut rng)
            .ok_or("Failed to choose random message")?;
    }

    ctx.say(picked_message).await?;
    info!("Responded to /boop with \"{picked_message}\"");

    Ok(())
}

/// Embrace the bobin within us all and gnaw on one's bones
#[poise::command(slash_command, context_menu_command = "Gnaw Bones")]
async fn gnaw(
    ctx: Context<'_>,
    #[description = "The subject of today's gnawing"] user: User,
) -> Result<(), Error> {
    let messages = [
        format!(
            "{} is gnawing on {}'s bones <:floofNom:1226944708366831637>",
            ctx.author().mention(),
            user.mention(),
        ),
        format!(
            "{} craves the bones of {} <:floofNom:1226944708366831637>",
            ctx.author().mention(),
            user.mention(),
        ),
        format!(
            "{} hungers for the bones of a {} <:floofNom:1226944708366831637>",
            ctx.author().mention(),
            user.mention(),
        ),
        format!(
            "Hey uh, {}, did you know there's a {} gnawing on your bones? <:floofLurk:1226944909446090922>",
            user.mention(),
            ctx.author().mention(),
        ),
    ];
    let picked_message;

    {
        let mut rng = thread_rng();

        picked_message = messages
            .choose(&mut rng)
            .ok_or("Failed to choose random message")?;
    }

    ctx.say(picked_message).await?;
    info!("Responded to /gnaw with \"{picked_message}\"");

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
                info!("Registered commands");

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
