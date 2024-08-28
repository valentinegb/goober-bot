// Goober Bot, Discord bot
// Copyright (C) 2024  Valentine Briese
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

use std::collections::HashMap;

use charts_rs::{HorizontalBarChart, THEME_DARK};
use chrono::{DateTime, TimeDelta, Utc};
use poise::{command, serenity_prelude::CreateAttachment, CreateReply};

use crate::{persist::load_or_save_default, Context, Error};

const KEY: &str = "analytics";

type Analytics = HashMap<String, Vec<DateTime<Utc>>>;

fn load(ctx: Context<'_>) -> Result<Analytics, Error> {
    let mut analytics: Analytics = load_or_save_default(ctx, KEY)?;

    for invocations in analytics.values_mut() {
        invocations
            .retain(|date_time| Utc::now().signed_duration_since(date_time) <= TimeDelta::days(1));
    }

    ctx.data().persist.save(KEY, &analytics)?;

    Ok(analytics)
}

pub(super) fn increment(ctx: Context<'_>) -> Result<(), Error> {
    let mut analytics = load(ctx)?;
    let invocations = analytics
        .entry(ctx.invoked_command_name().to_string())
        .or_default();

    invocations.push(Utc::now());
    ctx.data().persist.save(KEY, analytics)?;

    Ok(())
}

/// Displays the the usage of commands in the last 24 hours
#[command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel",
    owners_only,
    ephemeral
)]
pub(super) async fn analytics(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let mut analytics: Vec<(_, _)> = load(ctx)?.into_iter().collect();

    analytics.sort_by(|(_, invocations_a), (_, invocations_b)| {
        invocations_b.len().cmp(&invocations_a.len())
    });

    let mut series_data = Vec::new();
    let mut x_axis_data = Vec::new();

    for (command, invocations) in analytics {
        series_data.push(invocations.len() as f32);
        x_axis_data.push(format!("/{command}"));
    }

    let mut chart = HorizontalBarChart::new_with_theme(
        vec![("Invocations", series_data).into()],
        x_axis_data,
        THEME_DARK,
    );

    chart.width *= 1.5;
    chart.height *= 1.5;
    chart.margin = charts_rs::Box {
        left: 10.0,
        top: 5.0,
        right: 25.0,
        bottom: 10.0,
    };
    ctx.send(CreateReply::default().attachment(CreateAttachment::bytes(
        charts_rs::svg_to_png(&chart.svg()?)?,
        "analytics.png",
    )))
    .await?;

    Ok(())
}
