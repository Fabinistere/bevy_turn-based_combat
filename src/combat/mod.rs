//! Combat Implementation
//!
//! Handle
//!   - Combat Initialisation
//!   - Comabt System / Phases
//!     - Stand On
//!     - Open HUD
//!       - Display potential npc's catchphrase (*opening*)
//!       - Display Answers Choices
//!     - Select Approach in the HUD
//!       - talk
//!         - Initialize dialogue
//!       - fight
//!
//!         ```mermaid
//!         graph
//!             Observation-->ManageStuff;
//!             ManageStuff-->Observation;
//!             Observation-->Skills;
//!             Skills-->Observation;
//!             Skills-->Target;
//!             Target-->Skills;
//!             Target-->RollInitiative;
//!             RollInitiative-->Target;
//!             RollInitiative-->ExecuteSkills-->RollInitiative;
//!             ExecuteSkills-->Observation;
//!         ```
//!
//!     - Reward-s (gift or loot)
//!   - Combat Evasion (quit)

use std::{cmp::Ordering, fmt};

use bevy::prelude::*;
// use bevy_inspector_egui::prelude::*;

use crate::constants::combat::BASE_ACTION_COUNT;

use self::{
    alterations::Alteration, phases::observation, skills::Skill, stats::StatBundle,
    stuff::{Equipements, JobsMasteries, Job},
};

pub mod alteration_list;
pub mod alterations;
pub mod item_list;
pub mod phases;
pub mod skill_list;
pub mod skills;
pub mod stats;
pub mod stuff;
pub mod tactical_position;

/// Just help to create a ordered system in the app builder
#[derive(Default, SystemSet, PartialEq, Eq, Hash, Clone, Debug, Reflect)]
pub enum CombatState {
    /// DOC: what the freak is this phase
    Initiation,
    AlterationsExecution,
    /// Observation:
    /// 
    /// - ManageStuff
    /// - Watch Knows infos/techs from enemies
    /// - Watch yours
    Observation,
    #[default]
    SelectionCaster,
    SelectionSkill,
    SelectionTarget,
    RollInitiative,
    ExecuteSkills,

    // ShowExecution,
    Evasion,
}

impl fmt::Display for CombatState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CombatState::Initiation => write!(f, "Initiation"),
            CombatState::AlterationsExecution => write!(f, "AlterationsExecution"),
            CombatState::Observation => write!(f, "Observation"),
            CombatState::SelectionCaster => write!(f, "SelectionCaster"),
            CombatState::SelectionSkill => write!(f, "SelectionSkill"),
            CombatState::SelectionTarget => write!(f, "SelectionTarget"),
            CombatState::RollInitiative => write!(f, "RollInitiative"),
            CombatState::ExecuteSkills => write!(f, "ExecuteSkills"),
            CombatState::Evasion => write!(f, "Evasion"),
        }
    }
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            // .insert_resource(
            //     CombatPanel {
            //         phase: CombatState::SelectionCaster,
            //         history: vec![],
            //     }
            // )
            
            .add_event::<phases::TransitionPhaseEvent>()
            .add_event::<skills::ExecuteSkillEvent>()
            // .add_event::<alterations::ExecuteAlterationEvent>()
            .add_event::<tactical_position::UpdateCharacterPositionEvent>()

            .init_resource::<JobsMasteries>()

            .add_startup_system(stuff::spawn_stuff)
            
            .add_system(
                observation
                    // .run_if(in_observation_phase)
                    .in_set(CombatState::Observation)
            )
            .add_system(
                phases::execute_alteration
                    .run_if(in_alteration_phase)
                    .in_set(CombatState::AlterationsExecution)
            )
            .add_system(
                phases::roll_initiative
                // FixedTimestep::step(FIXED_TIME_STEP as f64)
                .run_if(in_initiative_phase)
                .in_set(CombatState::RollInitiative)
            )
            .add_system(
                phases::execution_phase
                .run_if(in_executive_phase)
                .in_set(CombatState::ExecuteSkills)
            )
            .add_system(
                skills::execute_skill
                .run_if(in_executive_phase)
                .after(CombatState::ExecuteSkills)
            )
            .add_system(phases::phase_transition)

            // -- UI ? --

            // .add_system(tactical_position::detect_window_tactical_pos_change)
            .add_system(
                tactical_position::update_character_position
                // .after(tactical_position::detect_window_tactical_pos_change)
            )
            // .add_systems(
            //     (show_hp, show_mana)
            //         .run_if(pressed_h)
            //         .in_base_set(CoreSet::PostUpdate)
            // )
            ;
    }
}

/* -------------------------------------------------------------------------- */
/*                           -- Combat Components --                          */
/* -------------------------------------------------------------------------- */

#[derive(Bundle)]
pub struct CombatBundle {
    pub karma: Karma,
    pub team: Team,
    pub job: Job,
    pub alterations: Alterations,
    pub skills: Skills,
    pub equipements: Equipements,
    pub action_count: ActionCount,
    pub tactical_position: TacticalPosition,

    #[bundle]
    pub stats: StatBundle,
}

impl Default for CombatBundle {
    fn default() -> Self {
        CombatBundle {
            karma: Karma(0),
            team: Team(None),
            job: Job::default(),
            alterations: Alterations(Vec::new()),
            skills: Skills(Vec::new()),
            equipements: Equipements { weapon: None, armor: None },
            action_count: ActionCount::default(),
            tactical_position: TacticalPosition::default(),
            stats: StatBundle::default()
        }
    }
}

