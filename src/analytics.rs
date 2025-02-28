// Goober Bot, Discord bot
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

use std::collections::HashSet;

use charts_rs::{HorizontalBarChart, THEME_DARK};
use chrono::{TimeDelta, Utc};
use diesel::{prelude::*, upsert::excluded};
use diesel_async::RunQueryDsl;
use poise::{CreateReply, command, serenity_prelude::CreateAttachment};

use crate::{Context, models::Analytics, schema};

async fn load(ctx: Context<'_>) -> Result<Vec<Analytics>, poise_error::anyhow::Error> {
    let mut conn = ctx.data().pool.get().await?;
    let mut analytics = schema::analytics::table
        .select(Analytics::as_select())
        .load(&mut conn)
        .await?;
    let commands: HashSet<_> = ctx
        .framework()
        .options
        .commands
        .iter()
        .map(|command| command.identifying_name.clone())
        .collect();

    // Ensure all commands are in analytics
    for command in &commands {
        if !analytics
            .iter()
            .any(|analytics| analytics.command == *command)
        {
            analytics.push(Analytics {
                command: command.clone(),
                invocations: Vec::new(),
            });
        }
    }

    // Remove commands from analytics that no longer exist
    analytics.retain(|analytics| commands.contains(&analytics.command));

    // Remove command invocations which were more than a day ago
    for analytics in analytics.iter_mut() {
        analytics.invocations.retain(|date_time| {
            Utc::now().signed_duration_since(date_time.and_utc()) <= TimeDelta::days(1)
        });
    }

    diesel::insert_into(schema::analytics::table)
        .values(&analytics)
        .on_conflict(schema::analytics::columns::command)
        .do_update()
        .set(schema::analytics::columns::command.eq(excluded(schema::analytics::columns::command)))
        .execute(&mut conn)
        .await?;

    Ok(analytics)
}

pub(super) async fn increment(ctx: Context<'_>) -> Result<(), poise_error::anyhow::Error> {
    let mut analytics = load(ctx).await?;
    let root_command = ctx
        .parent_commands()
        .first()
        .map_or(ctx.command().identifying_name.clone(), |root_command| {
            root_command.identifying_name.clone()
        });
    let invocations = match analytics
        .iter_mut()
        .find(|analytics| analytics.command == root_command)
    {
        Some(analytics) => &mut analytics.invocations,
        None => {
            analytics.push(Analytics {
                command: root_command.clone(),
                invocations: Vec::new(),
            });

            &mut analytics.last_mut().unwrap().invocations
        }
    };

    invocations.push(Utc::now().naive_utc());

    let mut conn = ctx.data().pool.get().await?;

    diesel::update(
        schema::analytics::table.filter(schema::analytics::columns::command.eq(root_command)),
    )
    .set(schema::analytics::columns::invocations.eq(invocations.clone()))
    .execute(&mut conn)
    .await?;

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
pub(super) async fn analytics(ctx: Context<'_>) -> Result<(), poise_error::anyhow::Error> {
    ctx.defer_ephemeral().await?;

    let mut analytics: Vec<Analytics> = load(ctx).await?;

    analytics.sort_by(|analytics_a, analytics_b| {
        analytics_b
            .invocations
            .len()
            .cmp(&analytics_a.invocations.len())
    });

    let mut series_data = Vec::new();
    let mut x_axis_data = Vec::new();

    for analytics in analytics {
        series_data.push(analytics.invocations.len() as f32);
        x_axis_data.push(format!("/{}", analytics.command));
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
    chart.title_text = "Command Invocations in the Last 24 Hours".to_string();
    chart.legend_margin = Some(charts_rs::Box {
        top: 25.0,
        ..Default::default()
    });
    ctx.send(CreateReply::default().attachment(CreateAttachment::bytes(
        charts_rs::svg_to_png(&chart.svg()?)?,
        "analytics.png",
    )))
    .await?;

    Ok(())
}
