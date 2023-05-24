use bevy::prelude::*;
use rand::Rng;

use crate::{
    combat::{
        alterations::{Alteration, AlterationAction},
        skills::{ExecuteSkillEvent, TargetSide},
        stats::{Attack, AttackSpe, Defense, DefenseSpe, Hp, Initiative, Mana, Shield},
        Action, ActionCount, Alterations, CombatPanel, CombatState, InCombat,
    },
    npc::NPC,
    ui::combat_system::{
        ActionHistoryDisplayer, ActionsLogs, LastActionHistoryDisplayer, Selected, Targeted,
    },
};

// ----- Transitions Between Phase -----

/// Whenever:
/// - A system ask for a phase transition/change
///
/// Read by:
/// - combat::phases::phase_transition()
///   - Determine which action to be taken,
///   accordingly with (/w.r.t.) to the phase we're currently in,
///   and the phase we want to transit.
pub struct TransitionPhaseEvent(pub CombatState);

/// Action manager, about phase transition.
/// And Change phase afterall
pub fn phase_transition(
    mut transition_phase_event: EventReader<TransitionPhaseEvent>,

    mut commands: Commands,
    mut combat_panel_query: Query<&mut CombatPanel>,

    mut selected_unit_query: Query<(Entity, &mut ActionCount), With<Selected>>,
    targeted_unit_query: Query<(Entity, &Name), With<Targeted>>,

    // REFACTOR: --- Abstraction Needed ---
    mut combat_unit: Query<&mut ActionCount, With<InCombat>>,

    action_displayer_query: Query<&Text, With<ActionHistoryDisplayer>>,
    mut last_action_displayer_query: Query<
        &mut Text,
        (
            With<LastActionHistoryDisplayer>,
            Without<ActionHistoryDisplayer>,
        ),
    >,
    mut actions_logs_query: Query<
        &mut Text,
        (
            With<ActionsLogs>,
            Without<LastActionHistoryDisplayer>,
            Without<ActionHistoryDisplayer>,
        ),
    >,
) {
    // TODO: event Handler to change phase
    for TransitionPhaseEvent(phase_requested) in transition_phase_event.iter() {
        let mut combat_panel = combat_panel_query.single_mut();
        let mut next_phase = phase_requested;

        let default_state = CombatState::default();

        match (combat_panel.phase.clone(), phase_requested) {
            (CombatState::SelectionCaster, CombatState::SelectionSkill) => {}
            (CombatState::SelectionSkill, CombatState::SelectionCaster) => {
                // FIXME: there is still some Targeted - While switching Caster to caster after the creation of a action
            }
            (CombatState::SelectionSkill, CombatState::SelectionTarget) => {
                // remove from previous entity the targeted component
                for (targeted, _) in targeted_unit_query.iter() {
                    commands.entity(targeted).remove::<Targeted>();
                }

                // if the skill is a selfcast => put the self into the target,
                // IDEA: change the phase to SelectionCaster and `continue;` -> skip the phase change
                let last_action = combat_panel.history.last_mut().unwrap();
                if last_action.skill.target_side == TargetSide::OneSelf {
                    last_action.targets = Some(vec![last_action.caster]);
                    // If there is still some action left for the current caster,
                    // skip SelectionTarget
                    let mut action_count = selected_unit_query
                        .get_component_mut::<ActionCount>(last_action.caster)
                        .unwrap();

                    action_count.current -= 1;
                    info!("action left: {}", action_count.current);

                    next_phase = if action_count.current > 0 {
                        &CombatState::SelectionSkill
                    } else {
                        &default_state
                    };
                }
            }
            // (CombatState::SelectionTarget, CombatState::default())
            (CombatState::SelectionTarget, CombatState::SelectionCaster) => {
                // If there is still some action left for the current caster,
                // skip SelectionCaster (The previous will still have the comp `Selected`)
                let last_action = combat_panel.history.last().unwrap();
                let mut action_count = selected_unit_query
                    .get_component_mut::<ActionCount>(last_action.caster)
                    .unwrap();

                action_count.current -= 1;
                info!("action left: {}", action_count.current);

                next_phase = if action_count.current > 0 {
                    info!("S.Target to S.Caster bypass to S.Skills");
                    &CombatState::SelectionSkill
                } else {
                    &default_state
                };
                // in SelectionSkill we can click another caster to switch
            }
            // --- End of Turn ---
            (_, CombatState::RollInitiative) => {
                // TODO: Warning if there is still action left
                // FIXME: this is a safeguard preventing from double click the `end_of_turn` (wasn't a pb back there)
                if combat_panel.history.len() == 0 {
                    info!("End of Turn - Refused (no action)");
                    continue;
                }
                // remove `Selected` from the last potential selected
                // DOC: will trigger all RemovedComponent queries
                if let Ok((selected, mut action_count)) = selected_unit_query.get_single_mut() {
                    commands.entity(selected).remove::<Selected>();
                    action_count.current = action_count.base;
                }
                // remove all `Targeted`
                for (targeted, _) in targeted_unit_query.iter() {
                    commands.entity(targeted).remove::<Targeted>();
                }
                info!("End of Turn - Accepted");
            }
            (CombatState::RollInitiative, CombatState::ExecuteSkills) => {
                // -----------------------------------------------
                // REFACTOR: Move these ui lines somewhere else -> [[combat::phases::phase_transition()]]
                // IDEA: Reset or just push infinitly ?
                let mut actions_logs_text = actions_logs_query.single_mut();

                actions_logs_text.sections[0].value =
                    String::from("---------------\nActions Logs:");
                // -----------------------------------------------
            }
            // --- New Turn ---
            // replace SelectionCaster by the default()
            (CombatState::ExecuteSkills, CombatState::SelectionCaster) => {
                // IDEA: add this history into a full-log to permit the player to see it.

                // -----------------------------------------------
                // REFACTOR: Move these ui related lines somewhere else
                // REFACTOR: Abstraction Needed
                // Save the Sorted Initiative Action Historic
                let action_displayer_text = action_displayer_query.single();
                let mut last_action_displayer_text = last_action_displayer_query.single_mut();

                last_action_displayer_text.sections[0].value = action_displayer_text.sections[0]
                    .value
                    .replace("Actions:", "Last Turn Actions:");

                // -----------------------------------------------

                // Reset the action history
                combat_panel.history = Vec::new();

                // Reset all ActionCounter/Limit
                for mut action_count in combat_unit.iter_mut() {
                    action_count.current = action_count.base;
                }
            }
            _ => {}
        }

        info!(
            "Phase: {:?} to {:?} (was requested: {:?})",
            combat_panel.phase.clone(),
            next_phase.clone(),
            phase_requested.clone(),
        );
        combat_panel.phase = next_phase.clone();
    }
}

