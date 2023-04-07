//! Spawn 5 NPC Entity

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::{
    combat::{
        skills::Skill, stats::*, stuff::Equipements, Alterations, CombatBundle, InCombat, Karma,
        Recruted, Skills, Team,
    },
    constants::{
        character::npc::{
            movement::{ADMIRAL_POSITION, FABICURION_POSITION, HUGO_POSITION, OLF_POSITION},
            *,
        },
        combat::team::*,
    },
    spritesheet::FabienSheet,
    ui::player_interaction::{Clickable, Hoverable, SpriteSize, SPRITE_SIZE},
};

#[derive(Component, Inspectable)]
pub struct NPC;

#[derive(Default)]
pub struct NPCPlugin;

impl Plugin for NPCPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_characters)
            .add_startup_system(spawn_aggresives_characters);
    }
}

// Check in location/temple/mod.rs
// the npc_z_position

fn spawn_characters(mut commands: Commands, fabien: Res<FabienSheet>) {
    // ADMIRAL
    commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(ADMIRAL_STARTING_ANIM),
            texture_atlas: fabien.0.clone(),
            transform: Transform {
                translation: Vec3::from(ADMIRAL_POSITION),
                scale: Vec3::splat(NPC_SCALE),
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
            team: Team(TEAM_MC),
            karma: Karma(100),
            equipements: Equipements {
                weapon: None,
                armor: None,
            },
            skills: Skills(vec![Skill::bam()]),
            alterations: Alterations(vec![]),
            stats: StatBundle::default(),
        },
        // -- UI Related Components --
        Hoverable,
        Clickable,
    ));

    // HUGO
    commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(HUGO_STARTING_ANIM),
            texture_atlas: fabien.0.clone(),
            transform: Transform {
                translation: Vec3::from(HUGO_POSITION),
                scale: Vec3::splat(NPC_SCALE),
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
            team: Team(TEAM_MC),
            karma: Karma(100),
            equipements: Equipements {
                weapon: None,
                armor: None,
            },
            skills: Skills(vec![Skill::bam()]),
            alterations: Alterations(vec![]),
            stats: StatBundle::default(),
        },
        // -- UI Related Components --
        Hoverable,
        Clickable,
    ));
}

fn spawn_aggresives_characters(mut commands: Commands, fabien: Res<FabienSheet>) {
    // OLF
    commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: OLF_STARTING_ANIM,
                flip_x: true,
                ..default()
            },
            texture_atlas: fabien.0.clone(),
            transform: Transform {
                translation: Vec3::from(OLF_POSITION),
                scale: Vec3::splat(NPC_SCALE),
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
            team: Team(TEAM_OLF),
            karma: Karma(-100),
            equipements: Equipements {
                weapon: None,
                armor: None,
            },
            skills: Skills(vec![Skill::bam()]),
            alterations: Alterations(vec![]),
            stats: StatBundle::default(),
        },
        // -- UI Related Components --
        Hoverable,
        Clickable,
    ));

    // Two FABICURION
    for i in 0..2 {
        let name = "NPC Fabicurion nmb".replace("nmb", &i.to_string());

        commands.spawn((
            SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: FABICURION_STARTING_ANIM,
                    flip_x: true,
                    ..default()
                },
                texture_atlas: fabien.0.clone(),
                transform: Transform {
                    translation: Vec3::new(
                        FABICURION_POSITION.0,
                        FABICURION_POSITION.1 + (i * 30) as f32,
                        FABICURION_POSITION.2,
                    ),
                    scale: Vec3::splat(NPC_SCALE),
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
                team: Team(TEAM_OLF),
                karma: Karma(-100),
                equipements: Equipements {
                    weapon: None,
                    armor: None,
                },
                skills: Skills(vec![Skill::bam()]),
                alterations: Alterations(vec![]),
                stats: StatBundle::default(),
            },
            // -- UI Related Components --
            Hoverable,
            Clickable,
        ));
    }
}
