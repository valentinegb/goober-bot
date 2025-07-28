use poise::{
    Framework, FrameworkOptions,
    samples::register_globally,
    serenity_prelude::{ClientBuilder, GatewayIntents},
};
use poise_error::{anyhow, dedup_error_chain};
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    if let Err(err) = try_main().await {
        error!("A fatal error occurred: {err:#}");
    }
}

async fn try_main() -> anyhow::Result<()> {
    info!("Starting up...");

    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: Vec::new().into_iter().chain(silly::commands()).collect(),
            on_error: |error| {
                Box::pin(async move {
                    if let Err(mut err) =
                        early_access::try_handle_error_or(error, poise_error::try_handle_error)
                            .await
                    {
                        dedup_error_chain(&mut err);
                        error!("Failed to handle error: {err:#}");
                    }
                })
            },
            ..Default::default()
        })
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                info!("Online as {}", ready.user.name);
                register_globally(ctx, &framework.options().commands).await?;
                info!("Finished setup");

                Ok(())
            })
        })
        .build();
    let mut client = ClientBuilder::new(env!("DISCORD_TOKEN"), GatewayIntents::empty())
        .framework(framework)
        .await?;

    client.start().await?;

    Ok(())
}
