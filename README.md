# Bevy Turn-Based Combat

[![v0.6.1](https://img.shields.io/badge/v0.6.1-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/Fabinistere/bevy_turn-based_combat/releases/tag/v0.6.1)](https://github.com/Fabinistere/bevy_turn-based_combat/releases/tag/v0.6.1)
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/fabinistere/bevy_turn-based_combat#license)

This project has purpose to help rust game dev to implement a full turn-based combat into their game. I intend to write a devlog.

## [Release Demo](https://fabinistere.github.io/bevy_turn-based_combat/)

[Combat Preview](https://github.com/Fabinistere/bevy_turn-based_combat/assets/73140258/536b91f1-6e4a-4e60-8c1d-21e19445676a)

## Features

- [x] Phases
  - [x] Roll Initiative
  - [x] Execute in order Actions
- [x] Mecanics
  - [x] Force Respect of the selected skill's TargetOption
  - [x] Decay turn left on Alteration / Effects on alteration
- [ ] TODO: MustHave - AI
  - [ ] NPC Behavior
    - [ ] Vision
    - [ ] Strategy
      - [ ] TODO: MustHave - Random
      - [ ] TODO: MustHave - SoloButMemory with history of their allies' actions
      - [ ] TODO: CouldHave - SoloBolo
      - [ ] TODO: PostDemo - HiveMind
- [ ] UI
  - [x] Nice, Smoother and intuitive ui inputs
  - [ ] TODO: CouldHave - [Outline](https://github.com/YoshieraHuang/bevy_outline "or simple outline sprite") while Hover
  - [ ] TODO: MustHave - Visual - Character's Sheet (impl UI Sprites, Base Scroll)
  - [ ] Fighting Hall
    - [x] Place Fighters corresponding of their tactical position (diamond shape)
    - [x] Update fighters' transform % window's size
    - [ ] TODO: MustHave - Display all basic stats on the Fighting Hall (Hp/Mp under all entities)
    - [x] Display alteration's icon
  - [x] Initiative Vertical Bar
    - [x] Display all action in the Initiative Vertical Bar
  - [x] Character Sheet
    - [x] Display all skill per entity (when selected)
    - [x] Scrolling Logs
    - [x] Display Job, Title, Stats
    - [x] Display current stuff
    - [x] Browse among sheets (arrows)

<!-- DOC: Write Devlog -->

## Notes

- FIXME: Select `Bam` or `Swing` instantly into select `Solo` will create two actions `Solo`
- REFACTOR: Team
- May replace all `if let Ok()/Some()/...` by the secure `match`
- PostDemo: <!-- TODO: PostDemo: -->
  - [ ] Mecanics
    - [ ] add in `TargetOption`: ClosestPosition
      - [ ] Skill Range (depending of the caster's position).
      Skill reachability (the closest enemy's line = First Line, even in `MiddleLine`/`BackLine`)
    - [ ] `TierExtra` Skills (unlocked bvy job's tree)
    - [ ] Select only 4 to 6 skills from your catalogue, for the next fight.
    - [ ] `Turn Delay` before a skill to be executed (being queued for )
    - [ ] Reduce skill cost by stuff and level
  - [ ] UI
    - [ ] Character Sheet
      - [ ] Stuff Tab, Equip from Team's Inventory, Desequip
      - [ ] Display somewhere else the skill catalogue
- Won't Have:
  - [x] Multiple Character Sheet Visible on the wall
    - ~~zoomed out CS with all real component~~ (a real mess...)

## Fun Facts

Dans ce projet, il existe une erreur que je connais:

$\exists e E(e) \wedge SelfAck(e)$

## Contribute

Release's format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

This project also follows to [A successful Git branching model](https://nvie.com/posts/a-successful-git-branching-model/).

### Assets deported - Ecological Issue

From now on, all my repertories using music and images that change frequently will have a special folder in our organisation's cloud.
To avoid using the git storage for such maters.
In fact, storing an image in git means that if a single pixel changes, git will completely save the previous image and the next image.
Which turns out to be a complete waste of energy in my case.

To get the assets of the last commit, please download this folder:
[Download Assets](https://drive.google.com/drive/folders/1VyAxd2Jsbv0EQ3Z_Ye4U7_Cybimk_Wk0?usp=share_link)

To find previous assets, they will be indicated for each release.

## License

This project is free, open source and permissively licensed!

All code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

See the very good reasons for including both [here](https://github.com/bevyengine/bevy/issues/2373).
