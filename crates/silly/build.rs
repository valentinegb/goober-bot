use std::{collections::HashMap, env, path::Path};

use serde::Deserialize;

#[derive(Deserialize)]
struct Silly {
    description: String,
    target_description: String,
    self_response: String,
    bot_response: String,
    other_responses: Vec<String>,
    #[serde(default)]
    early_access: bool,
}

fn main() {
    println!("cargo:rerun-if-changed=src/sillies.toml");

    let sillies: Result<HashMap<String, Silly>, _> =
        toml::from_slice(include_bytes!("src/sillies.toml"));

    match sillies {
        Err(err) => println!("cargo::error=could not deserialize `sillies.toml`: {err}"),
        Ok(sillies) => {
            let mut silly_commands = String::new();
            let mut commands_fn = String::from(
                "pub fn commands() -> Vec<poise::Command<(), anyhow::Error>> {\n    \
                    vec![\n",
            );

            for (
                name,
                Silly {
                    description,
                    target_description,
                    self_response,
                    bot_response,
                    other_responses,
                    early_access,
                },
            ) in sillies
            {
                let early_access_check = if early_access {
                    ",\ncheck = has_early_access"
                } else {
                    ""
                };
                let other_responses: Vec<String> = other_responses
                    .into_iter()
                    .map(|response| format!("{response:?},\n"))
                    .collect();
                let other_responses = other_responses.join("");

                silly_commands += &format!(
                    "/// {description}\n\
                    #[command(\n    \
                        slash_command,\n    \
                        install_context = \"Guild | User\",\n    \
                        interaction_context = \"Guild | BotDm | PrivateChannel\"{early_access_check}\n\
                    )]\n\
                    async fn {name}(\n    \
                        ctx: poise_error::Context<'_>,\n    \
                        #[description = {target_description:?}]\n    \
                        #[rename = \"user\"]\n    \
                        target: UserId,\n\
                    ) -> anyhow::Result<()> {{\n    \
                        const RESPONSES: Responses = Responses {{\n        \
                            self_response: {self_response:?},\n        \
                            bot_response: {bot_response:?},\n        \
                            other_responses: &[\n            \
                                {other_responses}\n        \
                            ],\n    \
                        }};\n\
                    \n    \
                        RESPONSES.respond(ctx, target).await\n\
                    }}\n\n"
                );
                commands_fn += &format!("        {name}(),\n");
            }

            if let Err(err) = std::fs::write(
                Path::new(
                    &env::var_os("OUT_DIR").expect("build scripts should always have `$OUT_DIR`"),
                )
                .join("sillies.rs"),
                format!("{silly_commands}\n{commands_fn}    ]\n}}"),
            ) {
                println!("cargo::error=failed to write `sillies.rs`: {err}");
            }
        }
    }
}
