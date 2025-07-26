use std::{collections::HashMap, env, path::Path};

use serde::Deserialize;

#[derive(Deserialize)]
struct Emoji {
    production_id: u64,
    development_id: u64,
    #[serde(default)]
    animated: bool,
}

fn main() {
    println!("cargo:rerun-if-changed=src/emojis.toml");

    let mut emojis_rs = String::new();
    let emojis: HashMap<String, Emoji> =
        toml::from_slice(include_bytes!("src/emojis.toml")).unwrap();

    for (
        name,
        Emoji {
            production_id,
            development_id,
            animated,
        },
    ) in emojis
    {
        let prefix = if animated { "a" } else { "" };
        let format = if animated { "gif" } else { "webp" };
        let identifier = to_upper_camel_case(&name);

        emojis_rs += &format!(
            "/// ![:{name}:](https://cdn.discordapp.com/emojis/{production_id}.{format}?quality=lossless)\n\
            #[cfg(not(debug_assertions))]\n\
            pub const {identifier}: &str = \"<{prefix}:{name}:{production_id}>\";\n\
            /// ![:{name}:](https://cdn.discordapp.com/emojis/{development_id}.{format}?quality=lossless)\n\
            #[cfg(debug_assertions)]\n\
            pub const {identifier}: &str = \"<{prefix}:{name}:{development_id}>\";\n"
        );
    }

    std::fs::write(
        Path::new(&env::var_os("OUT_DIR").unwrap()).join("emojis.rs"),
        emojis_rs,
    )
    .unwrap();
}

fn to_upper_camel_case(str: &str) -> String {
    let mut result = String::new();

    for char in str.chars() {
        if char.is_uppercase() {
            result += "_";
        }

        result += &char.to_uppercase().to_string();
    }

    result
}
