// Goober Bot, the Discord bot
// Copyright (C) 2026  Valentine Briese
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

    let emojis: Result<HashMap<String, Emoji>, _> =
        toml::from_slice(include_bytes!("src/emojis.toml"));

    match emojis {
        Err(err) => println!("cargo::error=could not deserialize `emojis.toml`: {err}"),
        Ok(emojis) => {
            let mut emoji_constants = String::new();
            let mut patterns = String::new();
            let mut replace_with = String::new();

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

                emoji_constants += &format!(
                    "/// ![:{name}:](https://cdn.discordapp.com/emojis/{production_id}.{format}?quality=lossless)\n\
                    #[cfg(not(debug_assertions))]\n\
                    pub const {identifier}: &str = \"<{prefix}:{name}:{production_id}>\";\n\
                    /// ![:{name}:](https://cdn.discordapp.com/emojis/{development_id}.{format}?quality=lossless)\n\
                    #[cfg(debug_assertions)]\n\
                    pub const {identifier}: &str = \"<{prefix}:{name}:{development_id}>\";\n"
                );
                patterns += &format!("\n        \"{{{identifier}}}\",");
                replace_with += &format!("\n            {identifier},");
            }

            if let Err(err) = std::fs::write(
                Path::new(
                    &env::var_os("OUT_DIR").expect("build scripts should always have `$OUT_DIR`"),
                )
                .join("emojis.rs"),
                format!(
                    "{emoji_constants}\n\
                    /// Substitutes emoji placeholders in a string with corresponding Discord\n\
                    /// formatted emoji.\n\
                    ///\n\
                    /// # Examples\n\
                    ///\n\
                    /// ```ignore\n\
                    /// assert_eq!(\n\
                    ///     substitute_emojis(\"This is a floof -> {{FLOOF}}\"),\n\
                    ///     \"This is a floof -> <:floof:1263609061539315722>\",\n\
                    /// );\n\
                    /// ```\n\
                    pub fn substitute_emojis(string: &str) -> String {{\n    \
                        aho_corasick::AhoCorasick::new([{patterns}\n    \
                        ])\n    \
                        .unwrap()\n    \
                        .replace_all(\n        \
                            string,\n        \
                            &[{replace_with}\n        \
                            ],\n    \
                        )\n\
                    }}"
                ),
            ) {
                println!("cargo::error=failed to write `emojis.rs`: {err}");
            }
        }
    }
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
