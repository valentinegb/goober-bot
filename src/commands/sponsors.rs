use anyhow::{bail, Context as _};
use poise::command;
use tracing::info;

use crate::{emoji::*, Context, Error};

/// Lists current GitHub sponsors ❤️
#[command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel",
    required_bot_permissions = "USE_EXTERNAL_EMOJIS",
    ephemeral
)]
pub(crate) async fn sponsors(ctx: Context<'_>) -> Result<(), Error> {
    let tier = "ST_kwDOAiT5_84ABlqV";
    let response: serde_json::Value = octocrab::instance()
        .graphql(
            &serde_json::json!({
                "query": format!("{{ viewer {{ sponsors(first: 100, tierId: \"{tier}\") {{ nodes {{ ... on User {{ login name }} ... on Organization {{ login name }} }} }} }} }}"),
            }),
        )
        .await?;

    info!("`/sponsors` GraphQL response: {response:#?}");

    if let serde_json::Value::Array(nodes) = response
        .get("data")
        .context("Expected `data` field to exist")?
        .get("viewer")
        .context("Expected `viewer` field to exist")?
        .get("sponsors")
        .context("Expected `sponsors` field to exist")?
        .get("nodes")
        .context("Expected `nodes` field to exist")?
    {
        let sponsors_page = "https://github.com/sponsors/valentinegb";

        if nodes.is_empty() {
            ctx.say(format!("Woah, hey, new command! Hmm... I don't have any sponsors to show yet, but you could be the first!\nYour name could [be here]({sponsors_page}) for **$5/month**, a little goes a long way! {FLOOF_MUG}")).await?;

            return Ok(());
        }

        let mut message = format!(
            "This project is made possible by these absolutely *lovely* sponsors {FLOOF_HEART}\n"
        );

        for object in nodes {
            if let serde_json::Value::Object(object) = object {
                if let serde_json::Value::String(login) = object
                    .get("login")
                    .context("Expected `login` field to be `Value::String`")?
                {
                    if let serde_json::Value::String(name) = object
                        .get("name")
                        .context("Expected `name` field to be `Value::String`")?
                    {
                        message += &format!("\n- {name} ([{login}](https://github.com/{login}))");
                    }
                }
            } else {
                bail!("Expected `object` to be `Value::Object`");
            }
        }

        message += &format!("\n\nYour name could [be here too]({sponsors_page}) for **$5/month**, a little goes a long way! {FLOOF_MUG}");
        ctx.say(message).await?;
    } else {
        bail!("Expected `nodes` field to be `Value::Array`");
    }

    Ok(())
}
