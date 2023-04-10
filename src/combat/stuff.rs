//! # Stuffs
//!
//! Stuff is more than just some stats changes.
//!
//! Equip a weapon will give you a full set of skill.

use bevy::prelude::*;

use super::{skills::Skill, stats::StatBundle};

/// On a character
#[derive(Component)]
pub struct Equipements {
    /// Max One
    pub weapon: Option<Entity>,
    pub armor: Option<Entity>,
}

// --- Equipement Components ---

/// Contains the user if in use
#[derive(Component)]
pub struct Equipement(pub Option<Entity>);

/// A Job being
///
/// - tier 2 have access to all tier 1 and tier 0
/// - tier 1 have access to all tier 0
/// - tier 0 only have access to its tier
#[derive(Component)]
pub struct SkillTiers {
    pub tier_2: Vec<Skill>,
    pub tier_1: Vec<Skill>,
    pub tier_0: Vec<Skill>,
}

pub fn spawn_stuff(mut commands: Commands) {
    // Bocal à gros cornichons
    commands.spawn((
        Name::new("Bocal à gros cornichons"),
        Equipement(None),
        SkillTiers {
            tier_2: vec![Skill::jar_selfdestruction()],
            tier_1: vec![Skill::eat_a_pickle()],
            tier_0: vec![],
        },
        StatBundle::default(),
    ));
}
