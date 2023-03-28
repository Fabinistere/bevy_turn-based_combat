use bevy::prelude::*;
use rand::Rng;

use crate::{
    combat::{stats::Initiative, Action, CombatPanel, CombatState},
    npc::NPC,
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
        let skill_init = &action.skill.initiative;

        match npc_query.get(caster) {
            Err(e) => warn!("Invalid Caster are in the History: {}", e),
            Ok(npc_init) => {
                let npc_number = if npc_init.0 - 20 <= 0 {
                    rand::thread_rng().gen_range(0..npc_init.0 + 20 + skill_init)
                } else if npc_init.0 == 100 {
                    100
                } else if npc_init.0 + 20 >= 100 {
                    rand::thread_rng().gen_range(npc_init.0 - 20..100 + skill_init)
                } else {
                    rand::thread_rng().gen_range(npc_init.0 - 20..npc_init.0 + 20 + skill_init)
                };

                // insert these number in a vector
                action.initiative = npc_number;
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
