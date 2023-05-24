use bevy::{prelude::*, winit::WinitSettings};

use crate::combat::{
    in_caster_phase,
    in_skill_phase, 
    in_target_phase, 
    // in_evasive_phase, in_executive_phase, in_initiation_phase, in_initiative_phase,
    CombatState,
};

pub mod character_sheet;
pub mod combat_panel;
pub mod combat_system;
pub mod initiative_bar;
pub mod player_interaction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
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

            .add_event::<combat_system::UpdateUnitSelectedEvent>()
            .add_event::<combat_system::UpdateUnitTargetedEvent>()

            .add_startup_system(combat_panel::setup.in_set(UiLabel::Textures))

            // --- Player Input Global ---
            .add_systems((
                player_interaction::mouse_scroll.in_set(UiLabel::Player),
                player_interaction::select_unit_by_mouse.in_set(UiLabel::Player),
                player_interaction::cancel_last_input.in_set(UiLabel::Player),
            ))
            
            // --- Limited Phase ---
            .configure_set(
                CombatState::SelectionCaster
                    .run_if(in_caster_phase)
            )
            .configure_set(
                CombatState::SelectionSkill
                    .run_if(in_skill_phase)
            )
            .configure_set(
                CombatState::SelectionTarget
                    .run_if(in_target_phase)
            )
            
            // .add_systems(
            //     ().run_if(in_initiation_phase)
            // )
            .add_systems(
                (
                    combat_system::caster_selection,
                    combat_system::update_selected_unit.after(UiLabel::Player),
                    player_interaction::end_of_turn_button,
                )
                    .in_set(CombatState::SelectionCaster)
                    // .distributive_run_if(in_caster_phase)
            )
            // in SkillPhase: There is one selected
            .add_systems(
                (
                    combat_system::caster_selection,
                    combat_system::update_selected_unit.after(UiLabel::Player),
                    player_interaction::select_skill,
                    // FIXME: In SelectionSkill, the end_of_turn trigger twice
                    // cancel the current action if imcomplete -----vvv
                    player_interaction::end_of_turn_button,
                )
                    .in_set(CombatState::SelectionSkill)
                    // .in_schedule(CoreSchedule::FixedUpdate)
            )
            .add_systems(
                (
                    combat_system::target_selection,
                    combat_system::update_targeted_unit.after(UiLabel::Player),
                    // switch to a new action ----vvv
                    player_interaction::select_skill,
                    player_interaction::end_of_turn_button,
                )
                    .in_set(CombatState::SelectionTarget)
            )
            // .add_systems(
            //     ().run_if(in_initiative_phase)
            // )
            // .add_systems(
            //     ().run_if(in_executive_phase)
            // )
            // .add_systems(
            //     ().run_if(in_evasive_phase)
            // )
            
            // DEBUG -- DISPLAYER --
            .add_systems((
                combat_system::update_combat_phase_displayer
                    .in_set(UiLabel::Display),
                combat_system::last_action_displayer
                    .in_set(UiLabel::Display)
                    .after(CombatState::RollInitiative)
                    .before(CombatState::ExecuteSkills),
                character_sheet::update_caster_stats_panel
                    .in_set(UiLabel::Display)
                    .after(UiLabel::Player),
                character_sheet::update_target_stats_panel
                    .in_set(UiLabel::Display)
                    .after(UiLabel::Player),
                initiative_bar::action_visibility
                    .in_set(UiLabel::Display)
                    .after(CombatState::SelectionSkill)
                    .after(CombatState::SelectionTarget),
                character_sheet::skill_visibility
                    .in_set(UiLabel::Display)
                    .after(CombatState::SelectionCaster),
                character_sheet::skill_color
                    .after(UiLabel::Display),
            ))

            // --- COLOR ---
            .add_system(player_interaction::button_system)
            ;
    }
}

#[derive(Component)]
pub struct UiElement;
