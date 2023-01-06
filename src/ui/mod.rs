use bevy::{prelude::*, winit::WinitSettings};

pub mod dialog_player;
pub mod dialog_box;
pub mod dialog_combat;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(SystemLabel)]
enum UiLabel {
    /// everything that handles textures
    Textures,
    /// everything that updates player state
    Player,
}
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            // OPTIMIZE: Only run the app when there is user input. This will significantly reduce CPU/GPU use.
            .insert_resource(WinitSettings::game())

            .add_startup_system(dialog_box::setup.label(UiLabel::Textures))

            .add_system(dialog_player::button_system.label(UiLabel::Player))
            .add_system(dialog_player::mouse_scroll)
            ;
    }
}

#[derive(Component)]
pub struct UiElement;
