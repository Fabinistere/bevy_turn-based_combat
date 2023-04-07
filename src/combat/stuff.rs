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

#[derive(Component)]
pub struct Equipement;

/// A Job being
///
/// - tier 2 have access to all tier 1 and tier 0
/// - tier 1 have access to all tier 0
#[derive(Component)]
pub struct SkillTiers {
    pub tier_2: Vec<Skill>,
    pub tier_1: Vec<Skill>,
    pub tier_0: Vec<Skill>,
}

pub fn spawn_stuff(mut commands: Commands) {
    // ADMIRAL
    commands.spawn((
        Name::new("Bocal à gros cornichons"),
        Equipement,
        SkillTiers {
            tier_2: vec![Skill::bam()],
            tier_1: vec![],
            tier_0: vec![],
        },
        StatBundle::default(),
    ));
}
