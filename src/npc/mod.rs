use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use log::info;

use crate::{
    combat::{stats::*, Leader, Recruted, Team},
    constants::{
        character::npc::{
            movement::{ADMIRAL_POSITION, FABICURION_POSITION, HUGO_POSITION, OLF_POSITION},
            *,
        },
        combat::team::*,
    },
    spritesheet::FabienSheet,
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
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(ADMIRAL_STARTING_ANIM),
            texture_atlas: fabien.0.clone(),
            transform: Transform {
                translation: Vec3::from(ADMIRAL_POSITION),
                scale: Vec3::splat(NPC_SCALE),
                ..default()
            },
            ..default()
        })
        .insert(Name::new("NPC Admiral"))
        .insert(NPC)
        .insert(Team(TEAM_MC))
        .insert(Recruted)
        .insert(StatBundle::default());

    // HUGO
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(HUGO_STARTING_ANIM),
            texture_atlas: fabien.0.clone(),
            transform: Transform {
                translation: Vec3::from(HUGO_POSITION),
                scale: Vec3::splat(NPC_SCALE),
                ..default()
            },
            ..default()
        })
        .insert(Name::new("NPC Hugo"))
        .insert(NPC)
        .insert(Team(TEAM_MC))
        .insert(Recruted)
        .insert(StatBundle::default());
}

fn spawn_aggresives_characters(mut commands: Commands, fabien: Res<FabienSheet>) {
    // let olf_dialog_tree = init_tree_flat(String::from(OLF_DIALOG));

    // OLF
    commands
        .spawn(SpriteSheetBundle {
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
        })
        .insert(Name::new("NPC Olf"))
        .insert(NPC)
        .insert(Leader)
        .insert(Team(TEAM_OLF))
        .insert(StatBundle::default());

    // Two FABICURION
    for i in 0..2 {
        let name = "NPC Fabicurion nmb".replace("nmb", &i.to_string());

        commands
            .spawn(SpriteSheetBundle {
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
            })
            .insert(Name::new(name))
            .insert(NPC)
            .insert(Leader)
            .insert(Team(TEAM_OLF))
            .insert(StatBundle::default());
    }
}
