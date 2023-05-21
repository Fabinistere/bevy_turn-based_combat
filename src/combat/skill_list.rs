//! List all the technic possible
//!
//! We call `spell`, technic that indivuals have regardless of their stuff
//! We call `skill`, technic given by using a certain weapon

use crate::combat::skills::{Skill, SkillType, TargetSide};

use super::alterations::Alteration;

impl Skill {
    pub fn pass() -> Self {
        Skill {
            skill_type: SkillType::Pass,
            target_side: TargetSide::OneSelf,
            target_number: 1,
            initiative: 0, // 50,
            description: String::from("Do nothing"),
            name: String::from("Pass"),
            ..Default::default()
        }
    }

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

    /// `Deal 25dmg to 3targets` (example of multi-targets skills)
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

    // ------------------------- Weapons' Skills -------------------------

    // --- Pickle Jar ---
    pub fn jar_selfdestruction() -> Self {
        Skill {
            skill_type: SkillType::Attack,
            target_side: TargetSide::Enemy,
            target_number: 1,
            hp_dealt: 60,
            initiative: 30,
            description: String::from("Explode 60 dmg to 1 enemies"),
            name: String::from("SelfDestruct of the Pickles Jar"),
            ..Default::default()
        }
    }

    pub fn eat_a_pickle() -> Self {
        Skill {
            skill_type: SkillType::Heal,
            target_side: TargetSide::OneSelf,
            // TODO: UI Fluidity - Self -> # of target = 0
            target_number: 1,
            hp_dealt: 25,
            initiative: 60,
            alterations: vec![Alteration::regenerate()],
            description: String::from("Heal 25Hp and add Regenerate"),
            name: String::from("Open the jar and eat a pickle"),
            ..Default::default()
        }
    }

    // --- Bass ---
    pub fn melody() -> Self {
        Skill {
            skill_type: SkillType::Buff,
            target_side: TargetSide::Ally,
            target_number: 6,
            initiative: 60,
            mana_cost: 20,
            shield_dealt: 10,
            alterations: vec![Alteration::swiftness()],
            description: String::from("Up the initative of allies and give Shield"),
            name: String::from("Melody"),
            ..Default::default()
        }
    }

    pub fn swing() -> Self {
        Skill {
            skill_type: SkillType::AttackSpe,
            target_side: TargetSide::Enemy,
            // TODO: feature - placement, here NEAR
            target_number: 3,
            initiative: 60,
            mana_cost: 25,
            hp_dealt: 25,
            description: String::from("Slash Near enemies with a hard bass wave"),
            name: String::from("Swing"),
            ..Default::default()
        }
    }

    pub fn solo() -> Self {
        Skill {
            skill_type: SkillType::Buff,
            target_side: TargetSide::OneSelf,
            target_number: 0,
            initiative: 35,
            mana_cost: 25,
            shield_dealt: 25,
            // TODO: alteration -> up % that enemies will target the cursed
            alterations: vec![Alteration::hardness()],
            description: String::from("Give yourself a medium shield and buff your physical defense, focus yourself to aggro"),
            name: String::from("Solo"),
            ..Default::default()
        }
    }
}
