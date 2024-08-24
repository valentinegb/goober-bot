# How to Contribute

Thanks to the care taken to improve the developer experience in this project,
contributing to Goober Bot can actually be quite easy!

## Running Locally

Before you can run the bot locally, you need to create and define a
`Secrets.dev.toml` file in the root of the project. The contents of such a
file should look like this:

```toml
DISCORD_TOKEN = ''
GITHUB_PAT = ''
```

In between the single-quotes for `DISCORD_TOKEN`, you must provide a token for a
bot to be piloted by the code. `GITHUB_PAT` is only used for `/sponsors`, so
unless you intend to use that command with your locally running instance of the
bot, you can just leave it blank.

Next, you need to have Rust, Cargo, and `cargo-shuttle` installed. Follow the
instructions [here](https://www.rust-lang.org/tools/install) to install Rust.
Cargo is usually installed along with Rust. To install `cargo-shuttle`, run the
following command:

```sh
cargo install cargo-shuttle@0.47.0
```

After setting all that up, you're ready to get going! Just run the following to
start the bot:

```sh
cargo shuttle run
```

## Adding Silly Commands

Commands like `/boop`, `/bap`, and `/meow`, are called "silly commands". They
can all be found in `src/commands/silly.rs`. The `silly_command! {}` macro makes
creating silly commands an absolute *breeze*. Check the documentation for this
macro and/or look around the `silly.rs` file for examples.
