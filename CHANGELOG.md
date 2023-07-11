# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Bevy 0.11](https://bevyengine.org/learn/migration-guides/0.10-0.11/) - [0.5.1] - 2023-07-11

[![v0.5.1](https://img.shields.io/badge/v0.5.1-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/Fabinistere/bevy_turn-based_combat/releases/tag/v0.5.1)](https://github.com/Fabinistere/bevy_turn-based_combat/releases/tag/v0.5.1)
[![**Full Commits History**](https://img.shields.io/badge/GitHubLog-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/Fabinistere/bevy_turn-based_combat/commits/v0.5.1)](https://github.com/Fabinistere/bevy_turn-based_combat/commits/v0.5.1)

[Migration Guide Bevy 0.10 -> 0.11](https://bevyengine.org/learn/migration-guides/0.10-0.11/)
<!-- [Changlog Bevy Rapier 0.21 -> 0.22](https://github.com/dimforge/bevy_rapier/blob/master/CHANGELOG.md#0220-10-july-2023) -->

### Changed

- ECS
  - `in_set(OnUpdate(*))` -> `run_if(in_state(*))`
  - Add the `#[derive(Event)]` macro for events.
  - Allow tuples and single plugins in `add_plugins`, deprecate `add_plugin`
  - [Schedule-First: the new and improved `add_systems`](https://bevyengine.org/learn/migration-guides/0.10-0.11/#schedule-first-the-new-and-improved-add-systems)
- UI
  - Flatten UI Style properties that use Size + remove Size
    - The `size`, `min_size`, `max_size`, and `gap` properties have been replaced by the `width`, `height`, `min_width`, `min_height`, `max_width`, `max_height`, `row_gap`, and `column_gap` properties. Use the new properties instead.
  - [Remove `Val::Undefinded`](https://bevyengine.org/learn/migration-guides/0.10-0.11/#remove-val-undefined)

## UI Update - [0.5] - 2023-07-04

[![v0.5](https://img.shields.io/badge/v0.5-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/Fabinistere/bevy_turn-based_combat/releases/tag/v0.5)](https://github.com/Fabinistere/bevy_turn-based_combat/releases/tag/v0.5)
[**Full Commits History**](https://github.com/Fabinistere/bevy_turn-based_combat/commits/v0.5)

<!-- TODO: Changelog -->
<!-- TODO: proper Release Description -->

### Preview

<!-- with a gif -->

[Playable Demo](https://fabinistere.github.io/bevy_turn-based_combat/) (enemies skills are disable to the second phase of dev: AI)

### Added

- [![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/fabinistere/bevy_turn-based_combat#license)
- Alterations
- Character Sheet layout
- Interactive Mini CharSheets in the wall (and pack of scrolls)
- new inputs
  - browse between allies (arrows)
  - browse between enemies (arrows)

### Changed

### Fixed

## UI Scene - [0.4] - 2023-05-11

[![v0.4](https://img.shields.io/badge/v0.4-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/Fabinistere/bevy_turn-based_combat/releases/tag/v0.4)](https://github.com/Fabinistere/bevy_turn-based_combat/releases/tag/v0.4)
[**Full Commits History**](https://github.com/Fabinistere/bevy_turn-based_combat/commits/v0.4)

### Preview

<!-- with a gif -->

### Added

### Changed
