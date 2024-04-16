use poise::serenity_prelude::{
    futures::StreamExt, CacheHttp, ChannelId, Color, CreateEmbed, CreateEmbedAuthor,
    CreateEmbedFooter, CreateMessage, EditMessage, Http, Reaction,
};
use tracing::{error, warn};

use crate::Error;

const MINIMUM_TOMATO_REACTIONS: u64 = 3;

pub(super) async fn check_portside_reactions(
    ctx: impl CacheHttp + AsRef<Http>,
    reaction: &Reaction,
) -> Result<(), Error> {
    if reaction.emoji.unicode_eq("🍅") {
        let reaction_message = reaction.message(&ctx).await?;
        let reaction_message_id = reaction.message_id.to_string();
        let reaction_message_link = reaction_message.link();
        let tomato_reaction = reaction_message
            .reactions
            .into_iter()
            .find(|message_reaction| message_reaction.reaction_type.unicode_eq("🍅"));
        let tomato_reaction_count = match tomato_reaction {
            Some(tomato_reaction) => tomato_reaction.count,
            None => 0,
        };
        let portside_message_content =
            format!("**🍅 {tomato_reaction_count} <#{}>**", reaction.channel_id);

        // check if message has corresponding embed in #portside

        let portside = ChannelId::new(1229587493100327003);
        let mut portside_message = portside.messages_iter(&ctx).boxed();

        while let Some(portside_message) = portside_message.next().await {
            match portside_message {
                Ok(mut portside_message) => match portside_message.embeds.get(0) {
                    Some(portside_message_embed) => match &portside_message_embed.footer {
                        Some(portside_message_embed_footer) => {
                            if portside_message_embed_footer.text == reaction_message_id {
                                // if it does, check if reaction count meets minimum
                                if tomato_reaction_count >= MINIMUM_TOMATO_REACTIONS {
                                    // if it does, edit #portside message content
                                    portside_message
                                        .edit(
                                            &ctx,
                                            EditMessage::new().content(portside_message_content),
                                        )
                                        .await?;
                                } else {
                                    // if not, delete #portside message
                                    portside_message.delete(&ctx).await?;
                                }

                                return Ok(());
                            }
                        }
                        None => warn!("Embed in #portside is missing a footer"),
                    },
                    None => warn!("Message in #portside doesn't have an embed"),
                },
                Err(err) => error!("Failed to get a message from #portside: {err}"),
            }
        }

        // if not, check if reaction count meets minimum
        if tomato_reaction_count >= MINIMUM_TOMATO_REACTIONS {
            // if it does, send new message in #portside

            let mut embed_builder = CreateEmbed::new()
                .color(Color::RED)
                .author(
                    CreateEmbedAuthor::new(
                        reaction_message
                            .author
                            .global_name
                            .as_ref()
                            .unwrap_or(&reaction_message.author.name),
                    )
                    .icon_url(reaction_message.author.face()),
                )
                .description(reaction_message.content)
                .field(
                    "",
                    format!("[Jump to Message]({reaction_message_link})"),
                    false,
                )
                .footer(CreateEmbedFooter::new(reaction_message_id))
                .timestamp(reaction_message.timestamp);

            if !reaction_message.attachments.is_empty() {
                for attachment in reaction_message.attachments {
                    if let Some(content_type) = attachment.content_type {
                        if content_type.starts_with("image") {
                            embed_builder = embed_builder.image(attachment.url);
                        }
                    }
                }
            }

            portside
                .send_message(
                    &ctx,
                    CreateMessage::new()
                        .content(portside_message_content)
                        .embed(embed_builder),
                )
                .await?;
        }

        // if not, do nothing
    }

    Ok(())
}
