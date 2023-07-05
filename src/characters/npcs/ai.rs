//! This module will repertoriate all system which directly determine
//! AI Aspect

use bevy::prelude::*;
use rand::seq::SliceRandom;

use crate::combat::{
    phases::TransitionPhaseEvent, skills::TargetOption, Action, ActionCount, CombatResources,
    CombatState, CurrentAlterations, InCombat, Recruted, Skills,
};

pub fn ai_decision_making(
    mut combat_resources: ResMut<CombatResources>,

    mut enemies_mut_query: Query<
        (
            Entity,
            &Skills,
            &CurrentAlterations,
            &mut ActionCount,
            &Name,
        ),
        (Without<Recruted>, With<InCombat>),
    >,
    enemies_immutable_query: Query<Entity, (Without<Recruted>, With<InCombat>)>,
    allies_query: Query<Entity, (With<Recruted>, With<InCombat>)>,

    mut transition_phase_event: EventWriter<TransitionPhaseEvent>,
) {
    info!("Strategic AI Assessment...");

    let mut rng = rand::thread_rng();

    // let mut enemies_combinations = enemies_mut_query.iter_combinations_mut();
    // while let Some(
    //     [(enemy1, skills1, _alterations1, mut action_count1), (enemy2, skills2, _alterations2, mut action_count2)],
    // ) = enemies_combinations.fetch_next()
    // {
    //     // iterate over all possible combination without repetition
    // }

    for (caster, skills, _alterations, mut action_count, name) in enemies_mut_query.iter_mut() {
        // info!("{} has {} actions to decide", name, action_count.current);
        while action_count.current > 0 {
            // Decision
            if skills.len() == 0 {
                warn!("{} doesn't have any skill", name);
                action_count.current = 0;
                break;
            }
            // let random_index = rng.gen_range(0..skills.len());
            // let skill = skills[random_index];
            let skill = skills.choose(&mut rng).unwrap();
            
            // info!("{} has chosen {:?}", name, skill);

            let targets: Vec<Entity> = match skill.target_option {
                TargetOption::OneSelf => vec![caster],
                TargetOption::Ally(target_number) => {
                    let mut targets = Vec::new();
                    for _ in 0..target_number {
                        let mut allies = Vec::new();
                        for ally in enemies_immutable_query.iter() {
                            allies.push(ally)
                        }
                        let target = allies.choose(&mut rng).unwrap();

                        // let potential_targets =
                        //     enemies_immutable_query.iter().collect::<Vec<Entity>>();
                        // let target = potential_targets.choose(&mut rng).unwrap();

                        // let target = enemies_immutable_query
                        //     .iter()
                        //     .collect::<Vec<Entity>>()
                        //     .choose(&mut rng)
                        //     .unwrap();

                        targets.push(*target);
                    }
                    targets
                }
                TargetOption::Enemy(target_number) => {
                    let mut targets = Vec::new();
                    for _ in 0..target_number {
                        let potential_targets = allies_query.iter().collect::<Vec<Entity>>();
                        let target = potential_targets.choose(&mut rng).unwrap();
                        targets.push(*target);
                    }
                    targets
                }
                TargetOption::AllyButSelf(target_number) => {
                    let mut targets = Vec::new();
                    for _ in 0..target_number {
                        let mut allies = Vec::new();
                        for ally in enemies_immutable_query.iter() {
                            allies.push(ally)
                        }
                        let target = allies.choose(&mut rng).unwrap();

                        if *target != caster {
                            targets.push(*target);
                        }
                        // else unlucky (could lead to `Some(vec![])` but it's ok.)
                    }
                    targets
                }
                TargetOption::AllAlly => {
                    let mut allies = Vec::new();
                    for ally in enemies_immutable_query.iter() {
                        allies.push(ally)
                    }
                    allies
                }
                TargetOption::AllEnemy => allies_query.iter().collect::<Vec<Entity>>(),
                TargetOption::All => {
                    // let mut targets: Vec<Entity> = enemies_immutable_query.iter().collect::<Vec<Entity>>();
                    let mut targets: Vec<Entity> = Vec::new();
                    for ally in enemies_immutable_query.iter() {
                        targets.push(ally)
                    }
                    let allies = allies_query.iter().collect::<Vec<Entity>>();
                    targets.extend_from_slice(&allies);

                    targets
                }
            };

            // info!("Targeted by {}: {:?}", name, targets);

            let action = Action::new(caster, skill.clone(), Some(targets));
            combat_resources.history.push(action);

            action_count.current -= 1;
        }
    }

    transition_phase_event.send(TransitionPhaseEvent(CombatState::RollInitiative));
}
