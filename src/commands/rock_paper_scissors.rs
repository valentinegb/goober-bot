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

use anyhow::{bail, Context as _};
use poise::{
    command,
    serenity_prelude::{
        futures::StreamExt, ButtonStyle, CreateActionRow, CreateAllowedMentions, CreateButton,
        CreateInteractionResponse, CreateInteractionResponseFollowup,
        CreateInteractionResponseMessage, Mentionable, ReactionType, UserId,
    },
    CreateReply,
};
use rand::{seq::IteratorRandom, thread_rng};

use crate::{emoji::*, Context, Error};

/// Challenge someone to a game of Rock Paper Scissors
#[command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel",
    required_bot_permissions = "USE_EXTERNAL_EMOJIS"
)]
pub(crate) async fn rock_paper_scissors(
    ctx: Context<'_>,
    #[description = "Person you want to play with"] user: UserId,
) -> Result<(), Error> {
    let bot_id = ctx.framework().bot_id;
    let choose_buttons = vec![
        CreateButton::new("rock")
            .emoji('🪨')
            .label("Rock")
            .style(ButtonStyle::Primary),
        CreateButton::new("paper")
            .emoji('📜')
            .label("Paper")
            .style(ButtonStyle::Primary),
        CreateButton::new("scissors")
            .emoji(ReactionType::Unicode("✂️".to_string()))
            .label("Scissors")
            .style(ButtonStyle::Primary),
    ];
    let disabled_choose_buttons: Vec<CreateButton> = choose_buttons
        .iter()
        .map(|button| button.clone().disabled(true))
        .collect();
    let author = ctx.author();

    if user == bot_id {
        let bot_reply = ctx
            .send(
                CreateReply::default()
                    .content("Okay, I accept! I've chosen which one I'm gonna do, now you choose")
                    .components(vec![CreateActionRow::Buttons(choose_buttons)])
                    .ephemeral(true),
            )
            .await?;
        let mut choose_interaction_stream = bot_reply
            .message()
            .await?
            .await_component_interactions(ctx)
            .stream();

        while let Some(choose_interaction) = choose_interaction_stream.next().await {
            bot_reply
                .edit(
                    ctx,
                    CreateReply::default()
                        .components(vec![CreateActionRow::Buttons(disabled_choose_buttons)]),
                )
                .await?;

            let bot_choice = {
                let mut rng = thread_rng();

                ["rock", "paper", "scissors"]
                    .into_iter()
                    .choose(&mut rng)
                    .context("Expected possible bot choices to not be empty")?
            };
            let outcome_message = if bot_choice == choose_interaction.data.custom_id {
                format!(
                    "{}{} and {}... it's a tie! {FLOOF_OWO}",
                    bot_choice
                        .chars()
                        .nth(0)
                        .context("Expected bot choice to have at least one character")?
                        .to_uppercase(),
                    &bot_choice[1..],
                    choose_interaction.data.custom_id,
                )
            } else {
                match bot_choice {
                    "rock" => match choose_interaction.data.custom_id.as_str() {
                        "paper" => format!(
                            "Paper beats rock and you win! {FLOOF_HAPPY}",
                        ),
                        "scissors" => format!(
                            "Rock beats scissors and I win! {FLOOF_HAPPY}",
                        ),
                        other => bail!("Expected component to have custom ID of either \"paper\" or \"scissors\", got \"{other}\""),
                    },
                    "paper" => match choose_interaction.data.custom_id.as_str() {
                        "rock" => format!(
                            "Paper beats rock and I win! {FLOOF_HAPPY}",
                        ),
                        "scissors" => format!(
                            "Scissors beats paper and you win! {FLOOF_HAPPY}",
                        ),
                        other => bail!("Expected component to have custom ID of either \"rock\" or \"scissors\", got \"{other}\""),
                    },
                    "scissors" => match choose_interaction.data.custom_id.as_str() {
                        "rock" => format!(
                            "Rock beats scissors and you win! {FLOOF_HAPPY}",
                        ),
                        "paper" => format!(
                            "Scissors beats paper and I win! {FLOOF_HAPPY}",
                        ),
                        other => bail!("Expected component to have custom ID of either \"rock\" or \"paper\", got \"{other}\""),
                    },
                    other => bail!("Expected component to have custom ID of either \"rock\", \"paper\", or \"scissors\", got \"{other}\""),
                }
            };

            choose_interaction
                .create_response(
                    ctx,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content(outcome_message)
                            .ephemeral(true),
                    ),
                )
                .await?;

            break;
        }

        return Ok(());
    }

    if user == author.id {
        ctx.send(
            CreateReply::default()
                .content(format!(
                    "You can't play Rock Paper Scissors with yourself, silly {FLOOF_BLEP}"
                ))
                .ephemeral(true),
        )
        .await?;

        return Ok(());
    }

    let accept_buttons = vec![
        CreateButton::new("accept")
            .label("Accept")
            .style(ButtonStyle::Success),
        CreateButton::new("decline")
            .label("Decline")
            .style(ButtonStyle::Danger),
    ];
    let disabled_accept_buttons = accept_buttons
        .iter()
        .map(|button| button.clone().disabled(true))
        .collect();
    let author_mention = author.mention();
    let user_mention = user.mention();
    let reply = ctx
        .send(
            CreateReply::default()
                .content(format!(
                    "{author} has challenged {user} to a game of Rock Paper Scissors! {user}, do you accept?",
                    author = author_mention,
                    user = user_mention,
                ))
                .components(vec![CreateActionRow::Buttons(accept_buttons)])
                .allowed_mentions(CreateAllowedMentions::new().users([user])),
        )
        .await?;
    let mut accept_interaction_stream = reply
        .message()
        .await?
        .await_component_interactions(ctx)
        .stream();

    while let Some(accept_interaction) = accept_interaction_stream.next().await {
        if accept_interaction.user.id != user {
            accept_interaction
                .create_response(
                    ctx,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content(format!(
                                "I'm asking {user_mention}, not you silly! {FLOOF_BLEP}",
                            ))
                            .ephemeral(true),
                    ),
                )
                .await?;

            continue;
        }

        reply
            .edit(
                ctx,
                CreateReply::default()
                    .components(vec![CreateActionRow::Buttons(disabled_accept_buttons)]),
            )
            .await?;

        match accept_interaction.data.custom_id.as_str() {
            "accept" => {
                accept_interaction.create_response(
                    ctx,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content(format!(
                                "{user_mention} accepted! {user_mention}, what would you like to choose?",
                            ))
                            .components(vec![CreateActionRow::Buttons(choose_buttons.clone())])
                    )
                ).await?;

                let mut user_choose_interaction_stream = accept_interaction
                    .get_response(ctx)
                    .await?
                    .await_component_interactions(ctx)
                    .stream();

                while let Some(user_choose_interaction) = user_choose_interaction_stream.next().await {
                    if user_choose_interaction.user.id == author.id {
                        user_choose_interaction
                            .create_response(
                                ctx,
                                CreateInteractionResponse::Message(
                                    CreateInteractionResponseMessage::new()
                                        .content(format!(
                                            "{user_mention} gets to choose first, then you choose! {FLOOF_HAPPY}",
                                        ))
                                        .ephemeral(true),
                                ),
                            )
                            .await?;

                        continue;
                    }

                    if user_choose_interaction.user.id != user {
                        user_choose_interaction
                            .create_response(
                                ctx,
                                CreateInteractionResponse::Message(
                                    CreateInteractionResponseMessage::new()
                                        .content(format!(
                                            "I'm asking {user_mention}, not you silly! {FLOOF_BLEP}",
                                        ))
                                        .ephemeral(true),
                                ),
                            )
                            .await?;

                        continue;
                    }

                    accept_interaction.delete_response(ctx).await?;

                    let author_choose_followup = accept_interaction.create_followup(
                        ctx,
                        CreateInteractionResponseFollowup::new()
                            .content(format!(
                                "{user_mention} has chosen! Now {author_mention}, what would *you* like to choose?",
                            ))
                            .components(vec![CreateActionRow::Buttons(choose_buttons)])
                            .allowed_mentions(CreateAllowedMentions::new().users([author]))
                    ).await?;
                    let mut author_choose_interaction_stream = author_choose_followup
                        .await_component_interactions(ctx)
                        .stream();

                    while let Some(author_choose_interaction) = author_choose_interaction_stream.next().await {
                        if author_choose_interaction.user.id == user {
                            author_choose_interaction
                                .create_response(
                                    ctx,
                                    CreateInteractionResponse::Message(
                                        CreateInteractionResponseMessage::new()
                                            .content(format!(
                                                "You already chose, silly, now it's {author_mention}'s turn {FLOOF_BLEP}",
                                            ))
                                            .ephemeral(true),
                                    ),
                                )
                                .await?;

                            continue;
                        }

                        if author_choose_interaction.user.id != author.id {
                            author_choose_interaction
                                .create_response(
                                    ctx,
                                    CreateInteractionResponse::Message(
                                        CreateInteractionResponseMessage::new()
                                            .content(format!(
                                                "I'm asking {author_mention}, not you silly! {FLOOF_BLEP}",
                                            ))
                                            .ephemeral(true),
                                    ),
                                )
                                .await?;

                            continue;
                        }

                        accept_interaction.delete_followup(ctx, author_choose_followup).await?;

                        let outcome_message = if user_choose_interaction.data.custom_id == author_choose_interaction.data.custom_id {
                            format!(
                                "{}{} and {}... it's a tie! {FLOOF_OWO}",
                                user_choose_interaction
                                    .data
                                    .custom_id
                                    .chars()
                                    .nth(0)
                                    .context("Expected component custom ID to have at least one character")?
                                    .to_uppercase(),
                                &user_choose_interaction
                                    .data
                                    .custom_id[1..],
                                author_choose_interaction
                                    .data
                                    .custom_id,
                            )
                        } else {
                            match user_choose_interaction.data.custom_id.as_str() {
                                "rock" => match author_choose_interaction.data.custom_id.as_str() {
                                    "paper" => format!(
                                        "Paper beats rock and {author_mention} wins! {FLOOF_HAPPY}",
                                    ),
                                    "scissors" => format!(
                                        "Rock beats scissors and {user_mention} wins! {FLOOF_HAPPY}",
                                    ),
                                    other => bail!("Expected component to have custom ID of either \"paper\" or \"scissors\", got \"{other}\""),
                                },
                                "paper" => match author_choose_interaction.data.custom_id.as_str() {
                                    "rock" => format!(
                                        "Paper beats rock and {user_mention} wins! {FLOOF_HAPPY}",
                                    ),
                                    "scissors" => format!(
                                        "Scissors beats paper and {author_mention} wins! {FLOOF_HAPPY}",
                                    ),
                                    other => bail!("Expected component to have custom ID of either \"rock\" or \"scissors\", got \"{other}\""),
                                },
                                "scissors" => match author_choose_interaction.data.custom_id.as_str() {
                                    "rock" => format!(
                                        "Rock beats scissors and {author_mention} wins! {FLOOF_HAPPY}",
                                    ),
                                    "paper" => format!(
                                        "Scissors beats paper and {user_mention} wins! {FLOOF_HAPPY}",
                                    ),
                                    other => bail!("Expected component to have custom ID of either \"rock\" or \"paper\", got \"{other}\""),
                                },
                                other => bail!("Expected component to have custom ID of either \"rock\", \"paper\", or \"scissors\", got \"{other}\""),
                            }
                        };

                        accept_interaction.create_followup(
                            ctx,
                            CreateInteractionResponseFollowup::new()
                                .content(outcome_message),
                        ).await?;

                        break;
                    }

                    break;
                }
            }
            "decline" => accept_interaction.create_response(
                ctx,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content(format!(
                            "{user_mention} declined {FLOOF_SAD}",
                        ))
                        .allowed_mentions(CreateAllowedMentions::new())
                )
            ).await?,
            other => bail!("Expected component to have custom ID of either \"accept\" or \"decline\", got \"{other}\""),
        }

        break;
    }

    Ok(())
}
