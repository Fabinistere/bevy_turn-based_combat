use bevy::{prelude::*, winit::WinitSettings};

pub mod dialog_player;
pub mod dialog_panel;
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

            .add_event::<dialog_player::ExecuteSkillEvent>()
            .add_event::<dialog_combat::UpdateUnitSelectedEvent>()
            .add_event::<dialog_combat::UpdateUnitTargetedEvent>()

            .add_startup_system(dialog_panel::setup.label(UiLabel::Textures))

            .add_system(dialog_player::button_system.label(UiLabel::Player))
            .add_system(dialog_player::mouse_scroll)

            .add_system(dialog_combat::select_unit_system)
            .add_system(dialog_combat::target_unit_system)
            .add_system(dialog_combat::update_selected_unit)
            .add_system(dialog_combat::update_targeted_unit)
            .add_system(dialog_combat::update_caster_stats_panel)
            .add_system(dialog_combat::update_caster_stats_panel)
            .add_system(dialog_combat::update_target_stats_panel)
            ;
    }
}

#[derive(Component)]
pub struct UiElement;
