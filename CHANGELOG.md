# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Skill VFX - [0.6](https://github.com/Fabinistere/bevy_turn-based_combat/releases/tag/v0.6) - 2023-07-12

[![v0.6](https://img.shields.io/badge/v0.6-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/Fabinistere/bevy_turn-based_combat/releases/tag/v0.6)](https://github.com/Fabinistere/bevy_turn-based_combat/releases/tag/v0.6)
[![**Full Commits History**](https://img.shields.io/badge/GitHubLog-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/Fabinistere/bevy_turn-based_combat/commits/v0.6)](https://github.com/Fabinistere/bevy_turn-based_combat/commits/v0.6)

### Preview

[Combat Preview](https://github.com/Fabinistere/bevy_turn-based_combat/assets/73140258/536b91f1-6e4a-4e60-8c1d-21e19445676a)

### Added

- Animate Skill before each skill execution
- Interactive Items
  - Ladder to go down into the LogCave
    - New Place/State
    - Click an entity, go up into the CombatWall + select clicked unit
    - Press `CancelInput` key (atm: Esc) to go back to the CombatWall
  - Character Sheet
    - Up to 6 Character Sheets, for allies
      - Can click on it to open corresponding Character Sheet
    - Pack of Scroll
      - Can click on it to open the first enemy's Character Sheet
    - Browse among character sheet of a same team with `left-arrow` and `right-arrow`
- AI Enemy
  - Currently: Random Behavior

### Fixed

- click on several overlapping entities no longer crash

### Optimize

- `ui::player_interaction::select_unit_by_mouse()`, light comparaison first (before long iteration + checking mouse position).
- `combat::mod::update_number_of_fighters()` no longer execute everytime.

## UI Update - [0.5](https://github.com/Fabinistere/bevy_turn-based_combat/releases/tag/v0.5) - 2023-07-04

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

## UI Scene - [0.4](https://github.com/Fabinistere/bevy_turn-based_combat/releases/tag/v0.4) - 2023-05-11

[![v0.4](https://img.shields.io/badge/v0.4-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/Fabinistere/bevy_turn-based_combat/releases/tag/v0.4)](https://github.com/Fabinistere/bevy_turn-based_combat/releases/tag/v0.4)
[**Full Commits History**](https://github.com/Fabinistere/bevy_turn-based_combat/commits/v0.4)

### Preview

<!-- with a gif -->

### Added

### Changed
