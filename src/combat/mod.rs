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
    // ecs::schedule::ShouldRun,
    time::FixedTimestep,
};

use crate::constants::FIXED_TIME_STEP;

use self::skills::Skill;

pub mod alterations;
pub mod phases;
pub mod skill_list;
pub mod skills;
pub mod stats;

/// Just help to create a ordered system in the app builder
#[derive(PartialEq, Clone, Hash, Debug, Eq, SystemLabel)]
pub enum CombatState {
    Initiation,
    Observation,
    // ManageStuff,
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
            .add_system_to_stage(
                CoreStage::Update,
                observation
                    .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                    .label(CombatState::Observation)
            )
            .add_system(skills::execute_skill)
            .add_system_to_stage(
                CoreStage::Update,
                phases::roll_initiative
                    // FixedTimestep::step(FIXED_TIME_STEP as f64)
                    .with_run_criteria(run_if_in_initiative_phase)
                    .label(CombatState::RollInitiative)
            )
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

fn observation() {
    // println!("Now it's your turn...")
}

pub fn run_if_in_initiation_phase(combat_panel_query: Query<&CombatPanel>) -> ShouldRun {
    let combat_panel = combat_panel_query.single();
    if combat_panel.phase == CombatState::Initiation {
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

#[derive(Component)]
pub struct Karma(pub i32);

#[derive(Component)]
pub struct InCombat;

#[derive(Clone, Copy, Component)]
pub struct Leader;

/// The team an entity is assigned to.
#[derive(Copy, Clone, PartialEq, Eq, Component, Deref, DerefMut)]
pub struct Team(pub i32);

/// The player can recruted some friendly npc
/// Can be called, TeamPlayer
#[derive(Copy, Clone, PartialEq, Eq, Component)]
pub struct Recruted;

#[derive(Component)]
pub struct FairPlayTimer {
    /// (non-repeating timer)
    /// Let the enemy go when reached/left behind
    pub timer: Timer,
}
