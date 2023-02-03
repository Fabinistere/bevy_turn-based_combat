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

use std::fmt;

use bevy::{
    ecs::schedule::ShouldRun,
    prelude::*,
    // ecs::schedule::ShouldRun,
    time::FixedTimestep,
};

use crate::constants::FIXED_TIME_STEP;

use self::skills::Skill;

pub mod alterations;
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
    // RollInitiative,
    // ExecuteSkills,

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
            CombatState::Evasion => write!(f, "Evasion"),
        }
    }
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_to_stage(
                CoreStage::Update,
                observation
                    .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
                    .label(CombatState::Observation)
            )
            .add_system(skills::execute_skill)
            // .add_system_to_stage(
            //     CoreStage::Update,
            //     stats::roll_initiative
            //         .with_run_criteria(FixedTimestep::step(FIXED_TIME_STEP as f64))
            //         .label(CombatState::RollInitiative)
            // )
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

pub fn run_if_in_caster_phase(combat_phase: Query<&CombatPanel>) -> ShouldRun {
    let combat_panel = combat_phase.single();
    if combat_panel.phase == CombatState::SelectionCaster {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub fn run_if_in_skill_phase(combat_phase: Query<&CombatPanel>) -> ShouldRun {
    let combat_panel = combat_phase.single();
    if combat_panel.phase == CombatState::SelectionSkills {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub fn run_if_in_target_phase(combat_phase: Query<&CombatPanel>) -> ShouldRun {
    let combat_panel = combat_phase.single();
    if combat_panel.phase == CombatState::SelectionTarget {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

#[derive(Component)]
pub struct CombatPanel {
    pub phase: CombatState,
    pub history: Vec<Action>,
}

#[derive(Clone)]
pub struct Action {
    pub caster: Entity,
    pub skill: Skill,
    /// Optional only to allow selecting skill before the target
    pub target: Option<Entity>,
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
        }
    }
}

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