// ----------- Phase Actions -----------

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

// TODO: ShouldHave - Display mutable change (dmg, heal) (on the field)

/// # Note
///
/// DOC
pub fn execute_alteration(
    // mut execute_alteration_event: EventReader<ExecuteAlterationEvent>,
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

    mut transition_phase_event: EventWriter<TransitionPhaseEvent>,
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
        for alteration in alterations.iter_mut() {
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
                    // if alteration.initiative != 0 {
                    //     initiative.0 *= alteration.initiative;
                    // }
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

    transition_phase_event.send(TransitionPhaseEvent(CombatState::SelectionCaster));
}

pub fn observation() {
    // println!("Now it's your turn...")
}

/// Roll for each entity a d100 ranged into +-20 initiative
/// Also Display the final score
///
/// Sort the result in a nice table
/// In case of egality: pick the higher initiative boyo to be on top
pub fn roll_initiative(
    npc_query: Query<(&Initiative, &Alterations), With<NPC>>,
    mut combat_panel_query: Query<&mut CombatPanel>,

    mut transition_phase_event: EventWriter<TransitionPhaseEvent>,
) {
    let mut combat_panel = combat_panel_query.single_mut();

    let mut initiatives: Vec<Action> = Vec::new();

    for action in combat_panel.history.iter_mut() {
        let caster = action.caster;
        // REFACTOR: how the initiative is calculated
        let skill_init = action.skill.initiative.clone();

        match npc_query.get(caster) {
            Err(e) => warn!("Invalid Caster are in the History: {}", e),
            Ok((npc_init, alterations)) => {
                let mut current_init = npc_init.0;

                // ---- Alterations Rules ----
                for alteration in alterations.iter() {
                    match alteration.action {
                        AlterationAction::StatsFlat => {
                            current_init += alteration.initiative;
                        }
                        _ => {}
                    }
                }
                // ---- Calculus ----

                let npc_number = if current_init - 20 <= 0 {
                    rand::thread_rng().gen_range(0..current_init + 20)
                } else if current_init == 100 {
                    100
                } else if current_init + 20 >= 100 {
                    rand::thread_rng().gen_range(current_init - 20..100)
                } else {
                    rand::thread_rng().gen_range(current_init - 20..npc_init.0 + 20)
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

    // info!("DEBUG: history: {:?}", combat_panel.history);

    transition_phase_event.send(TransitionPhaseEvent(CombatState::ExecuteSkills));
}

pub fn execution_phase(
    mut combat_panel_query: Query<&mut CombatPanel>,

    mut execute_skill_event: EventWriter<ExecuteSkillEvent>,
    mut transition_phase_event: EventWriter<TransitionPhaseEvent>,
) {
    let mut combat_panel = combat_panel_query.single_mut();

    for Action {
        caster,
        skill,
        targets,
        initiative: _,
    } in combat_panel.history.iter()
    {
        match targets {
            None => warn!(
                "A Skill without any target ! \n caster: {:?} skill: {:?}",
                caster, skill
            ),
            Some(targets) => {
                for target in targets {
                    // we will do a verification anyway (skill's hp_cost)
                    // in the event handler
                    // to control that the caster is alive at the time of the execution
                    execute_skill_event.send(ExecuteSkillEvent {
                        skill: skill.clone(),
                        caster: *caster,
                        target: *target,
                    });

                    // should be in order
                    for combo_skill in skill.skills_queue.iter() {
                        execute_skill_event.send(ExecuteSkillEvent {
                            skill: combo_skill.clone(),
                            caster: *caster,
                            // All skills in the queue will be directed to the same target
                            target: *target,
                        });
                    }
                }
            }
        }
    }

    transition_phase_event.send(TransitionPhaseEvent(CombatState::default()));
}
