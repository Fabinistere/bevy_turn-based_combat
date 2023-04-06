use bevy::prelude::*;
use rand::Rng;

use crate::{
    combat::{stats::Initiative, Action, CombatPanel, CombatState},
    npc::NPC,
};

use super::{
    alterations::{Alteration, AlterationAction},
    skills::ExecuteSkillEvent,
    stats::{Attack, AttackSpe, Defense, DefenseSpe, Hp, Mana, Shield},
    Alterations,
};

// /// Inflict Dots and lower of 1turn all alterations duration
// ///
// /// # Notes
// ///
// /// REFACTOR: Not sure that this abstraction is usefull (not need of execution order)
// pub fn alteration_phase(
//     mut character_query: Query<(Entity, &Alterations), With<InCombat>>,
//     mut combat_panel_query: Query<&mut CombatPanel>,

//     mut execute_alteration_event: EventWriter<ExecuteAlterationEvent>,
// ) {
//     for (character, alterations) in character_query.iter_mut() {
//         for alteration in alterations.iter() {
//             execute_alteration_event.send(ExecuteAlterationEvent {
//                 target: character,
//                 alteration: alteration.clone(),
//             });
//         }
//     }

//     let mut combat_panel = combat_panel_query.single_mut();
//     combat_panel.phase = CombatState::SelectionCaster;
// }

// TODO: ShouldHave - Display mutable change (dmg, heal)
// TODO: CouldHave - Display the combat log somewhere

/// # Note
///
/// DOC
pub fn execute_alteration(
    // mut execute_alteration_event: EventReader<ExecuteAlterationEvent>,
    mut combat_panel_query: Query<&mut CombatPanel>,
    mut character_query: Query<(
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
    )>,
) {
    // for ExecuteAlterationEvent { target, alteration } in execute_alteration_event.iter() {
    for (
        _character,
        mut hp,
        mut mp,
        mut shield,
        // TODO: remove these queries
        _attack,
        _attack_spe,
        _defense_spe,
        _defense,
        mut alterations,
        name,
    ) in character_query.iter_mut()
    {
        let mut new_alterations_vector: Vec<Alteration> = vec![];
        for mut alteration in alterations.iter_mut() {
            info!("DEBUG: Execute Alteration: {} on {}", alteration.name, name);

            match alteration.action {
                AlterationAction::StatsFlat | AlterationAction::StatsPercentage => {
                    // occurs only the first time
                    if alteration.turn_count != 0 {
                        continue;
                    }
                }
                _ => {}
            }
            match alteration.action {
                AlterationAction::Dots => {
                    hp.current += alteration.hp;
                    mp.current += alteration.mana;
                    shield.0 += alteration.shield;
                }
                // The action of StatsPercentage will not trigger if its tunn_count != 0
                AlterationAction::StatsPercentage | AlterationAction::PercentageAsDots => {
                    if alteration.hp != 0 {
                        hp.current *= alteration.hp;
                    }
                    if alteration.mana != 0 {
                        mp.current *= alteration.mana;
                    }
                    if alteration.shield != 0 {
                        shield.0 *= alteration.shield;
                    }
                    if alteration.turn_count != 0 {
                        // At each turn, we increment/decrement the alteration's stats
                        // ----- EX: +10% attack/turn -----
                        // t0: alt.attck = 10;  When the alteration is inserted
                        // t1: 10 + 10/1;       When the first altPhase occurs
                        // ...
                        // t5: 10 + 10/1 + 20/2 + 30/3 + 40/4;
                        alteration.attack += alteration.attack / alteration.turn_count;
                        alteration.attack_spe += alteration.attack_spe / alteration.turn_count;
                        alteration.defense += alteration.defense / alteration.turn_count;
                        alteration.defense_spe += alteration.defense_spe / alteration.turn_count;
                    }
                }
                AlterationAction::StatsFlat => {
                    // no action, the alteration being still in the entity contains all the info.
                }
            }

            if alteration.duration > 0 {
                new_alterations_vector.push(alteration.clone());
            }
            alteration.turn_count += 1;
            alteration.duration -= 1;
        }
        // update the set of alteration
        alterations.0 = new_alterations_vector;
    }
    let mut combat_panel = combat_panel_query.single_mut();
    combat_panel.phase = CombatState::SelectionCaster;
}

pub fn observation() {
    // println!("Now it's your turn...")
}

/// Roll for each entity a d100 ranged into +-20 initiative
/// ALso Display the final score
///
/// Sort the result in a nice table
/// In case of egality: pick the higher initiative boyo to be on top
pub fn roll_initiative(
    npc_query: Query<&Initiative, With<NPC>>,
    mut combat_panel_query: Query<&mut CombatPanel>,
) {
    let mut combat_panel = combat_panel_query.single_mut();

    let mut initiatives: Vec<Action> = Vec::new();

    for mut action in combat_panel.history.iter_mut() {
        let caster = action.caster;
        // REFACTOR: how the initiative is calculated
        let skill_init = action.skill.initiative.clone();

        match npc_query.get(caster) {
            Err(e) => warn!("Invalid Caster are in the History: {}", e),
            Ok(npc_init) => {
                let npc_number = if npc_init.0 - 20 <= 0 {
                    rand::thread_rng().gen_range(0..npc_init.0 + 20)
                } else if npc_init.0 == 100 {
                    100
                } else if npc_init.0 + 20 >= 100 {
                    rand::thread_rng().gen_range(npc_init.0 - 20..100)
                } else {
                    rand::thread_rng().gen_range(npc_init.0 - 20..npc_init.0 + 20)
                };

                let skill_number = if skill_init - 20 <= 0 {
                    rand::thread_rng().gen_range(0..skill_init + 20)
                } else if skill_init == 100 {
                    100
                } else if skill_init + 20 >= 100 {
                    rand::thread_rng().gen_range(skill_init - 20..100)
                } else {
                    rand::thread_rng().gen_range(skill_init - 20..skill_init + 20)
                };

                // 0 <= action.initiative <= 200

                // insert these number in a vector
                action.initiative = npc_number + skill_number;
                initiatives.push(action.clone());
            }
        }
    }

    initiatives.sort();
    initiatives.reverse();

    info!("DEBUG: Initiative: {:?}", initiatives);

    // Update the actions history
    combat_panel.history = initiatives;

    combat_panel.phase = CombatState::ExecuteSkills;
}

pub fn execution_phase(
    mut combat_panel_query: Query<&mut CombatPanel>,

    mut execute_skill_event: EventWriter<ExecuteSkillEvent>,
) {
    let mut combat_panel = combat_panel_query.single_mut();

    for Action {
        caster,
        skill,
        target,
        initiative: _,
    } in combat_panel.history.iter()
    {
        // we will do a verification anyway (skill's hp_cost)
        // in the event handler
        // to control tabht the caster is alive at the time of the execution
        execute_skill_event.send(ExecuteSkillEvent {
            skill: skill.clone(),
            caster: *caster,
            target: target.unwrap(),
        });

        // should be in order
        for combo_skill in skill.skills_queue.iter() {
            execute_skill_event.send(ExecuteSkillEvent {
                skill: combo_skill.clone(),
                caster: *caster,
                // All skills in the queue will be directed to the same target
                target: target.unwrap(),
            });
        }
    }

    // IDEA: add this history into a log to permit the player to see it.

    // Reset the action history
    combat_panel.history = Vec::new();

    // TODO: Go to Observation
    combat_panel.phase = CombatState::SelectionCaster;
}
