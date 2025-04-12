use poise::serenity_prelude::{self, HttpError, Message, ModelError};
use poise_error::{
    UserError,
    anyhow::{self, Context, anyhow},
};

pub trait LogChannelContextualizable {
    fn contextualize_log_channel_errors(self) -> Result<Message, anyhow::Error>;
}

impl LogChannelContextualizable for Result<Message, serenity_prelude::Error> {
    fn contextualize_log_channel_errors(self) -> Result<Message, anyhow::Error> {
        self.map_err(|err| {
            const MISSING_ACCESS: isize = 50001;

            match &err {
                serenity_prelude::Error::Http(HttpError::UnsuccessfulRequest(response))
                    if response.error.code == MISSING_ACCESS =>
                {
                    anyhow!(UserError(anyhow!(err).context(
                        "unable to access the log channel (may be missing permission)",
                    )))
                }
                serenity_prelude::Error::Model(ModelError::InvalidPermissions {
                    required,
                    present: _,
                }) => {
                    let required = *required;

                    anyhow!(UserError(anyhow!(err).context(format!(
                        "missing required log channel permission(s): {required}",
                    ))))
                }
                _ => anyhow!(err),
            }
        })
        .context("failed to send message in log channel")
    }
}

#[derive(Default)]
pub struct CustomData {
    pub early_access: bool,
}
