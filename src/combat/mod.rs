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

use bevy::{
    ecs::schedule::ShouldRun,
    prelude::*,
};
use bevy_inspector_egui::Inspectable;

use self::{
    alterations::Alteration, phases::observation, skills::Skill, stats::StatBundle,
    stuff::Equipements,
};

pub mod alteration_list;
pub mod alterations;
pub mod item_list;
pub mod phases;
pub mod skill_list;
pub mod skills;
pub mod stats;
pub mod stuff;

/// Just help to create a ordered system in the app builder
#[derive(PartialEq, Clone, Hash, Debug, Eq, SystemLabel)]
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
    SelectionCaster,
    SelectionSkills,
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
            CombatState::SelectionSkills => write!(f, "SelectionSkills"),
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
            
            .add_event::<skills::ExecuteSkillEvent>()
            // .add_event::<alterations::ExecuteAlterationEvent>()
            
            .add_system(
                observation
                    // .with_run_criteria(run_if_in_observation_phase)
                    .label(CombatState::Observation)
            )
            .add_system(
                phases::roll_initiative
                // FixedTimestep::step(FIXED_TIME_STEP as f64)
                .with_run_criteria(run_if_in_initiative_phase)
                .label(CombatState::RollInitiative)
            )
            .add_system(
                phases::execution_phase
                .with_run_criteria(run_if_in_executive_phase)
                .label(CombatState::ExecuteSkills)
            )
            .add_system(skills::execute_skill)
            // .add_system_set_to_stage(
            //     CoreStage::PostUpdate,
            //     SystemSet::new()
            //         .with_run_criteria(run_if_pressed_h)
            //         .with_system(show_hp)
            //         .with_system(show_mana)
            // )
            ;
    }
}

// -- Combat Components --

#[derive(Bundle)]
pub struct CombatBundle {
    pub karma: Karma,
    pub team: Team,
    pub alterations: Alterations,
    pub skills: Skills,
    pub equipements: Equipements,

    #[bundle]
    pub stats: StatBundle,
}

#[derive(Component, Default)]
pub struct Karma(pub i32);

/// The team an entity is assigned to.
#[derive(Copy, Clone, PartialEq, Eq, Component, Deref, DerefMut)]
pub struct Team(pub i32);

/// Ongoing alterations, Debuff or Buff
#[derive(Component, Deref, DerefMut, Inspectable)]
pub struct Alterations(pub Vec<Alteration>);

/// Basic/Natural skills own by the entity  
#[derive(Component)]
pub struct Skills(pub Vec<Skill>);

#[derive(Component)]
pub struct InCombat;

#[derive(Clone, Copy, Component)]
pub struct Leader;

/// The player can recruted some friendly npc
/// Can be called, TeamPlayer
#[derive(Copy, Clone, PartialEq, Eq, Component)]
pub struct Recruted;

// -- Combat Core Operation --

/// REFACTOR: Resource ?
#[derive(Component)]
pub struct CombatPanel {
    pub phase: CombatState,
    pub history: Vec<Action>,
}

#[derive(Debug, Clone)]
pub struct Action {
    pub caster: Entity,
    pub skill: Skill,
    /// Optional only to allow selecting skill before the target
    pub target: Option<Entity>,
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
    pub fn new(caster: Entity, skill: Skill, target: Option<Entity>) -> Action {
        Action {
            caster,
            skill,
            target,
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

// -- Run Criteria --

pub fn run_if_in_initiation_phase(combat_panel_query: Query<&CombatPanel>) -> ShouldRun {
    let combat_panel = combat_panel_query.single();
    if combat_panel.phase == CombatState::Initiation {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub fn run_if_in_alteration_phase(combat_panel_query: Query<&CombatPanel>) -> ShouldRun {
    let combat_panel = combat_panel_query.single();
    if combat_panel.phase == CombatState::AlterationsExecution {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub fn run_if_in_caster_phase(combat_panel_query: Query<&CombatPanel>) -> ShouldRun {
    let combat_panel = combat_panel_query.single();
    if combat_panel.phase == CombatState::SelectionCaster {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub fn run_if_in_skill_phase(combat_panel_query: Query<&CombatPanel>) -> ShouldRun {
    let combat_panel = combat_panel_query.single();
    if combat_panel.phase == CombatState::SelectionSkills {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub fn run_if_in_target_phase(combat_panel_query: Query<&CombatPanel>) -> ShouldRun {
    let combat_panel = combat_panel_query.single();
    if combat_panel.phase == CombatState::SelectionTarget {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub fn run_if_in_initiative_phase(combat_panel_query: Query<&CombatPanel>) -> ShouldRun {
    let combat_panel = combat_panel_query.single();
    if combat_panel.phase == CombatState::RollInitiative {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub fn run_if_in_executive_phase(combat_panel_query: Query<&CombatPanel>) -> ShouldRun {
    let combat_panel = combat_panel_query.single();
    if combat_panel.phase == CombatState::ExecuteSkills {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub fn run_if_in_evasive_phase(combat_panel_query: Query<&CombatPanel>) -> ShouldRun {
    let combat_panel = combat_panel_query.single();
    if combat_panel.phase == CombatState::Evasion {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}
