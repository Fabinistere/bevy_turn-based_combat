use bevy::{prelude::*, winit::WinitSettings};

use crate::{
    characters::FabiensInfos,
    combat::{
        CombatState,
        tactical_position,
    },
};

use self::{combat_system::{ActionHistory, ActionsLogs, LastTurnActionHistory}};

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

            .insert_resource(ActionsLogs(String::from("---------------\nActions Logs:")))
            .insert_resource(ActionHistory(String::from("---------------\nActions:")))
            .insert_resource(LastTurnActionHistory(String::from("---------------\nLast Turn Actions:")))
            .init_resource::<FabiensInfos>()

            .add_event::<combat_system::UpdateUnitSelectedEvent>()
            .add_event::<combat_system::UpdateUnitTargetedEvent>()

            .add_startup_system(combat_panel::setup.in_set(UiLabel::Textures))

            /* -------------------------------------------------------------------------- */
            /*                         --- Player Input Global ---                        */
            /* -------------------------------------------------------------------------- */
            .add_systems(
                (
                    player_interaction::mouse_scroll,
                    player_interaction::select_unit_by_mouse,
                    player_interaction::cancel_last_input,
                ).in_set(UiLabel::Player)
            )
            .add_system(player_interaction::action_button.after(initiative_bar::action_visibility))
            
            /* -------------------------------------------------------------------------- */
            /*                            --- Limited Phase ---                           */
            /* -------------------------------------------------------------------------- */
            
            // .add_systems(
            //     ().run_if(in_initiation_phase)
            // )
            .add_system(
                // always run
                combat_system::update_alterations_status.after(CombatState::AlterationsExecution)
            )
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
                    character_sheet::update_headers,
                    character_sheet::update_caster_stats_panel.after(UiLabel::Player),
                    character_sheet::update_weapon_displayer,
                )
                    .in_set(CombatState::SelectionSkill)
            )
            .add_systems(
                (
                    combat_system::target_selection,
                    combat_system::update_targeted_unit.after(UiLabel::Player),
                    // switch to a new action ----vvv
                    player_interaction::select_skill,
                    player_interaction::end_of_turn_button,
                    character_sheet::update_headers,
                    character_sheet::update_caster_stats_panel.after(UiLabel::Player),
                    character_sheet::update_weapon_displayer,
                )
                    .in_set(CombatState::SelectionTarget)
            )
            // .add_systems(
            //     ().run_if(in_initiative_phase)
            // )
            .add_system(
                // always run
                combat_system::update_alterations_status.after(CombatState::ExecuteSkills)
            )
            // .add_systems(
            //     ().run_if(in_evasive_phase)
            // )
            
            /* -------------------------------------------------------------------------- */
            /*                            -- DEBUG DISPLAYER --                           */
            /* -------------------------------------------------------------------------- */
            .add_systems((
                combat_system::update_combat_phase_displayer
                    .in_set(UiLabel::Display),
                combat_system::current_action_formater
                    .in_set(UiLabel::Display)
                    .after(CombatState::RollInitiative)
                    .before(CombatState::ExecuteSkills),
                combat_system::current_action_displayer
                    .after(combat_system::current_action_formater),
                combat_system::last_action_displayer
                    .after(CombatState::ExecuteSkills),
                combat_system::actions_logs_displayer
                    .after(CombatState::RollInitiative)
                    .after(CombatState::ExecuteSkills),
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
            ))

            /* -------------------------------------------------------------------------- */
            /*                                --- COLOR ---                               */
            /* -------------------------------------------------------------------------- */
            .add_systems(
                (
                    character_sheet::skill_color,
                    player_interaction::button_system,
                )
                    .after(UiLabel::Display)
            )

            /* -------------------------------------------------------------------------- */
            /*                                   Window                                   */
            /* -------------------------------------------------------------------------- */
            .add_systems((
                tactical_position::detect_window_tactical_pos_change,
                tactical_position::update_character_position
                    .after(tactical_position::detect_window_tactical_pos_change)
            ))
            ;
    }
}
