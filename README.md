# Goober Bot

In case you missed it, Goober Bot is no longer in active development. This
branch, which is now the default branch, was created with the intention of
making the bot easier to host for myself and for others. Though easier to host,
it lacks many features the bot had in v1. Polling shows that you probably don't
care about any of those features! But if you do, then the v1 branch is still
available.

## How to host

1. [Install Nix, the package manager](https://nixos.org/download/)
2. Create an application on [Discord's developer dashboard](https://discord.com/developers/applications)
3. Upload emojis to the application \
   The emojis the bot officially uses can be found [here](https://volpeon.ink/emojis/floof/)
4. Replace the IDs in `crates/emoji/src/emojis.toml` with the IDs of the emojis
   you uploaded in step 3 \
   You only need to replace `production_id` if you're not developing the bot
5. Run the following in a shell:
   ```sh
   GOOBER_BOT_DISCORD_TOKEN=<put your token here, remove the angle brackets> nix run
   ```
