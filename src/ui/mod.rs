use bevy::{prelude::*, utils::HashMap, winit::WinitSettings};

use crate::{
    characters::FabiensInfos,
    combat::{CombatState, GameState, tactical_position},
    ui::{
        combat_panel::{CharacterSheetResources, CombatWallResources},
        combat_system::{ActionHistory, ActionsLogs, LastTurnActionHistory},
    }
};

use self::combat_panel::CharacterSheetAssociations;

pub mod character_sheet;
pub mod combat_panel;
pub mod combat_system;
pub mod initiative_bar;
pub mod player_interaction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
enum UiLabel {
    // /// everything that handles textures
    // Textures,
    /// everything that updates player state
    Player,
    ///
    Display,
}
pub struct UiPlugin;

impl Plugin for UiPlugin {
    /// # Note
    /// 
    /// `.in_set(OnUpdate(GameState::CombatWall))` is implied by any
    /// `.in_set(CombatState::...)`
    /// 
    /// REFACTOR: Add everywhere it's not implied `.in_set(OnUpdate(GameState::CombatWall))`  
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app
            // OPTIMIZE: Only run the app when there is user input. This will significantly reduce CPU/GPU use.
            .insert_resource(WinitSettings::game())

            .insert_resource(ActionsLogs(String::from("---------------\nActions Logs:")))
            .insert_resource(ActionHistory(String::from("---------------\nActions:")))
            .insert_resource(LastTurnActionHistory(String::from("---------------\nLast Turn Actions:")))
            // CharacterSheetAssociations will be initialized in the `ui::combat_panel::set_up()`
            .insert_resource(CharacterSheetAssociations(HashMap::default()))
            .init_resource::<FabiensInfos>()
            .init_resource::<CharacterSheetResources>()
            .init_resource::<CombatWallResources>()

            .add_event::<combat_system::UpdateUnitSelectedEvent>()
            .add_event::<combat_system::UpdateUnitTargetedEvent>()
            .add_event::<character_sheet::InitializeCharacterSheetHeaders>()
            .add_event::<character_sheet::InitializeCharacterSheetSkills>()
            .add_event::<character_sheet::FocusCharacterSheet>()
            .add_event::<character_sheet::UnFocusCharacterSheet>()

            /* -------------------------------------------------------------------------- */
            /*                         --- Player Input Global ---                        */
            /* -------------------------------------------------------------------------- */
            .add_systems(
                (
                    character_sheet::ally_character_sheet_interact,
                    player_interaction::mouse_scroll,
                    player_interaction::select_unit_by_mouse,
                    player_interaction::cancel_last_input,
                ).in_set(UiLabel::Player)
            )
            .add_system(player_interaction::action_button.after(initiative_bar::action_visibility))

            /* -------------------------------------------------------------------------- */
            /*                                   States                                   */
            /* -------------------------------------------------------------------------- */
                    
            // .after(characters::npcs::spawn_characters)
            .add_system(combat_panel::setup.in_schedule(OnEnter(GameState::CombatWall)))
            
            /* -------------------------------------------------------------------------- */
            /*                            --- Limited Phase ---                           */
            /* -------------------------------------------------------------------------- */
            
            .add_systems(
                (
                    character_sheet::initialisation,
                )
                    .in_set(CombatState::Initialisation)
            )
            .add_system(
                combat_system::update_alterations_status.after(CombatState::AlterationsExecution)
            )
            .add_systems(
                (
                    combat_system::caster_selection,
                    combat_system::update_selected_unit.after(UiLabel::Player),
                    player_interaction::end_of_turn_button,
                )
                    .in_set(CombatState::SelectionCaster)
            )
            // in SkillPhase: There is one selected
            .add_systems(
                (
                    combat_system::caster_selection,
                    combat_system::update_selected_unit.after(UiLabel::Player),
                    player_interaction::select_skill,
                    // FIXME: In SelectionSkill, the end_of_turn trigger twice, CombatStates -> derive States could fix that but having so much States might not be so cool
                    // cancel the current action if imcomplete -----vvv
                    player_interaction::end_of_turn_button,
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
                )
                    .in_set(CombatState::SelectionTarget)
            )
            .add_system(
                combat_system::update_alterations_status.after(CombatState::ExecuteSkills)
            )
            // .add_systems(
            //     ().run_if(in_evasive_phase)
            // )
            
            /* -------------------------------------------------------------------------- */
            /*                            -- DEBUG DISPLAYER --                           */
            /* -------------------------------------------------------------------------- */
            .add_systems(
                (
                    combat_system::update_combat_phase_displayer,
                    combat_system::current_action_formater
                        .after(CombatState::RollInitiative)
                        .before(CombatState::ExecuteSkills),
                    character_sheet::update_target_stats_panel
                        .after(UiLabel::Player),
                    initiative_bar::action_visibility
                        .after(CombatState::SelectionSkill)
                        .after(CombatState::SelectionTarget),
                )
                    .in_set(UiLabel::Display)
            )
            .add_systems((
                combat_system::current_action_displayer
                    .after(combat_system::current_action_formater),
                combat_system::last_action_displayer
                    .after(CombatState::ExecuteSkills),
                combat_system::actions_logs_displayer
                    .after(CombatState::RollInitiative)
                    .after(CombatState::ExecuteSkills),
            ))

            /* -------------------------------------------------------------------------- */
            /*                              Characters' Sheet                             */
            /* -------------------------------------------------------------------------- */

            .add_systems((
                // --- Handle Init Event ---
                character_sheet::update_headers,
                character_sheet::skill_visibility,
                // -------------------------
                // TODO: CouldHave - Hover Stats to reveal precise calculus (with base, alt, stuffs etc)
                character_sheet::update_caster_stats_panel,
                character_sheet::update_weapon_displayer,
                // OPTIMIZE: Restrain execution to the Initialisation (there wont be any change in the total of ally)
                character_sheet::ally_character_sheet_visibility,
            ))
             // TOTEST: any limitations ?
            .add_systems(
                (
                    character_sheet::focus_character_sheet,
                    character_sheet::unfocus_character_sheet,
                )
                    .after(UiLabel::Player)
            )

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
