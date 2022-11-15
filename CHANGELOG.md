# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Simple migrations system
  - migration to new version of triggers
- saving chats and their owner
- option to call commands with {command}@{bot_username}
### Changed
- Reputation triggers
  - change structure
  - triggers are now separated by chat_id
  - triggers now support `is_wildcard` option
- removed message data
- general refactoring
### Fixed
- bug when first reputation is added in chat it was 0 not 1/-1


## [0.2.0] - 2022-06-13

### Added
- settings
  - `bot_id` - loads `TELOXIDE_TOKEN` to env from settings
  - `can_self_rep` - loads can user give rep to himself
  - `can_rep_bot` - loads can user give rep to bot
  - `display_username` loads will username or full name will be displayed
  - `save_history` - if checked it will save history of reputations
  - `disable_multiple_reps` if checked it will disable multiple reputation by same person to same message
- reputation triggers now can be sticker by adding [`file_unique_id`](https://core.telegram.org/bots/api#sticker)
- commands
  - `/help` - shows all commands
  - `/toprep {u32} ` - shows top 10 or top N users ranked by reputation
- data
  - user list so it can be mapped to `UserId`
  - reputation history file where to store all reputation messages
- more unit tests added

### Changed
- Username or full name is displayed based on settings
- Refactored code

### Removed
- unused `to_string` trait function 

## [0.1.0] - 2022-06-06

### Added
- tgrep project