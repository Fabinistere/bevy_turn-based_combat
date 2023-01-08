//! Implement SKILLS

use crate::combat::alterations::*;
use bevy::prelude::*;

pub enum SkillType {
    Heal,
    Attack,
    AttackSpe,
    Defense,
    DefenseSpe,
    Buff,
    Debuff,
    Pass,
    Flee,
}

pub enum TargetSide {
    Enemy,
    Ally,
}

/// Endure every stats to the target
///
/// - Negative = MALUS
/// - Positive = BONUS
#[derive(Component)]
pub struct Skill {
    pub skill_type: SkillType,
    /// Which side the skill is allow to target
    /// and how many (0 for self-target)
    ///
    /// # Example
    ///
    /// - target all ally party: (Ally, 6)
    /// - self-target: (Ally, 0)
    pub target_option: (TargetSide, i32),
    /// Wait for the turn delay to execute
    pub turn_delay: i32,
    /// initiave: slower; faster
    pub initiative: i32,
    /// hp: dmg/heal to the target
    pub hp_dealt: i32,
    /// mana: consume/gain to the target
    pub mana_dealt: i32,
    /// shield: reduce/addition to the target
    pub shield_dealt: i32,
    /// Self-inflicted Dmg
    ///
    /// # Note
    ///
    /// Shouldn't be used for casual self-healing
    pub hp_cost: i32,
    /// The Skill's Mana cost
    pub mana_cost: i32,
    // TODO: feature: shield cost ?
    /// Debuff or Buff
    pub alteration: Vec<Alteration>,
    /// The 'list' of skills called after this one
    ///
    /// # Note
    ///
    /// Used for complex skill
    pub skills_queue: Vec<Skill>,
    pub description: String,
}

impl Default for Skill {
    fn default() -> Self {
        Skill {
            skill_type: SkillType::Pass,
            target_option: (TargetSide::Ally, 0),
            turn_delay: 0,
            initiative: 0,
            hp_dealt: 0,
            hp_cost: 0,
            mana_dealt: 0,
            mana_cost: 0,
            shield_dealt: 0,
            alteration: vec![],
            skills_queue: vec![],
            description: String::from("..."),
        }
    }
}

fn skill_caller(query: Query<(Entity, &Skill)>, // ??
) {
}
