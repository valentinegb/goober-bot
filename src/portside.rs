use poise::serenity_prelude::{
    futures::StreamExt, CacheHttp, ChannelId, Color, CreateEmbed, CreateEmbedAuthor,
    CreateEmbedFooter, CreateMessage, EditMessage, Http, Reaction,
};
use tracing::warn;

use crate::Error;

pub(super) async fn check_portside_reactions(
    ctx: impl CacheHttp + AsRef<Http>,
    reaction: &Reaction,
) -> Result<(), Error> {
    let message = reaction.message(&ctx).await?;
    let tomato_reactions = message
        .reactions
        .into_iter()
        .find(|message_rection| message_rection.reaction_type.unicode_eq("🍅"));

    if let Some(tomato_reactions) = tomato_reactions {
        let tomato_reactions_count = tomato_reactions.count;

        if tomato_reactions_count > 1 {
            let portside_channel = ChannelId::new(1229587493100327003);
            let mut portside_messages = portside_channel.messages_iter(&ctx).boxed();
            let portside_message_content =
                format!("**🍅 {tomato_reactions_count} <#{}>**", message.channel_id,);
            let message_id_string = message.id.to_string();

            while let Some(portside_message) = portside_messages.next().await {
                if let Ok(mut portside_message) = portside_message {
                    match portside_message.embeds.get(0) {
                        Some(embed) => match &embed.footer {
                            Some(footer) => {
                                if footer.text == message_id_string {
                                    portside_message
                                        .edit(
                                            &ctx,
                                            EditMessage::new().content(portside_message_content),
                                        )
                                        .await?;

                                    return Ok(());
                                }
                            }
                            None => warn!("Embed in #portside is missing its footer"),
                        },
                        None => {
                            warn!(
                                "Message in #portside does not have embed, ID: {}",
                                portside_message.id,
                            );
                        }
                    }
                }
            }

            portside_channel
                .send_message(
                    &ctx,
                    CreateMessage::new()
                        .content(portside_message_content)
                        .embed(
                            CreateEmbed::new()
                                .color(Color::RED)
                                .author(
                                    CreateEmbedAuthor::new(&message.author.name).icon_url(
                                        message
                                            .author
                                            .avatar_url()
                                            .unwrap_or(message.author.default_avatar_url()),
                                    ),
                                )
                                .description(message.content)
                                .footer(CreateEmbedFooter::new(message_id_string))
                                .timestamp(message.timestamp),
                        ),
                )
                .await?;
        }
    }

    Ok(())
}
