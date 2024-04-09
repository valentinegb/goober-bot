use std::{
    sync::{
        atomic::{self, AtomicBool, AtomicU64},
        Arc,
    },
    time::Duration,
};

use anyhow::Context as _;
use poise::serenity_prelude::{
    prelude::TypeMapKey, ChannelId, ClientBuilder, FullEvent, GatewayIntents, GuildId, Mentionable,
    User,
};
use rand::{seq::SliceRandom, thread_rng};
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;
use tracing::{debug, error, info};

struct UserData {} // User data, which is stored and accessible in all command invocations

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, UserData, Error>;

struct BoredomTracker;

impl TypeMapKey for BoredomTracker {
    type Value = Arc<AtomicBool>;
}

struct BoredomMessage;

impl TypeMapKey for BoredomMessage {
    type Value = Arc<AtomicU64>;
}

/// ```
/// rp_command!(
///     name: ident,
///     context_menu_name: literal,
///     description: literal,
///     user_description: literal,
///     [ message..: literal ],
/// );
/// ```
macro_rules! rp_command {
    (
        $name:ident,
        $context_menu_name:literal,
        $description:literal,
        $user_description:literal,
        [$($message:literal),+$(,)?]$(,)?
    ) => {
        #[doc = $description]
        #[poise::command(slash_command, context_menu_command = $context_menu_name)]
        async fn $name(
            ctx: Context<'_>,
            #[description = $user_description] user: User,
        ) -> Result<(), Error> {
            let mut data = ctx.serenity_context().data.write().await;

            data.insert::<BoredomTracker>(Arc::new(AtomicBool::new(false)));

            let messages = [
                $(format!(
                    $message,
                    a = ctx.author().mention(),
                    b = user.mention(),
                )),+
            ];
            let picked_message;

            {
                let mut rng = thread_rng();

                picked_message = messages
                    .choose(&mut rng)
                    .ok_or("Failed to choose random message")?;
            }

            ctx.say(picked_message).await?;
            info!("Responded to a command with \"{picked_message}\"");

            Ok(())
        }
    };
}

rp_command!(
    boop,
    "Boop",
    "Boops a being :3c",
    "Your victim >:3",
    [
        "{a} booped {b}!!! <:floofOwO:1226944711768412280>",
        "{b} just got booped by {a}?? <a:afloofLoad:1227015489792905360>",
        "Lmao I just saw {a} boop {b} <:floofLol:1226944692541980692>",
        "Dear {b},\n\nGet booped, nerd. <:floofSmug:1226944728734629970>\n\nSincerely, {a}",
        "{a} booped {b}, I think they're trying to pick a fight <:floofNervous:1226944704541622394>",
    ],
);

rp_command!(
    gnaw,
    "Gnaw Bones",
    "Embrace the bobin within us all and gnaw on one's bones",
    "The subject of today's gnawing",
    [
        "{a} is gnawing on {b}'s bones <:floofNom:1226944708366831637>",
        "{a} craves the bones of {b} <:floofNom:1226944708366831637>",
        "{a} hungers for the bones of a {b} <:floofNom:1226944708366831637>",
        "Hey uh, {b}, did you know there's a {a} gnawing on your bones? <:floofLurk:1226944909446090922>",
    ],
);

rp_command!(
    bite,
    "Bite",
    "Express a wide range of emotions via- your teeth in somebody's skin",
    "The skin-haver in question",
    [
        "D- did {a} just bite {b}?? <:floofOwO:1226944711768412280>",
        "Awww, {a} gave {b} a love bite... I think- actually, it's hard to say <:floofTired:1226944734640078878>",
        "The intrusive thoughts won and now {a}'s biting {b} <:floofMischief:1226944697579077692>",
        "\\*CHOMP\\*\n{a} bit {b} <:floofNom:1226944708366831637>",
    ],
);

rp_command!(
    meow,
    "Meow At",
    "You know what you are",
    "Get their attention",
    [
        "Uhh, {a} just meowed at {b} <:floofWhat:1226944914315804683>",
        "{a} is a furry and they want {b} to know it <:floofMischief:1226944697579077692>",
        "{a} is so silly, they think {b} can understand their meowing <:floofLol:1226944692541980692>",
        "{b}, be afraid... {a} is meowing at you <:floofPeek:1226944911857815594>",
        "{b}, {a} is meowing at you, won't you give them what they want? <:floofPlead:1226944718735151266>",
        "{b}, I have a message for you: \"meow meow meow meow meow meow meow meow\"\n{a} gave it to me <:floofHappy:1226944682815258755>",
        "{a} just *nya*-ed all over the place- {b}, clean this up! <:floofWhat:1226944914315804683>",
        "{b}... sire... I have a message for you, from {a}... \\*ahem\\*... \"meow meow meow, meow meow, meow meow meow meow meow, meow!\"\nI'm just the messenger please don't hurt me <:floofNervous:1226944704541622394>",
    ],
);

