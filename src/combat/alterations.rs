//! Implement all Combat Buffs and Debuffs

// use std::default;

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use super::skills::*;

// #[derive(Debug, Clone, Default)]
// pub enum AlterationAction {
//     StatsReduction,
//     Poison,
//     RestrainToAttack,
//     #[default]
//     ForcePass,
//     DamageSponge,
// }

#[derive(Debug, Clone, Default, Inspectable)]
pub enum AlterationAction {
    #[default]
    Dots,
    StatsFlat,
    /// FIXME: how to undo changes?
    StatsPercentage,
    /// ??
    PercentageAsDots,
}

/// Alteration will last for exactly `duration` turn,
/// can be cured/removed by some clean skills.
///
/// # Note
///
/// - Curse or Benediction
/// - Debuff or Buff
/// - Detract or Enhance
#[derive(Debug, Component, Clone, Inspectable)]
pub struct Alteration {
    /// Alteration's type
    pub action: AlterationAction,
    /// Number of turn that the alteration lasts
    ///
    /// SHOULD NOT BE MODIFIED by user
    pub turn_count: i32,
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

    /// hp dealt or healed each time the target plays
    ///
    /// hp: dmg/heal to the target
    pub hp: i32,
    /// mp consume or gained each time the target plays
    ///
    /// mana: consume/gain to the target
    pub mana: i32,
    /// shiled point reduced or added each time the target plays
    ///
    /// shield: reduce/addition to the target
    pub shield: i32,
    /// att: lose/gain
    pub attack: i32,
    /// att spe: lose/gain
    pub attack_spe: i32,
    /// def: lose/gain
    pub defense: i32,
    /// def spe: lose/gain
    pub defense_spe: i32,

    /// 0 = no change;
    /// x = + x%
    pub damage_inflicted: i32,
    /// 0 = no change;
    /// x = + x%
    pub damage_suffered: i32,

    /// 0 = no change;
    /// x = + x%
    ///
    /// DOC: or heal_gived
    pub heal_inflicted: i32,
    /// 0 = no change;
    /// x = + x%
    pub heal_received: i32,

    pub description: String,
    pub name: String,
    pub path_icon: String,
}

impl Default for Alteration {
    fn default() -> Self {
        Alteration {
            action: AlterationAction::Dots,
            turn_count: 0,
            duration: 1,
            target_option: (TargetSide::Ally, 0),
            hp: 0,
            mana: 0,
            shield: 0,
            attack: 0,
            attack_spe: 0,
            defense: 0,
            defense_spe: 0,
            damage_inflicted: 0,
            damage_suffered: 0,
            heal_inflicted: 0,
            heal_received: 0,
            description: String::from("Mystery Alteration"),
            name: String::from("An Alteration"),
            path_icon: String::from("assets/textures/icons/Nature_7"),
        }
    }
}

// /// Happens when
// ///   - combat::phases::alteration_phase
// ///     - There is an alteration to execute
// ///
// /// Read in
// ///   - ???
// ///     - Execute the alteration to the given entity
// pub struct ExecuteAlterationEvent {
//     pub target: Entity,
//     pub alteration: Alteration,
// }
