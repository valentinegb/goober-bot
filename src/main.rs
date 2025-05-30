use dotenvy_macro::dotenv;
use poise::serenity_prelude::{Client, GatewayIntents};
use poise_error::anyhow::{self, anyhow};
use tracing::{error, warn};
use tracing_subscriber::{EnvFilter, Layer, layer::SubscriberExt, util::SubscriberInitExt};

async fn try_main() -> anyhow::Result<()> {
    let mut client = Client::builder(dotenv!("DISCORD_TOKEN"), GatewayIntents::empty()).await?;

    client.start_autosharded().await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let registry = tracing_subscriber::registry();

    match tracing_journald::layer() {
        Ok(journald_layer) => registry.with(journald_layer).init(),
        Err(err) => {
            registry
                .with(tracing_subscriber::fmt::layer().with_filter(EnvFilter::from_default_env()))
                .init();
            warn!(
                "Could not add journald layer to registry: {:#}",
                anyhow!(err),
            );
        }
    }

    if let Err(err) = try_main().await {
        error!("A fatal error occurred: {err:#}")
    }
}
