use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_tweening::TweeningPlugin;
use combat::CombatPlugin;
use constants::*;

pub mod combat;
pub mod constants;
mod debug;
pub mod npc;
pub mod spritesheet;
pub mod ui;

use debug::DebugPlugin;
use npc::NPCPlugin;
use spritesheet::FabienPlugin;
use ui::UiPlugin;

fn main() {
    let height = 720.0;

    let mut app = App::new();
    app.insert_resource(ClearColor(CLEAR))
        .insert_resource(Msaa { samples: 1 })
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: height * RESOLUTION,
                        height: height,
                        title: "Dialog".to_string(),
                        resizable: false,
                        ..default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(TweeningPlugin)
        .add_plugin(CombatPlugin)
        .add_plugin(NPCPlugin)
        .add_plugin(UiPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(FabienPlugin)
        .add_startup_system(spawn_camera);

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.top = 50. * TILE_SIZE;
    camera.projection.bottom = -50. * TILE_SIZE;

    camera.projection.left = 50. * TILE_SIZE * RESOLUTION;
    camera.projection.right = -50. * TILE_SIZE * RESOLUTION;

    camera.projection.scaling_mode = ScalingMode::None;

    commands.spawn(camera);
}
