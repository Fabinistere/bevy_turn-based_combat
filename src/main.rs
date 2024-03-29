#![feature(trivial_bounds)]
// ^^--- allow reflect on Vec<T>

use bevy::{prelude::*, window::WindowResolution};
// use bevy_ecs::schedule::{LogLevel, ScheduleBuildSettings};
use bevy_tweening::TweeningPlugin;
use combat::CombatPlugin;
use constants::{CLEAR, HEIGHT, RESOLUTION};

pub mod characters;
pub mod combat;
pub mod constants;
mod debug;
pub mod fx;
pub mod spritesheet;
pub mod ui;

use characters::npcs::NPCPlugin;
use debug::DebugPlugin;
use fx::FXPlugin;
use spritesheet::FabienPlugin;
use ui::UiPlugin;

#[rustfmt::skip]
fn main() {
    let mut app = App::new();
    app .insert_resource(ClearColor(CLEAR))
        .insert_resource(Msaa::Off)
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(
                            HEIGHT * RESOLUTION,
                            HEIGHT
                        ),
                        title: "Turn-Based Combat".to_string(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            DebugPlugin,
            TweeningPlugin,
            FabienPlugin,
            CombatPlugin,
            NPCPlugin,
            UiPlugin,
            FXPlugin
        ))
        // .edit_schedule(CoreSchedule::Main, |schedule| {
        //     schedule.set_build_settings(ScheduleBuildSettings {
        //         ambiguity_detection: LogLevel::Warn,
        //         ..default()
        //     });
        // })
        .add_systems(Startup, spawn_camera);

    app.run();

    // TODO: update the assets folder in the cloud
    // TODO: publish the demo on the web
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scale = 0.1;

    commands.spawn(camera);
}
