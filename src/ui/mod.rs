use bevy::{prelude::*, winit::WinitSettings};

use crate::combat::{run_if_in_target_phase, run_if_in_caster_phase};

pub mod player_interaction;
pub mod combat_panel;
pub mod combat_system;

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

            .add_event::<player_interaction::ExecuteSkillEvent>()
            .add_event::<combat_system::UpdateUnitSelectedEvent>()
            .add_event::<combat_system::UpdateUnitTargetedEvent>()

            .add_startup_system(combat_panel::setup.label(UiLabel::Textures))

            .add_system(player_interaction::button_system.label(UiLabel::Player))
            .add_system(player_interaction::mouse_scroll)
            .add_system(player_interaction::select_unit_by_mouse)
            

            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(run_if_in_caster_phase)
                    .with_system(combat_system::caster_selection)
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(run_if_in_target_phase)
                    .with_system(combat_system::target_selection)
            )
            .add_system(combat_system::target_unit_system)
            .add_system(combat_system::update_selected_unit)
            .add_system(combat_system::update_targeted_unit)
            .add_system(combat_system::update_caster_stats_panel)
            .add_system(combat_system::update_caster_stats_panel)
            .add_system(combat_system::update_target_stats_panel)
            ;
    }
}

#[derive(Component)]
pub struct UiElement;
