use bevy::prelude::*;
use rand::Rng;

use crate::{
    combat::{stats::Initiative, Action, CombatPanel, CombatState},
    npc::NPC,
    ui::player_interaction::ExecuteSkillEvent,
};

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
        })
    }

    // IDEA: add this history into a log to permit the player to see it.

    // Reset the action history
    combat_panel.history = Vec::new();

    // TODO: Go to Observation
    combat_panel.phase = CombatState::SelectionCaster;
}
