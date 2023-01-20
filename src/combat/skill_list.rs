//! List all the technic possible

use super::skills::{Skill, SkillType};

impl Skill {
    pub fn bam() -> Self {
        Skill {
            skill_type: SkillType::Attack,
            hp_dealt: 150,
            initiative: 50,
            description: String::from("Bam"),
            ..Default::default()
        }
    }
}
