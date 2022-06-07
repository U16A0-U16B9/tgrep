# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- settings
  - `bot_id` - loads `TELOXIDE_TOKEN` to env from settings
  - `can_self_rep` - loads can user give rep to himself
  - `can_rep_bot` - loads can user give rep to bot
  - `display_username` loads will username or full name will be displayed
- reputation triggers now can be sticker by adding [`file_unique_id`](https://core.telegram.org/bots/api#sticker)

### Changed
- Username or full name is displayed based on settings

## [0.1.0] - 2022-06-06

### Added
- tgrep project