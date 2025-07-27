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

    let mut sillies_rs = String::new();
    let sillies: Result<HashMap<String, Silly>, _> =
        toml::from_slice(include_bytes!("src/sillies.toml"));

    match sillies {
        Err(err) => println!("cargo::error=could not deserialize `sillies.toml`: {err}"),
        Ok(sillies) => {
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
                // TODO
            }
        }
    }

    if let Err(err) = std::fs::write(
        Path::new(&env::var_os("OUT_DIR").expect("build scripts should always have `$OUT_DIR`"))
            .join("sillies.rs"),
        sillies_rs,
    ) {
        println!("cargo::error=failed to write `sillies.rs`: {err}");
    }
}
