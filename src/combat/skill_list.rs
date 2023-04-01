//! List all the technic possible

use crate::combat::skills::{Skill, SkillType, TargetSide};

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
}
