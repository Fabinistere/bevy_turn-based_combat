//! Spawn 5 NPC Entity

use bevy::prelude::*;

use crate::{
    combat::{
        skills::Skill,
        stuff::{Equipements, Job, WeaponBundle},
        ActionCount, AllAlterationStatuses, CombatBundle, CombatState, InCombat, Karma, Player,
        Recruted, Skills, TacticalPlace, TacticalPosition, Team,
    },
    constants::{
        character::{npc::*, CHAR_SCALE, SPRITE_SIZE},
        combat::{team::*, FIRST_ALLY_ID, FIRST_ENEMY_ID},
    },
    spritesheet::FabienSheet,
    ui::player_interaction::{Clickable, Hoverable, SpriteSize},
};

pub mod ai;

#[derive(Component)]
pub struct NPC;

#[derive(Default)]
pub struct NPCPlugin;

impl Plugin for NPCPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_characters)
            .add_system(ai::ai_decision_making.in_set(CombatState::AIStrategy));
    }
}

// Check in location/temple/mod.rs
// the npc_z_position

/// TODO: Move the spwan player up ?
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

    // Morgan
    commands
        .spawn((
            SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: MORGAN_STARTING_ANIM,
                    flip_x: true,
                    ..default()
                },
                texture_atlas: fabien.0.clone(),
                transform: Transform {
                    scale: Vec3::splat(CHAR_SCALE * 1.),
                    ..default()
                },
                ..default()
            },
            SpriteSize {
                width: SPRITE_SIZE.0,
                height: SPRITE_SIZE.1,
            },
            Name::new("Morgan"),
            Player,
            // -- Combat Components --
            InCombat(FIRST_ALLY_ID),
            Recruted,
            CombatBundle {
                team: Team(Some(TEAM_MC)),
                karma: Karma(200),
                skills: Skills(vec![Skill::bam(), Skill::pass()]),
                equipements: Equipements {
                    weapon: Some(bass),
                    armor: None,
                },
                job: Job::Musician,
                action_count: ActionCount::new(20),
                tactical_position: TacticalPosition::MiddleLine(TacticalPlace::Middle),
                ..Default::default()
            },
            // -- UI Related Components --
            Hoverable,
            Clickable,
        ))
        .with_children(|parent| {
            // Contains all current alterations with their icons
            parent.spawn((
                TransformBundle::default(),
                VisibilityBundle::default(),
                AllAlterationStatuses,
                Name::new("Alterations Status"),
            ));
        });

    // ADMIRAL
    commands
        .spawn((
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
            Name::new("Admiral"),
            NPC,
            // -- Combat Components --
            InCombat(FIRST_ALLY_ID + 1),
            Recruted,
            CombatBundle {
                team: Team(Some(TEAM_MC)),
                karma: Karma(100),
                skills: Skills(vec![
                    Skill::bam(),
                    Skill::gifle(),
                    Skill::diffamation(),
                    Skill::pass(),
                ]),
                job: Job::Musician,
                action_count: ActionCount::new(1),
                tactical_position: TacticalPosition::FrontLine(TacticalPlace::Left),
                ..Default::default()
            },
            // -- UI Related Components --
            Hoverable,
            Clickable,
        ))
        .with_children(|parent| {
            // Contains all current alterations with their icons
            parent.spawn((
                TransformBundle::default(),
                VisibilityBundle::default(),
                AllAlterationStatuses,
                Name::new("Alterations Status"),
            ));
        });

    // HUGO
    commands
        .spawn((
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
            Name::new("Hugo"),
            NPC,
            // -- Combat Components --
            InCombat(FIRST_ALLY_ID + 2),
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
        ))
        .with_children(|parent| {
            // Contains all current alterations with their icons
            parent.spawn((
                TransformBundle::default(),
                VisibilityBundle::default(),
                AllAlterationStatuses,
                Name::new("Alterations Status"),
            ));
        });

    /* -------------------------------------------------------------------------- */
    /*                              ---- Enemies ----                             */
    /* -------------------------------------------------------------------------- */

    // OLF
    commands
        .spawn((
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
            Name::new("Olf"),
            NPC,
            // -- Combat Components --
            InCombat(FIRST_ENEMY_ID),
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
        ))
        .with_children(|parent| {
            // Contains all current alterations with their icons
            parent.spawn((
                TransformBundle::default(),
                VisibilityBundle::default(),
                AllAlterationStatuses,
                Name::new("Alterations Status"),
            ));
        });

    // Two FABICURION
    for i in 0..2 {
        let name = format!("Fabicurion {}", i);

        commands
            .spawn((
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
                InCombat(FIRST_ENEMY_ID + 1 + i),
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
            ))
            .with_children(|parent| {
                // Contains all current alterations with their icons
                parent.spawn((
                    TransformBundle::default(),
                    VisibilityBundle::default(),
                    AllAlterationStatuses,
                    Name::new("Alterations Status"),
                ));
            });
    }
}
