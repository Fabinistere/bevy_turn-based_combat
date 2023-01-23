//! List all the technic possible

use crate::combat::skills::{Skill, SkillType, TargetSide};

impl Skill {
    pub fn bam() -> Self {
        Skill {
            target_side: TargetSide::Enemy,
            // invalidate by the target side but just in case.
            self_cast: false,
            skill_type: SkillType::Attack,
            hp_dealt: 150,
            initiative: 50,
            description: String::from("Bam"),
            ..Default::default()
        }
    }
}
