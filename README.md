# Bevy Turn-Based Combat

[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

This project has purpose to help rust game dev to implement a full turn-based combat into their game.

## [Release Demo](https://fabinistere.github.io/bevy_turn-based_combat/)

v0.4

## Assets

Assets are deported on a different cloud to stop wasting git use which is very costfull.
[Download Assets](https://drive.google.com/drive/folders/1VyAxd2Jsbv0EQ3Z_Ye4U7_Cybimk_Wk0?usp=share_link)

## Features

- [x] Phases
  - [x] Roll Initiative
  - [x] Execute in order Actions
- [ ] Mecanics
  - [ ] TODO: CouldHave - Skill Range (depending of the caster's position)
  - [x] Force Respect of the selected skill's TargetOption
  - [ ] TODO: ShouldHave - Decay turn left on Alteration / Effects on alteration
  - [ ] TODO: CouldHave - Select only 4 to 6 skills from your catalogue, for the next fight.
  - [ ] TODO: CouldHave - TierExtra Skills (unlocked bvy job's tree)
- [ ] UI
  - [x] Nice, Smoother and intuitive ui inputs
  - [ ] Fighting Hall
    - [x] Place Fighters corresponding of their tactical position (diamond shape)
    - [x] Update fighters' transform % window's size
    - [ ] TODO: MustHave - Display all basic stats on the Fighting Hall (Hp/Mp under all entities)
    - [x] CouldHave - Display alteration's icon
  - [x] Initiative Vertical Bar
    - [x] Display all action in the Initiative Vertical Bar
  - [ ] Character Sheet
    - [x] Display all skill per entity (when selected)
    - [x] Scrolling Logs
    - [x] Display Job, Title, Stats
    - [x] Display current stuff
    - [ ] TODO: CouldHave - Stuff Tab, Equip from Team's Inventory, Desequip
    - [ ] TODO: CouldHave - Display somewhere else the skill catalogue
    - [ ] TODO: MustHave - Browse among sheets (arrows)

## Example

// DOC

## Notes

- May replace all `if let Ok()/Some()/...` by the secure `match`

## Fun Facts

Dans ce projet, il existe une erreur que je connais:

$\exists e E(e) \wedge SelfAck(e)$
