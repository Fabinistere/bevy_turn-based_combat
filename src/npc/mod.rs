//! Spawn 5 NPC Entity

use bevy::prelude::*;

use crate::{
    combat::{
        skills::Skill,
        stuff::{Equipements, Job, SkillTiers, WeaponBundle, WeaponType},
        CombatBundle, InCombat, Karma, Recruted, Skills, Team,
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

#[derive(Default, Component, Reflect)]
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
    // ---- Equipements ----
    // TODO: Equip Stuff from Inventory (+ spawn this weapon in the team's inventory)

    let bass = commands
        .spawn(WeaponBundle {
            name: Name::new("Bass"),
            weapon_type: WeaponType::Instrument,
            skill_tiers: SkillTiers {
                tier_2: vec![Skill::swing(), Skill::solo()],
                tier_1: vec![Skill::melody()],
                tier_0: vec![],
            },
            // TODO: ownership
            // equipement: Equipement(None),
            ..Default::default()
        })
        .id();

    // ---- Characters ----

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
            team: Team(Some(TEAM_MC)),
            karma: Karma(100),
            skills: Skills(vec![Skill::bam(), Skill::gifle(), Skill::pass()]),
            equipements: Equipements {
                weapon: Some(bass),
                armor: None,
            },
            job: Job::Musician,
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
            team: Team(Some(TEAM_MC)),
            karma: Karma(100),
            skills: Skills(vec![Skill::bam(), Skill::implosion(), Skill::pass()]),
            ..Default::default()
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
            sprite: TextureAtlasSprite::new(OLF_STARTING_ANIM),
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
            team: Team(Some(TEAM_OLF)),
            karma: Karma(-100),
            skills: Skills(vec![Skill::implosion(), Skill::bam(), Skill::pass()]),
            equipements: Equipements {
                weapon: None,
                armor: None,
            },
            ..Default::default()
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
                sprite: TextureAtlasSprite::new(FABICURION_STARTING_ANIM),
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
                team: Team(Some(TEAM_OLF)),
                karma: Karma(-100),
                skills: Skills(vec![Skill::bam(), Skill::pass()]),
                equipements: Equipements {
                    weapon: None,
                    armor: None,
                },
                ..Default::default()
            },
            // -- UI Related Components --
            Hoverable,
            Clickable,
        ));
    }
}
