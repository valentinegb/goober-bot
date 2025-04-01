use shuttle_shared_db::SerdeJsonOperator;

/// User data, which is stored and accessible in all command invocations
#[derive(Debug)]
pub struct Data {
    pub op: SerdeJsonOperator,
    #[cfg(not(debug_assertions))]
    pub topgg_client: topgg::Client,
}

pub type Context<'a> = poise::Context<'a, Data, poise_error::anyhow::Error>;
