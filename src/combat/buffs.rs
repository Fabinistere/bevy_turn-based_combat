//! Implement all Combat Buffs and Debuffs 

use bevy::prelude::*;

use super::stats::*;

pub enum BuffAction {
    StatsReduction,
    Poison,
    RestrainToAttack,
    ForcePass
}

/// Evolve
/// 
/// # Note
/// 
/// DOC: Find a better neutral name
/// 
/// - Curse or Benediction
/// - Debuff or Buff
/// - Detract or Enhance
#[derive(Component)]
pub struct Evolve {
    pub action: BuffAction,
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
}

impl Default for Evolve {
    fn default() -> Self {
        Evolve {
            action: BuffAction::ForcePass,
            hp_max: 0,
            mana_max: 0,
            attack: Attack::default(),
            attack_spe: AttackSpe::default(),
            defense: Defense::default(),
            defense_spe: DefenseSpe::default(),
        }
    }
}
