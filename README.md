# Goober Bot

[![Rust](https://github.com/valentinegb/goober-bot/actions/workflows/rust.yml/badge.svg)](https://github.com/valentinegb/goober-bot/actions/workflows/rust.yml)

Bot that is also a goober for ~~the [Gooberland Discord server](https://discord.gg/NCd88hxUFc)~~ any server, DM, or group chat!

Join the [Goober Bot Dev](https://discord.gg/7v2aY2YzJU) Discord server to try the bot or [install it](https://discord.com/oauth2/authorize?client_id=1226752321971687537) to a guild or your user now!

<img width="674" alt="Screenshot 2024-04-09 at 9 52 04 AM" src="https://github.com/valentinegb/goober-bot/assets/35977727/57d110bf-97da-4c2e-a78a-fee1eb5e0307">

*Adorable [floof emojis](https://volpeon.ink/emojis/floof/) courtesy of [Volpeon](https://volpeon.ink)!*

## Features

- silly commands (eg. `/boop`, `/bite`, `/gnaw`)
- per-server configuration
- (configurable) robust strike system
- (configurable) `/anon` command
- rock paper scissors
- guild and user installable

## Commands

*Last updated Nov 15, 2024*

### Silly

- `/bap <user>`
- `/bite <user>`
- `/boop <user>`
- `/carry <user>`
- `/gnaw <user>`
- `/hug <user>`
- `/jumpscare <user>`
- `/kiss <user>`
- `/meow <user>`
- `/murder <user>`
- `/pat <user>`
- `/poke <user>`
- `/revive <user>`
- `/slap <user>`
- `/tickle <user>`

### Strikes

- `/strike give <user> [rule] [comment] [expiration]`
- `/strike history [user] [all]`
- `/strike repeal <user> [strike]`

### Config

- `/config list`
- `/config get strikes_enabled`
- `/config get strikes_log_channel`
- `/config get anon_enabled`
- `/config get anon_channel`
- `/config get anon_log_channel`
- `/config set strikes_enabled <value>`
- `/config set strikes_log_channel [value]`
- `/config set anon_enabled <value>`
- `/config set anon_channel [value]`
- `/config set anon_log_channel [value]`

### Other

- `/anon <message>`
- `/rock_paper_scissors <user>`
- `/timestamp [year] [month] [day] [hour] [minute] [second] [timezone] [style]`
- `/updates`
- `/vote` ❤️
