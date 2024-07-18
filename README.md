# Goober Bot

<!--- [![Rust](https://github.com/valentinegb/goober-bot/actions/workflows/rust.yml/badge.svg)](https://github.com/valentinegb/goober-bot/actions/workflows/rust.yml) --->

Bot that is also a goober for ~~the [Gooberland Discord server](https://discord.gg/NCd88hxUFc)~~ any server, DM, or group chat!

Join the [Goober Bot Dev](https://discord.gg/7v2aY2YzJU) Discord server to try the bot or [install it](https://discord.com/oauth2/authorize?client_id=1226752321971687537) to a guild or your user now!

<img width="674" alt="Screenshot 2024-04-09 at 9 52 04 AM" src="https://github.com/valentinegb/goober-bot/assets/35977727/57d110bf-97da-4c2e-a78a-fee1eb5e0307">

## Features

- silly commands (eg. `/boop`, `/bite`, `/gnaw`)
- per-server configuration
- (configurable) robust strike system
- (configurable) `/anon` command
- rock paper scissors
- guild and user installable

## Commands

*Last updated Jul 17, 2024*

### Silly

- `/bite <user>`
- `/boop <user>`
- `/gnaw <user>`
- `/kiss <user>`
- `/meow <user>`
- `/murder <user>`
- `/pat <user>`

### Configuration

- `/config list`
- `/config get <option>`
- `/config set <option> <value>`
- `/config unset <option>`

### Strikes

- `/strike give <user> [rule] [comment] [expiration]`
- `/strike history [user] [all]`
- `/strike repeal <user> [strike]`

### Other

- `/anon <message>`
- `/debug <error|delete_config>`
- `/rock_paper_scissors [user]`
