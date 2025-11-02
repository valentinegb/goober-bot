// Goober Bot, the Discord bot
// Copyright (C) 2025  Valentine Briese
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
// You may contact me via electronic mail at <valentinegb@icloud.com>.

use poise::{
    CreateReply, FrameworkError,
    serenity_prelude::{
        CreateActionRow, CreateButton, CreateEmbed, SkuId, colours::branding::BLURPLE,
    },
};
use poise_error::anyhow;
use thiserror::Error;

const SUBSCRIPTION_SKU_ID: SkuId = SkuId::new(1351234259867926671);

#[derive(Error, Debug)]
pub enum Error {
    #[error("command author is not a subscriber")]
    NotSubscriber,
    #[error("attempted to check for subscription from a prefix command")]
    PrefixContext,
}

pub async fn is_subscriber<U>(
    #[allow(unused_variables)] // Used in release build but not debug build
    ctx: poise::Context<'_, U, anyhow::Error>,
) -> Result<bool, anyhow::Error> {
    #[cfg(not(debug_assertions))]
    {
        use poise::serenity_prelude::Timestamp;
        use poise_error::anyhow::anyhow;

        match ctx {
            poise::Context::Application(application_context) => application_context
                .interaction
                .entitlements
                .iter()
                .any(|entitlement| {
                    let now = Timestamp::now();

                    entitlement.sku_id == SUBSCRIPTION_SKU_ID
                        && entitlement
                            .starts_at
                            .is_none_or(|starts_at| starts_at <= now)
                        && entitlement.ends_at.is_none_or(|ends_at| ends_at > now)
                })
                .then_some(true)
                .ok_or(anyhow!(Error::NotSubscriber)),
            poise::Context::Prefix(_prefix_context) => Err(anyhow!(Error::PrefixContext)),
        }
    }

    #[cfg(debug_assertions)]
    Ok(true)
}

pub async fn try_handle_error_or<U, F>(
    error: FrameworkError<'_, U, anyhow::Error>,
    try_handle_other: F,
) -> Result<(), anyhow::Error>
where
    F: AsyncFn(FrameworkError<'_, U, anyhow::Error>) -> Result<(), anyhow::Error>,
{
    match error {
        FrameworkError::CommandCheckFailed {
            error: Some(error),
            ctx,
            ..
        } if matches!(error.downcast_ref(), Some(Error::NotSubscriber)) => {
            ctx.send(
                CreateReply::default()
                    .embed(
                        CreateEmbed::default()
                            .title("Goober Bot Plus Required")
                            .description(
                                "This command requires you to be a member of \
                                **Goober Bot Plus**.\n\
                                \n\
                                Subscribing to **Goober Bot Plus** gives you \
                                access to a handful of extra commands no one \
                                else gets while also supporting me, the \
                                developer (hi!). I appreciate it more than you \
                                know!",
                            )
                            .color(BLURPLE),
                    )
                    .components(vec![CreateActionRow::Buttons(vec![
                        CreateButton::new_premium(SUBSCRIPTION_SKU_ID),
                    ])])
                    .ephemeral(true),
            )
            .await?;
        }
        other => return try_handle_other(other).await,
    }

    Ok(())
}