#[derive(Component, Default)]
pub struct Karma(pub i32);

#[derive(Component, Reflect)]
pub struct ActionCount {
    pub current: usize,
    /// Number of action given each new turn
    pub base: usize,
}

impl ActionCount {
    pub fn new(base: usize) -> Self {
        ActionCount { current: base, base }
    }
}

impl Default for ActionCount {
    fn default() -> Self {
        ActionCount { current: BASE_ACTION_COUNT, base: BASE_ACTION_COUNT }
    }
}

/// The team an entity is assigned to.
/// 
/// `None` being Neutral
#[derive(Copy, Clone, PartialEq, Eq, Component, Deref, DerefMut)]
pub struct Team(pub Option<i32>);

/// Ongoing alterations, Debuff or Buff
#[derive(Default, Component, Deref, DerefMut)]
pub struct Alterations(pub Vec<Alteration>);

/// Basic/Natural skills own by the entity  
#[derive(Component, Deref, DerefMut)]
pub struct Skills(pub Vec<Skill>);

#[derive(Component)]
pub struct InCombat;

#[derive(Clone, Copy, Component)]
pub struct Leader;

/// The player can recruted some friendly npc
/// Can be called, TeamPlayer
#[derive(Copy, Clone, PartialEq, Eq, Component)]
pub struct Recruted;

#[derive(Copy, Clone, PartialEq, Eq, Component)]
pub struct Player;

/* -------------------------------------------------------------------------- */
/*                         -- Position in the Group --                        */
/* -------------------------------------------------------------------------- */

#[derive(Default, Reflect, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum TacticalPlace {
    #[default]
    Left,
    Middle,
    Right,
}

#[derive(Component, Reflect, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum TacticalPosition {
    FrontLine(TacticalPlace),
    MiddleLine(TacticalPlace),
    BackLine(TacticalPlace),
}

impl Default for TacticalPosition {
    fn default() -> Self {
        TacticalPosition::FrontLine(TacticalPlace::default())
    }
}

/* -------------------------------------------------------------------------- */
/*                         -- Combat Core Operation --                        */
/* -------------------------------------------------------------------------- */

/// REFACTOR: Resource ?
#[derive(Default, Component)]
pub struct CombatPanel {
    pub phase: CombatState,
    pub history: Vec<Action>,
}

#[derive(Debug, Clone)]
pub struct Action {
    pub caster: Entity,
    pub skill: Skill,
    /// Optional only to allow selecting skill before the target
    pub targets: Option<Vec<Entity>>,
    /// From caster + skill calculus
    ///
    /// Default: -1
    pub initiative: i32,
}

// impl fmt::Display for Action {
//     fn fmt(&self, f: &mut fmt::Formatter, unit_query: Query<Entity, &Name>) -> fmt::Result {
//         match self {
//             Action {caster, skill, target} => {
//                 match unit_query.get(caster) {
//                     (_, catser_name) => {

//                     }
//                 }
//                 write!(f, "Initiation")
//             }
//         }
//     }
// }

impl Action {
    pub fn new(caster: Entity, skill: Skill, targets: Option<Vec<Entity>>) -> Action {
        Action {
            caster,
            skill,
            targets,
            initiative: -1,
        }
    }
}

impl Ord for Action {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.initiative).cmp(&(other.initiative))
    }
}

impl PartialOrd for Action {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Action {
    /// compare with just the initiative
    fn eq(&self, other: &Self) -> bool {
        (self.initiative) == (other.initiative)
    }
}

impl Eq for Action {}

/* -------------------------------------------------------------------------- */
/*                             -- Run Criteria --                             */
/* -------------------------------------------------------------------------- */

pub fn in_initiation_phase(combat_panel_query: Query<&CombatPanel>) -> bool {
    let combat_panel = combat_panel_query.single();
    combat_panel.phase == CombatState::Initiation
}

pub fn in_alteration_phase(combat_panel_query: Query<&CombatPanel>) -> bool {
    let combat_panel = combat_panel_query.single();
    combat_panel.phase == CombatState::AlterationsExecution
}

pub fn in_caster_phase(combat_panel_query: Query<&CombatPanel>) -> bool {
    let combat_panel = combat_panel_query.single();
    combat_panel.phase == CombatState::SelectionCaster
}

pub fn in_skill_phase(combat_panel_query: Query<&CombatPanel>) -> bool {
    let combat_panel = combat_panel_query.single();
    combat_panel.phase == CombatState::SelectionSkill
}

pub fn in_target_phase(combat_panel_query: Query<&CombatPanel>) -> bool {
    let combat_panel = combat_panel_query.single();
    combat_panel.phase == CombatState::SelectionTarget
}

pub fn in_initiative_phase(combat_panel_query: Query<&CombatPanel>) -> bool {
    let combat_panel = combat_panel_query.single();
    combat_panel.phase == CombatState::RollInitiative
}

pub fn in_executive_phase(combat_panel_query: Query<&CombatPanel>) -> bool {
    let combat_panel = combat_panel_query.single();
    combat_panel.phase == CombatState::ExecuteSkills
}

pub fn in_evasive_phase(combat_panel_query: Query<&CombatPanel>) -> bool {
    let combat_panel = combat_panel_query.single();
    combat_panel.phase == CombatState::Evasion
}
