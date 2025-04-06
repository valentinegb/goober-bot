use shuttle_shared_db::SerdeJsonOperator;
#[cfg(not(debug_assertions))]
use topgg::{Autoposter, autoposter::Serenity};

/// User data, which is stored and accessible in all command invocations
pub struct Data {
    pub op: SerdeJsonOperator,
    #[cfg(not(debug_assertions))]
    pub topgg_client: topgg::Client,
    // The autoposter is moved here so that it is not dropped, which would stop
    // its thread.
    #[cfg(not(debug_assertions))]
    pub _autoposter: Autoposter<Serenity>,
}

pub type Context<'a> = poise::Context<'a, Data, poise_error::anyhow::Error>;
