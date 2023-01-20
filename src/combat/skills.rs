//! Implement SKILLS

use crate::{
    combat::{
        alterations::*,
        stats::{Attack, Hp},
    },
    ui::dialog_player::ExecuteSkillEvent,
};
use bevy::prelude::*;

#[derive(Default)]
pub enum SkillType {
    Heal,
    Attack,
    AttackSpe,
    Defense,
    DefenseSpe,
    Buff,
    Debuff,
    #[default]
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
            skill_type: Default::default(),
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

fn _skill_caller(_query: Query<(Entity, &Skill)>, // ??
) {
}

/// DOC
pub fn execute_skill(
    mut execute_skill_event: EventReader<ExecuteSkillEvent>,
    // unit_query: Query<
    //     (Entity, &UnitTargeted, &UnitSelected)
    // >,
    mut combat_unit: Query<(Entity, &mut Hp, &Attack)>,
) {
    for event in execute_skill_event.iter() {
        let self_cast = event.caster == event.target;

        if self_cast {
            match combat_unit.get_mut(event.caster) {
                Err(_) => warn!("Caster (=Target) is invalid"),
                Ok((_caster, mut caster_hp, caster_attack)) => {
                    let hp_dealt = event.skill.hp_dealt + event.skill.hp_dealt * caster_attack.0;
                    caster_hp.current_hp = caster_hp.current_hp - hp_dealt;
                    info!(
                        "hp dealt: {}; caster's hp: {}",
                        hp_dealt, caster_hp.current_hp
                    );
                }
            }
        } else {
            match combat_unit.get_many_mut([event.caster, event.target]) {
                Err(e) => warn!("Caster or Target Invalid: {:?}", e),
                Ok(
                    [(_caster, _caster_hp, caster_attack), (_target, mut target_hp, _target_attack)],
                ) => {
                    let hp_dealt = event.skill.hp_dealt + event.skill.hp_dealt * caster_attack.0;
                    target_hp.current_hp = target_hp.current_hp - hp_dealt;
                    info!(
                        "hp dealt: {}; target's hp: {}",
                        hp_dealt, target_hp.current_hp
                    );
                }
            }
        }
    }
}
