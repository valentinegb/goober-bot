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
2. Navigate into the cloned repository
   ```sh
   cd goober-bot
   ```
3. You must publish the changes you make, if you make any besides the absolutely
   necessary changes describes in these instructions. Run this after making
   changes:
   ```sh
   git add .
   git commit -m "<Describe your changes here>"
   git push
   ```
4. Create an application on [Discord's developer dashboard](https://discord.com/developers/applications) \
   It doesn't require any privileged intents or any permissions. You don't even
   have to add the `bot` scope when inviting it to a server if you don't want
   to, just `applications.commands`
5. Upload emojis to the application \
   The emojis the bot officially uses can be found [here](https://volpeon.ink/emojis/floof/)
6. Replace the IDs in `crates/emoji/src/emojis.toml` with the IDs of the emojis
   you uploaded in step 4 \
   You only need to replace `production_id` if you're not developing the bot
7. If using Nix, run the following:
   ```sh
   GOOBER_BOT_DISCORD_TOKEN=<your.discord.token> nix run
   ```
   If using Rust, instead run:
   ```sh
   GOOBER_BOT_DISCORD_TOKEN=<your.discord.token> cargo run --release
   ```
   If you are using [NixOS](https://nixos.org) with
   [flakes](https://wiki.nixos.org/wiki/Flakes), you can use Goober Bot's NixOS
   module. Add this repository as an input like so:
   ```nix
   {
     inputs = {
       nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
       goober-bot = {
         url = "github:valentinegb/goober-bot";
         inputs.nixpkgs.follows = "nixpkgs";
       };
     };
     outputs =
       {
         self,
         nixpkgs,
         goober-bot
       }:
       {
         nixosConfigurations = {
           my-config = nixpkgs.lib.nixosSystem {
             modules = [
               goober-bot.nixosModules.goober-bot
               ./configuration.nix
             ];
           };
         };
       };
   }
   ```
   Then add the following to your configuration:
   ```nix
   { ... }:
   {
     # ...
     services.goober-bot = {
       enable = true;
       token = "<your.discord.token>";
     };
     # ...
   }
   ```
   Using NixOS is the way I officially run Goober Bot myself.
