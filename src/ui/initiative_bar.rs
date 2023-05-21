//! Display the Initiative Vertical Bar
//! All Set action and interaction systems

use bevy::prelude::*;

use crate::{combat::CombatPanel, ui::combat_panel::ActionDisplayer};

/// Disables empty action,
/// (invisible == disable)
///
/// Prevents checking a index in the action list.
pub fn action_visibility(
    combat_panel_query: Query<&CombatPanel, Changed<CombatPanel>>,
    mut action_query: Query<(&ActionDisplayer, &mut Visibility, &Children)>,

    mut text_query: Query<&mut Text>,
    caster_name_query: Query<&Name>,
) {
    if let Ok(combat_panel) = combat_panel_query.get_single() {
        for (action_number, mut visibility, children) in action_query.iter_mut() {
            let old_visibility = visibility.clone();

            let mut text = text_query.get_mut(children[0]).unwrap();

            *visibility = if action_number.0 < combat_panel.history.len() {
                let caster_name = caster_name_query
                    .get(combat_panel.history[action_number.0].caster)
                    .unwrap();
                text.sections[0].value = caster_name.to_string();

                // --- Visibility ---
                Visibility::Inherited
            } else {
                // useless --vv
                text.sections[0].value = "None".to_string();
                Visibility::Hidden
            };

            // --- Logs ---
            if old_visibility != *visibility {
                // DEBUG: Actions' Visibility switcher
                info!(
                    "action °{} visibility switch: {:?}",
                    action_number.0, *visibility
                );
            }
        }
    }
}

// TODO: Interaction with action in the Initiative Bar
