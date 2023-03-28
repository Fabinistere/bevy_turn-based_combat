use bevy::{prelude::*, winit::WinitSettings};

use crate::combat::{
    run_if_in_caster_phase, run_if_in_evasive_phase, run_if_in_executive_phase,
    run_if_in_initiation_phase, run_if_in_initiative_phase, run_if_in_skill_phase,
    run_if_in_target_phase,
};

pub mod character_sheet;
pub mod combat_panel;
pub mod combat_system;
pub mod player_interaction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
enum UiLabel {
    /// everything that handles textures
    Textures,
    /// everything that updates player state
    Player,
    ///
    Display,
}
pub struct UiPlugin;

impl Plugin for UiPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app
            // OPTIMIZE: Only run the app when there is user input. This will significantly reduce CPU/GPU use.
            .insert_resource(WinitSettings::game())

            .add_event::<player_interaction::ExecuteSkillEvent>()
            .add_event::<combat_system::UpdateUnitSelectedEvent>()
            .add_event::<combat_system::UpdateUnitTargetedEvent>()

            .add_startup_system(combat_panel::setup.label(UiLabel::Textures))

            // --- Player Input Global ---
            .add_system(player_interaction::mouse_scroll.label(UiLabel::Player))
            .add_system(player_interaction::select_unit_by_mouse.label(UiLabel::Player))
            .add_system(combat_system::target_random_system)

            // --- Limited Phase ---
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(run_if_in_initiation_phase)
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(run_if_in_caster_phase)
                    .with_system(combat_system::caster_selection)
                    .with_system(player_interaction::end_of_turn_button)
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(run_if_in_skill_phase)
                    .with_system(combat_system::caster_selection)
                    .with_system(character_sheet::select_skill)
                    .with_system(player_interaction::end_of_turn_button)
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(run_if_in_target_phase)
                    .with_system(combat_system::target_selection)
                    .with_system(character_sheet::select_skill)
                    .with_system(player_interaction::end_of_turn_button)
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(run_if_in_initiative_phase)
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(run_if_in_executive_phase)
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(run_if_in_evasive_phase)
            )
            // DONE: Display Actions
            // DONE: Confirm Actions
            // DONE: Roll Initiative
            // DONE: Execute in order Actions

            // --- COLOR ---
            .add_system(player_interaction::button_system)

            // DEBUG -- DISPLAYER --
            .add_system(combat_system::update_selected_unit.after(UiLabel::Player))
            .add_system(combat_system::update_targeted_unit.after(UiLabel::Player))
            .add_system(combat_system::update_combat_phase_displayer)
            .add_system(combat_system::last_action_displayer)

            .add_system(
                character_sheet::update_caster_stats_panel
                    .label(UiLabel::Display)
                    .after(UiLabel::Player)
            )
            .add_system(
                character_sheet::update_target_stats_panel
                    .label(UiLabel::Display)
                    .after(UiLabel::Player)
            )
            ;
    }
}

#[derive(Component)]
pub struct UiElement;
