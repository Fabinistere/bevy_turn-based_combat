//! Implement SKILLS

use bevy::prelude::*;

use crate::{
    combat::{
        alterations::*,
        stats::{Attack, AttackSpe, Defense, DefenseSpe, Hp, Mana, Shield},
    },
    ui::player_interaction::ExecuteSkillEvent,
};

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
    pub alteration: Vec<Alteration>,
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
            alteration: vec![],
            skills_queue: vec![],
            description: String::from("..."),
            name: String::from("Skill"),
        }
    }
}

fn _skill_caller(_query: Query<(Entity, &Skill)>, // ??
) {
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
                caster_hp.current_hp -= skill_executed.hp_cost;
                caster_mp.current_mana -= skill_executed.mana_cost;

                // don't execute the rest if the current_hp of the caster is < 0
                if caster_hp.current_hp <= 0 {
                    continue;
                }

                let multiplier;
                match skill_executed.skill_type {
                    SkillType::Heal => {
                        // IDEA: no multiplier ? based on attackspe?

                        // Can't revive with a Heal

                        if target_hp.current_hp < 0 {
                            target_hp.current_hp += skill_executed.hp_dealt;
                            if target_hp.current_hp > target_hp.max_hp {
                                target_hp.current_hp = target_hp.max_hp;
                            }
                        }
                    }
                    SkillType::Attack => {
                        multiplier = caster_attack.0;

                        // ---- HP ----
                        // x + x*(caster_attack)% - x*(target_defense)%
                        let hp_dealt = skill_executed.hp_dealt
                            + skill_executed.hp_dealt * multiplier / 100
                            - skill_executed.hp_dealt * target_defense.0 / 100;
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
                            target_hp.current_hp -= hp_dealt - target_shield.0;
                            target_shield.0 = 0;
                        } else {
                            // the shield fulyl tank the attack
                            target_shield.0 -= hp_dealt;
                        }
                        // neagtive hp allowed

                        target_hp.current_hp -= mp_dealt;
                        if target_mp.current_mana < 0 {
                            target_mp.current_mana = 0
                        }
                    }
                    SkillType::AttackSpe => {
                        multiplier = caster_attack_spe.0;

                        // ---- HP ----
                        // x + x*(caster_attack_spe)% - x*(target_defense_spe)%
                        let hp_dealt = skill_executed.hp_dealt
                            + skill_executed.hp_dealt * multiplier / 100
                            - skill_executed.hp_dealt * target_defense_spe.0 / 100;
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
                        target_hp.current_hp -= hp_dealt;
                        // neagtive hp allowed

                        target_mp.current_mana -= mp_dealt;
                        if target_mp.current_mana < 0 {
                            target_mp.current_mana = 0
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
