//! Spawn 5 NPC Entity

use bevy::prelude::*;

use crate::{
    combat::{
        skills::Skill,
        stuff::{Equipements, Job, SkillTiers, WeaponBundle},
        ActionCount, CombatBundle, InCombat, Karma, Recruted, Skills, TacticalPlace,
        TacticalPosition, Team,
    },
    constants::{character::npc::*, combat::team::*},
    spritesheet::FabienSheet,
    ui::player_interaction::{Clickable, Hoverable, SpriteSize, SPRITE_SIZE},
};

#[derive(Default, Component, Reflect)]
pub struct NPC;

#[derive(Default)]
pub struct NPCPlugin;

impl Plugin for NPCPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_characters);
    }
}

// Check in location/temple/mod.rs
// the npc_z_position

fn spawn_characters(mut commands: Commands, fabien: Res<FabienSheet>) {
    /* -------------------------------------------------------------------------- */
    /*                            ---- Equipements ----                           */
    /* -------------------------------------------------------------------------- */
    // TODO: feat - Equip Stuff from Inventory (+ spawn this weapon in the team's inventory)
    // TODO: feat - Team's Inventory

    let bass = commands.spawn(WeaponBundle::bass()).id();
    let smallmouth_bass = commands.spawn(WeaponBundle::smallmouth_bass()).id();

    /* -------------------------------------------------------------------------- */
    /*                            ---- Characters ----                            */
    /* -------------------------------------------------------------------------- */

    // ADMIRAL
    commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: ADMIRAL_STARTING_ANIM,
                flip_x: true,
                ..default()
            },
            texture_atlas: fabien.0.clone(),
            transform: Transform {
                scale: Vec3::splat(NPC_SCALE * 1.),
                ..default()
            },
            ..default()
        },
        SpriteSize {
            width: SPRITE_SIZE.0,
            height: SPRITE_SIZE.1,
        },
        Name::new("NPC Admiral"),
        NPC,
        // -- Combat Components --
        InCombat,
        Recruted,
        CombatBundle {
            team: Team(Some(TEAM_MC)),
            karma: Karma(100),
            skills: Skills(vec![Skill::bam(), Skill::gifle(), Skill::pass()]),
            equipements: Equipements {
                weapon: Some(bass),
                armor: None,
            },
            job: Job::Musician,
            action_count: ActionCount::new(20),
            tactical_position: TacticalPosition::FrontLine(TacticalPlace::Left),
            ..Default::default()
        },
        // -- UI Related Components --
        Hoverable,
        Clickable,
    ));

    // HUGO
    commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: HUGO_STARTING_ANIM,
                flip_x: true,
                ..default()
            },
            texture_atlas: fabien.0.clone(),
            transform: Transform {
                scale: Vec3::splat(NPC_SCALE * 1.),
                ..default()
            },
            ..default()
        },
        SpriteSize {
            width: SPRITE_SIZE.0,
            height: SPRITE_SIZE.1,
        },
        Name::new("NPC Hugo"),
        NPC,
        // -- Combat Components --
        InCombat,
        Recruted,
        CombatBundle {
            team: Team(Some(TEAM_MC)),
            karma: Karma(100),
            skills: Skills(vec![Skill::bam(), Skill::implosion(), Skill::pass()]),
            tactical_position: TacticalPosition::FrontLine(TacticalPlace::Middle),
            ..Default::default()
        },
        // -- UI Related Components --
        Hoverable,
        Clickable,
    ));

    /* -------------------------------------------------------------------------- */
    /*                              ---- Enemies ----                             */
    /* -------------------------------------------------------------------------- */

    // OLF
    commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(OLF_STARTING_ANIM),
            texture_atlas: fabien.0.clone(),
            transform: Transform {
                scale: Vec3::splat(NPC_SCALE * 1.),
                ..default()
            },
            ..default()
        },
        SpriteSize {
            width: SPRITE_SIZE.0,
            height: SPRITE_SIZE.1,
        },
        Name::new("NPC Olf"),
        NPC,
        // -- Combat Components --
        InCombat,
        CombatBundle {
            team: Team(Some(TEAM_OLF)),
            karma: Karma(-100),
            skills: Skills(vec![Skill::implosion(), Skill::bam(), Skill::pass()]),
            equipements: Equipements {
                weapon: None,
                armor: None,
            },
            tactical_position: TacticalPosition::FrontLine(TacticalPlace::Middle),
            ..Default::default()
        },
        // -- UI Related Components --
        Hoverable,
        Clickable,
    ));

    // Two FABICURION
    for i in 0..2 {
        let name = format!("NPC Fabicurion {}", i);

        commands.spawn((
            SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(FABICURION_STARTING_ANIM),
                texture_atlas: fabien.0.clone(),
                transform: Transform {
                    scale: Vec3::splat(NPC_SCALE * 1.),
                    ..default()
                },
                ..default()
            },
            SpriteSize {
                width: SPRITE_SIZE.0,
                height: SPRITE_SIZE.1,
            },
            Name::new(name),
            NPC,
            // -- Combat Components --
            InCombat,
            CombatBundle {
                team: Team(Some(TEAM_OLF)),
                karma: Karma(-100),
                skills: Skills(vec![Skill::bam(), Skill::pass()]),
                equipements: Equipements {
                    weapon: Some(smallmouth_bass),
                    armor: None,
                },
                job: Job::Fabicurion,
                tactical_position: if i == 0 {
                    TacticalPosition::MiddleLine(TacticalPlace::Right)
                } else {
                    TacticalPosition::MiddleLine(TacticalPlace::Left)
                },
                ..Default::default()
            },
            // -- UI Related Components --
            Hoverable,
            Clickable,
        ));
    }

    // commands
    // .spawn((
    //     NodeBundle {
    //         style: Style {
    //             size: Size::width(Val::Percent(100.0)),
    //             justify_content: JustifyContent::SpaceBetween,
    //             flex_direction: FlexDirection::Row,
    //             ..default()
    //         },
    //         ..default()
    //     },
    //     Name::new("NPC Scene"),
    // ))
    // .with_children(|parent| {
    //     // Fighting Hall - Where the npcs are
    //     parent
    //         .spawn((
    //             NodeBundle {
    //                 style: Style {
    //                     size: Size::width(Val::Percent(56.)),
    //                     flex_direction: FlexDirection::Column,
    //                     ..default()
    //                 },
    //                 background_color: Color::rgba(0., 0., 0., 0.).into(),
    //                 ..default()
    //             },
    //             Name::new("NPCs - Fighting Hall"),
    //         ))
    //         .with_children(|parent| {});
}
