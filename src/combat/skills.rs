//! Implement SKILLS

use bevy::prelude::*;

use crate::combat::{
    alterations::*,
    stats::{Attack, AttackSpe, Defense, DefenseSpe, Hp, Mana, Shield},
};

use super::Alterations;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum SkillType {
    Heal,
    Attack,
    AttackSpe,
    ShieldBreaker,
    Defense,
    DefenseSpe,
    Buff,
    Debuff,
    #[default]
    Pass,
    Flee,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum TargetSide {
    /// Identity
    OneSelf,
    Enemy,
    /// Include the identity (self)
    #[default]
    Ally,
    /// Exclude the identity (self)
    AllyButSelf,
    All,
}

/// Endure every stats to the target
///
/// - Negative = MALUS
/// - Positive = BONUS
#[derive(Debug, Component, Clone)]
pub struct Skill {
    pub skill_type: SkillType,
    /// Which side the skill is allow to target
    pub target_side: TargetSide,
    /// # Example
    ///
    /// - target all ally/enemy party: MAX_PARTY (6)
    /// - self-target: 1
    /// - targeted heal: 1
    /// - small explosion: 2
    pub target_number: i32,
    /// Area of Effect
    ///
    /// Should the skill affect all target
    /// or one by one
    pub aoe: bool,
    /// Wait for the turn delay to execute
    pub turn_delay: i32,
    /// initiave: slower; faster
    ///
    /// 0 <= init <= 100
    pub initiative: i32,
    /// hp: dmg/heal to the target
    pub hp_dealt: i32,
    /// mana: consume/gain to the target
    pub mana_dealt: i32,
    /// shield: reduce/addition to the target
    ///
    /// # Note
    ///
    /// Can direct
    ///
    /// - a attack to only target shield
    /// - a bonus to regenerate/add shield
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
    pub alterations: Vec<Alteration>,
    /// The 'list' of skills called after this one
    ///
    /// # Note
    ///
    /// Used for complex skill
    pub skills_queue: Vec<Skill>,
    pub description: String,
    pub name: String,
}

impl Default for Skill {
    fn default() -> Self {
        Skill {
            skill_type: Default::default(),
            target_side: TargetSide::default(),
            target_number: 1,
            aoe: false,
            turn_delay: 0,
            initiative: 0,
            hp_dealt: 0,
            hp_cost: 0,
            mana_dealt: 0,
            mana_cost: 0,
            shield_dealt: 0,
            alterations: vec![],
            skills_queue: vec![],
            description: String::from("..."),
            name: String::from("Skill"),
        }
    }
}

fn _skill_caller(_query: Query<(Entity, &Skill)>, // ??
) {
}

/// Happens in
///   - combat::phases::execution_phase
///     - There is a skill to execute
/// Read in
///   - combat::skills::execute_shill
///     - Execute the skill with the caster's Stats
///     to the target
pub struct ExecuteSkillEvent {
    pub skill: Skill,
    pub caster: Entity,
    pub target: Entity,
}

/// # Note
///
/// DOC
/// Carefull with default Skill value
pub fn execute_skill(
    mut execute_skill_event: EventReader<ExecuteSkillEvent>,
    // unit_query: Query<
    //     (Entity, &UnitTargeted, &UnitSelected)
    // >,
    mut combat_unit: Query<
        (
            Entity,
            &mut Hp,
            &mut Mana,
            &mut Shield,
            &Attack,
            &AttackSpe,
            &Defense,
            &DefenseSpe,
            &mut Alterations,
            &Name,
        ),
        // Or<(With<Selected>, With<Targeted>)>
    >,
) {
    for ExecuteSkillEvent {
        skill,
        caster,
        target,
    } in execute_skill_event.iter()
    {
        match combat_unit.get_many_mut([*caster, *target]) {
            // REFACTOR: Handle SelfCast
            Err(e) => warn!("SelfCast or: Caster and/or Target Invalid {:?}", e),
            Ok(
                [(
                    _caster,
                    mut caster_hp,
                    mut caster_mp,
                    _caster_shield,
                    caster_attack,
                    caster_attack_spe,
                    _caster_defense,
                    _caster_defense_spe,
                    caster_alterations,
                    caster_name,
                ), (
                    _target,
                    mut target_hp,
                    mut target_mp,
                    mut target_shield,
                    _target_attack,
                    _target_attack_spe,
                    target_defense_spe,
                    target_defense,
                    mut target_alterations,
                    target_name,
                )],
            ) => {
                info!(
                    "DEBUG: Execute skill: {}, from {} to {}",
                    skill.name, caster_name, target_name
                );

                let skill_executed = &skill;

                // TODO: turn delay?
                // TODO: alteration.s

                // ---- COST ----

                // TODO: feature - reduce cost by stuff and level
                caster_hp.current -= skill_executed.hp_cost;
                caster_mp.current -= skill_executed.mana_cost;

                // don't execute the rest if the current of the caster is < 0
                if caster_hp.current <= 0 {
                    continue;
                }

                // ---- Alterations ----

                target_alterations.extend(skill.clone().alterations);

                // ---- Multipliers ----

                let mut multiplier: i32 = 0;
                for alt in target_alterations.iter() {
                    multiplier += alt.damage_suffered
                }
                for alt in caster_alterations.iter() {
                    multiplier += alt.damage_inflicted
                }

                let target_multiplier: i32;

                match skill_executed.skill_type {
                    SkillType::Heal => {
                        // IDEA: no multiplier ? based on attackspe?

                        // Can't revive with a Heal

                        if target_hp.current < 0 {
                            target_hp.current += skill_executed.hp_dealt;
                            if target_hp.current > target_hp.max {
                                target_hp.current = target_hp.max;
                            }
                        }
                    }
                    SkillType::Attack => {
                        multiplier += (caster_attack.base + caster_attack.modifier_flat)
                            * caster_attack.modifier_percent
                            / 100;
                        target_multiplier = (target_defense.base + target_defense.modifier_flat)
                            * target_defense.modifier_percent
                            / 100;

                        // x + x*(caster_attack)% - x*(target_defense)%
                        let hp_dealt = skill_executed.hp_dealt
                            + skill_executed.hp_dealt * multiplier / 100
                            - skill_executed.hp_dealt * target_multiplier / 100;
                        if hp_dealt > 0 {
                            info!("hp dealt: {}", hp_dealt);
                        }

                        // ---- MP ----
                        // x + x*(caster_attack_spe)%
                        let mp_dealt = skill_executed.mana_dealt;
                        if mp_dealt > 0 {
                            info!("mp dealt: {}", mp_dealt);
                        }

                        // ---- EXECUTION ----
                        if target_shield.0 < hp_dealt {
                            target_hp.current -= hp_dealt - target_shield.0;
                            target_shield.0 = 0;
                        } else {
                            // the shield fulyl tank the attack
                            target_shield.0 -= hp_dealt;
                        }
                        // neagtive hp allowed

                        target_hp.current -= mp_dealt;
                        if target_mp.current < 0 {
                            target_mp.current = 0
                        }
                    }
                    SkillType::AttackSpe => {
                        multiplier += (caster_attack_spe.base + caster_attack_spe.modifier_flat)
                            * caster_attack_spe.modifier_percent
                            / 100;
                        target_multiplier = (target_defense_spe.base
                            + target_defense_spe.modifier_flat)
                            * target_defense_spe.modifier_percent
                            / 100;

                        // ---- HP ----
                        // x + x*(caster_attack_spe)% - x*(target_defense_spe)%
                        let hp_dealt = skill_executed.hp_dealt
                            + skill_executed.hp_dealt * multiplier / 100
                            - skill_executed.hp_dealt * target_multiplier / 100;
                        if hp_dealt > 0 {
                            info!("hp dealt: {}", hp_dealt);
                        }

                        // ---- MP ----
                        // x + x*(caster_attack_spe)%
                        let mp_dealt = skill_executed.mana_dealt
                            + skill_executed.mana_dealt * multiplier / 100;
                        if mp_dealt > 0 {
                            info!("mp dealt: {}", mp_dealt);
                        }

                        // ---- EXECUTION ----
                        target_hp.current -= hp_dealt;
                        // neagtive hp allowed

                        target_mp.current -= mp_dealt;
                        if target_mp.current < 0 {
                            target_mp.current = 0
                        }
                    }
                    // shield_dealt is neagtive when harmfull or positive when bonus
                    SkillType::ShieldBreaker | SkillType::Defense => {
                        target_shield.0 += skill_executed.shield_dealt;
                        if target_shield.0 < 0 {
                            target_shield.0 = 0
                        }
                    }
                    SkillType::DefenseSpe => {
                        // TODO: Magic Shield
                    }
                    SkillType::Pass => {
                        // force action: Pass to the target next turn
                    }
                    _ => {}
                }
            }
        }
    }
}
