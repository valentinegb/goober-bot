use poise::{
    Framework, FrameworkOptions,
    samples::register_globally,
    serenity_prelude::{ClientBuilder, GatewayIntents},
};
use poise_error::anyhow;
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
            on_error: poise_error::on_error,
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
