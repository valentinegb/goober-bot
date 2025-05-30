use dotenvy_macro::dotenv;
use poise::{
    FrameworkOptions,
    serenity_prelude::{Client, GatewayIntents},
};
use poise_error::anyhow::{self, anyhow};
use tracing::{error, info, warn};
use tracing_subscriber::{EnvFilter, Layer, layer::SubscriberExt, util::SubscriberInitExt};

async fn try_main() -> anyhow::Result<()> {
    let framework = poise::Framework::builder()
        .options(FrameworkOptions {
            on_error: poise_error::on_error,
            ..Default::default()
        })
        .setup(|_ctx, ready, _framework| {
            Box::pin(async move {
                info!("Logged in as {}", ready.user.display_name());

                Ok(())
            })
        })
        .build();
    let mut client = Client::builder(dotenv!("DISCORD_TOKEN"), GatewayIntents::empty())
        .framework(framework)
        .await?;

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

    info!(
        "Running {} v{} ({} build)",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        },
    );

    if let Err(err) = try_main().await {
        error!("A fatal error occurred: {err:#}")
    }
}
