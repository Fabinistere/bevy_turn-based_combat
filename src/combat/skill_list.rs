//! List all the technic possible
//!
//! We call `spell`, technic that indivuals have regardless of their stuff
//! We call `skill`, technic given by using a certain weapon

use crate::combat::skills::{Skill, SkillType, TargetSide};

use super::alterations::Alteration;

impl Skill {
    pub fn bam() -> Self {
        Skill {
            skill_type: SkillType::Attack,
            target_side: TargetSide::Enemy,
            target_number: 1,
            hp_dealt: 50,
            initiative: 50,
            description: String::from("Deal 50 dmg"),
            name: String::from("Bam"),
            ..Default::default()
        }
    }

    /// Is a spell
    pub fn block() -> Self {
        Skill {
            skill_type: SkillType::Defense,
            target_side: TargetSide::OneSelf,
            target_number: 1,
            shield_dealt: 200,
            initiative: 50,
            description: String::from("Give 200shield"),
            name: String::from("Block"),
            ..Default::default()
        }
    }

    pub fn gifle() -> Self {
        Skill {
            skill_type: SkillType::Attack,
            target_side: TargetSide::Enemy,
            target_number: 1,
            // Immediate
            hp_dealt: 1,
            initiative: 70,
            alterations: vec![Alteration::honte()],
            description: String::from("Frappe Vile qui inflige le débuff Honte"),
            name: String::from("Gifle"),
            ..Default::default()
        }
    }

    pub fn implosion() -> Self {
        Skill {
            skill_type: SkillType::AttackSpe,
            target_side: TargetSide::Enemy,
            target_number: 3,
            aoe: false,
            hp_dealt: 50,
            initiative: 25,
            description: String::from("Deal 25 dmg to 3 enemies"),
            name: String::from("Implosion"),
            ..Default::default()
        }
    }
}