rp_command!(
    murder,
    "Murder",
    "MURRRRRDEERRRRRRRRRRR",
    "KILL THEM KILL THEM KILL THEM >:D",
    [
        "{a} crept up behind {b} and murdered them!!! <:floofOwO:1226944711768412280>",
        "{a} just pulled out a bazooka and blew {b} up!?!? <:floofOwO:1226944711768412280>",
        "{a} stared directly into {b}'s eyes and shouted \"POMEGRANATE\", triggering the cognitohazard previously planted in {b}'s brain, killing them instantly <:floofNervous:1226944704541622394>",
        "{a} just went \"BOO\", giving {b} a fatal heart attack <:floofOwO:1226944711768412280>",
        "{a} just went \"OOGA BOOGA\", giving {b} a fatal heart attack <:floofOwO:1226944711768412280>",
        "{a} killed {b} when the lights went out so no one would know it was them... <:floofSmug:1226944728734629970>",
    ],
);

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![boop(), gnaw(), bite(), meow(), murder()],
            event_handler: |ctx, event, _framework, _data| {
                Box::pin(async move {
                    if let FullEvent::Message { new_message } = event {
                        if let Some(referenced_message) = &new_message.referenced_message {
                            let mut write_data = false;

                            // Read data
                            {
                                let data = ctx.data.read().await;

                                if referenced_message.id.get() == data
                                    .get::<BoredomMessage>()
                                    .ok_or("Failed to get BoredomMessage (it may not have a value, which is probably okay)")?
                                    .load(atomic::Ordering::SeqCst)
                                {
                                    let messages = [
                                        "Omg you're alive!!! <:floofBlep:1226944673281609788>",
                                        "\\*gasp\\* contact has been established! <:floofOwO:1226944711768412280>",
                                        "Oh, phew, you're not dead! <:floofTired:1226944734640078878>",
                                        "Yaaaaay friends!!! <:floofBlep:1226944673281609788>",
                                    ];
                                    let picked_message;

                                    {
                                        let mut rng = thread_rng();

                                        picked_message = messages
                                            .choose(&mut rng)
                                            .ok_or("Failed to choose random message")?;
                                    }

                                    new_message.reply_ping(ctx, *picked_message).await?;
                                    info!("Replyed to boredom acknowledgment: {picked_message}");
                                    write_data = true;
                                }
                            }

                            // Write data
                            if write_data {
                                let mut data = ctx.data.write().await;

                                data.insert::<BoredomTracker>(Arc::new(AtomicBool::new(false)));
                                data.remove::<BoredomMessage>();
                            }
                        }
                    }

                    Ok(())
                })
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(
                    ctx,
                    &framework.options().commands,
                    GuildId::new(1225919005362098176),
                )
                .await?;
                info!("Registered commands");

                let mut data = ctx.data.write().await;

                data.insert::<BoredomTracker>(Arc::new(AtomicBool::new(true)));
                info!("Initialized BoredomTracker");

                let bored_ctx = ctx.clone();

                tokio::spawn(async move {
                    loop {
                        // Sleep for 2 days
                        tokio::time::sleep(Duration::from_secs(60 * 60 * 24 * 2)).await;
                        debug!("It's time to check for boredom!");

                        let mut boredom_message_value = None;
                        let mut boredom_tracker_value = None;

                        // Read data
                        {
                            let data = bored_ctx.data.read().await;

                            match data.get::<BoredomTracker>() {
                                Some(boredom_tracker) => {
                                    if boredom_tracker.load(atomic::Ordering::SeqCst) {
                                        debug!("... I'm bored");

                                        let messages = [
                                            "Waaaaa nobody's talking to me <:floofCry:1226944679833112598>",
                                            "Hello? Did you guys die? <:floofOwO:1226944711768412280>",
                                            "Guys... I'm bored... <:floofSad:1226944722908483665>",
                                            "Hi hello I am the engagement inspector, here for your bi-daily engagement inspection and- WOAH WOAH WOAH, these engagement levels are too low!!! You guys gotta start doing fun stuff right now!!!",
                                            "Are you ignoring me??? Nobody's said anything to me in a while... <:floofAngry:1226944671423660133>",
                                        ];
                                        let picked_message;

                                        {
                                            let mut rng = thread_rng();

                                            picked_message = messages.choose(&mut rng);
                                        }

                                        match picked_message {
                                            Some(picked_message) => match ChannelId::new(1226773600258883675)
                                                .say(&bored_ctx, *picked_message)
                                                .await
                                            {
                                                Ok(message) => {
                                                    info!("Sent boredom message: {picked_message}");
                                                    boredom_message_value = Some(Arc::new(AtomicU64::new(message.id.get())));
                                                }
                                                Err(err) => error!("Failed to send bored message: {err}"),
                                            },
                                            None => error!("Failed to choose random message"),
                                        }
                                    } else {
                                        debug!("... I'm not bored!");
                                        boredom_tracker_value = Some(Arc::new(AtomicBool::new(true)));
                                    }
                                }
                                None => error!("Failed to get BoredomTracker"),
                            }
                        }

                        // Write data
                        {
                            let mut data = bored_ctx.data.write().await;

                            if let Some(value) = boredom_message_value {
                                debug!("I'm saving my boredom message");
                                data.insert::<BoredomMessage>(value);
                            }

                            if let Some(value) = boredom_tracker_value {
                                debug!("I'll be bored next time unless I'm interacted with");
                                data.insert::<BoredomTracker>(value);
                            }
                        }
                    }
                });

                Ok(UserData {})
            })
        })
        .build();
    let client = ClientBuilder::new(discord_token, GatewayIntents::non_privileged())
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}
