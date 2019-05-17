# Change Log
All notable changes to this project will be documented in this file.

## 0.7.8 - 2019-05-17

### Fixes
- New Telegram Bot API optional ForwardFrom user

## 0.7.7 - 2018-11-23

### Features
- Added new export_chat_invite_link request

### Fixes
- Fix stream long polling
- Add current task notifying

## 0.7.6 - 2018-11-19

### Features
- Pub ids for fast accessing
- Make public all elements of `future` mod
- Connector config improvements
- Renamed "configure" into "new"

### Fixes
- Doc fixes
- Fix compilation without default features

## 0.7.4 - 2018-11-19

### Fixes
- Tests fixes
- Sample fixes
- README fixes

## 0.7.3 - 2018-11-10

### Fixes
- Samples fix for new fork

## 0.7.1 - 2018-11-10

### Fixes
- README, docs and tests fixes

## 0.7.0 - 2018-11-09

### Features
- Forked the original [telegra-bot](https://github.com/telegram-rs/telegram-bot) library
- Implemented restrictChatMember method ([#96](https://github.com/telegram-rs/telegram-bot/pull/96))
- SendDocument request implemented ([#105](https://github.com/telegram-rs/telegram-bot/pull/105))
- Configurable getUpdates limit and language_code to User ([#108](https://github.com/telegram-rs/telegram-bot/pull/108))
- Added missing fields for ChatMember ([#109](https://github.com/telegram-rs/telegram-bot/pull/109))
- New version of tokio ([#110](https://github.com/telegram-rs/telegram-bot/pull/110))
- User bot field ([#114](https://github.com/telegram-rs/telegram-bot/pull/114))
- Added allowed_updates selection for getUpdates ([#116](https://github.com/telegram-rs/telegram-bot/pull/116))

## 0.6.1 - 2018-02-17

### Fixes
- Re-export forgotten types ([#85](https://github.com/telegram-rs/telegram-bot/issues/85))

### Features
- pinChatMessage and unpinChatMessage methods
- Bots can now send and receive Live Locations ([#83](https://github.com/telegram-rs/telegram-bot/issues/83))

## 0.6.0 - 2018-01-09

Rewritten from scratch.

## 0.5.0 - 2016-10-21

### Fixed
- Update dependencies.
- Handle unknown messages.

## 0.4.1 - 2016-02-25

### Fixed
- Fix a bug with broken forward messages.

## 0.4.0 - 2016-02-18

### Added
- Supergroups support.
- `ParseMode` structure.

### Changed
- `Integer` type to be an alias to i64 instead of i32 because of supergroups.
- New `parse_mode` parameter in `API::send_message` method.
- `Chat` enum to support supergroups and channels.
- Specified dependencies versions in Cargo.toml.

### Fixed
- Update type of `user_id` field in `Contact` struct
- Handling of replies to a message.

## 0.3.0 - 2015-08-29

## 0.2.0 - 2015-08-10

## 0.1.2 - 2015-07-30

### Changed
- `Api::long_poll` method to take `FnMut` instead of `Fn`.

## 0.1.1 - 2015-07-26

## 0.1.0 - 2015-06-30

- Initial release
