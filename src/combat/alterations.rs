//! Implement all Combat Buffs and Debuffs 

use bevy::prelude::*;

use super::stats::*;
use super::skills::*;

#[derive(Clone)]
pub enum AlterationAction {
    StatsReduction,
    Poison,
    RestrainToAttack,
    ForcePass
}

/// Alteration will last for exactly `duration` turn,
/// can be cured/removed by some clean skills.
/// 
/// 
/// 
/// # Note
/// 
/// - Curse or Benediction
/// - Debuff or Buff
/// - Detract or Enhance
#[derive(Component, Clone)]
pub struct Alteration {
    /// Alteration's type
    pub action: AlterationAction,
    /// Number of turn remaining
    pub duration: i32,
    /// Which side the skill is allow to target
    /// and how many (0 for self-target)
    ///
    /// # Example
    ///
    /// - target all ally party: (Ally, 6)
    /// - self-target: (Ally, 0)
    pub target_option: (TargetSide, i32),

    // Dots

    /// hp dealt or healed each time the target plays
    /// 
    /// hp: dmg/heal to the target
    pub hp_dealt: i32,
    /// mp consume or gained each time the target plays
    /// 
    /// mana: consume/gain to the target
    pub mana_dealt: i32,
    /// shiled point reduced or added each time the target plays
    /// 
    /// shield: reduce/addition to the target
    pub shield_dealt: i32,

    // Stats' Modification
    // TODO: feature allow Stats' Modification as a dot
    /// reduce/gain hp max
    pub hp_max: i32,
    /// reduce/gain mp max
    pub mana_max: i32,
    /// att: lose/gain
    pub attack: Attack,
    /// att spe: lose/gain
    pub attack_spe: AttackSpe,
    /// def: lose/gain
    pub defense: Defense,
    /// def spe: lose/gain
    pub defense_spe: DefenseSpe,

    pub description: String,
}

impl Default for Alteration {
    fn default() -> Self {
        Alteration {
            action: AlterationAction::ForcePass,
            duration: 0,
            target_option: (TargetSide::Ally, 0),
            // Dots
            hp_dealt: 0,
            mana_dealt: 0,
            shield_dealt: 0,
            // Stats
            hp_max: 0,
            mana_max: 0,
            attack: Attack::default(),
            attack_spe: AttackSpe::default(),
            defense: Defense::default(),
            defense_spe: DefenseSpe::default(),
            description: String::from("An Alteration"),
        }
    }
}
