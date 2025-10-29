# Goober Bot

In case you missed it, Goober Bot is no longer in active development. This
branch, which is now the default branch, was created with the intention of
making the bot easier to host for myself and for others. Though easier to host,
it lacks many features the bot had in v1. Polling shows that you probably don't
care about any of those features! But if you do, then the v1 branch is still
available.

## How to host

### Prerequisites

- [Git](https://git-scm.com/install/)
- [Nix](https://nixos.org/download/) (Recommended) OR [Rust](https://rust-lang.org/tools/install/)

### Steps

Throughout these steps, replace any angled brackets with what they describe and
remove the angle brackets.

1. Clone the repository \
   If you don't make any changes except the absolutely necessary ones mentioned
   in these steps, just clone like this:
   ```sh
   git clone https://github.com/valentinegb/goober-bot.git
   ```
   However, if you plan on making other changes, you must [fork this repository](https://github.com/valentinegb/goober-bot/fork)
   and clone your fork.
   ```sh
   git clone https://github.com/<your-username>/goober-bot.git
   ```
   You must also publish the changes you make. Run this after making changes:
   ```sh
   git add .
   git commit -m "<Describe your changes here>"
   git push
   ```
2. Navigate into the cloned repository
   ```sh
   cd goober-bot
   ```
3. Create an application on [Discord's developer dashboard](https://discord.com/developers/applications) \
   It doesn't require any privileged intents or any permissions. You don't even
   have to add the `bot` scope when inviting it to a server if you don't want
   to, just `applications.commands`
4. Upload emojis to the application \
   The emojis the bot officially uses can be found [here](https://volpeon.ink/emojis/floof/)
5. Replace the IDs in `crates/emoji/src/emojis.toml` with the IDs of the emojis
   you uploaded in step 4 \
   You only need to replace `production_id` if you're not developing the bot
6. If using Nix, run the following:
   ```sh
   GOOBER_BOT_DISCORD_TOKEN=<your.discord.token> nix run
   ```
   If using Rust, instead run:
   ```sh
   GOOBER_BOT_DISCORD_TOKEN=<your.discord.token> cargo run
   ```
