//! List all the technic possible

use crate::combat::skills::{Skill, SkillType, TargetSide};

impl Skill {
    pub fn bam() -> Self {
        Skill {
            skill_type: SkillType::Attack,
            target_side: TargetSide::Enemy,
            target_number: 1,
            // invalidate by the target side but just in case.
            self_cast: false,
            hp_dealt: 150,
            initiative: 50,
            description: String::from("Bam"),
            ..Default::default()
        }
    }

    pub fn block() -> Self {
        Skill {
            skill_type: SkillType::Defense,
            target_side: TargetSide::Ally,
            target_number: 1,
            // invalidate by the target side but just in case.
            self_cast: true,
            shield_dealt: 200,
            initiative: 50,
            description: String::from("Block"),
            ..Default::default()
        }
    }
}
