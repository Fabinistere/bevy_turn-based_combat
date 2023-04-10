use bevy::prelude::*;

use crate::{
    combat::{phases::TransitionPhaseEvent, CombatPanel, CombatState, InCombat},
    constants::ui::dialogs::*,
    ui::player_interaction::Clicked,
};

// ------------------------- UI Components -------------------------

#[derive(Component)]
pub struct ButtonTargeting;

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct Targeted;

#[derive(Component)]
pub struct HpMeter;

#[derive(Component)]
pub struct MpMeter;

#[derive(Component)]
pub struct ActionHistoryDisplayer;

#[derive(Component)]
pub struct LastActionHistoryDisplayer;

#[derive(Component)]
pub struct ActionsLogs;

/// DOC
pub struct UpdateUnitSelectedEvent(pub Entity);

/// DOC
pub struct UpdateUnitTargetedEvent(pub Entity);

// -------------------------- UI Systems --------------------------

/// # Note
pub fn caster_selection(
    mut commands: Commands,

    selectable_unit_query: Query<(Entity, &Name), (With<Clicked>, With<InCombat>)>,

    mut update_unit_selected_event: EventWriter<UpdateUnitSelectedEvent>,
) {
    for (entity, _name) in selectable_unit_query.iter() {
        update_unit_selected_event.send(UpdateUnitSelectedEvent(entity));

        commands.entity(entity).remove::<Clicked>();
        // info!("DEBUG: {} remove clicked to be selected", _name);
    }
}

/// # Note
pub fn target_selection(
    mut commands: Commands,

    targetable_unit_query: Query<(Entity, &Name), (With<Clicked>, With<InCombat>)>,

    mut update_unit_targeted_event: EventWriter<UpdateUnitTargetedEvent>,
) {
    for (entity, _name) in targetable_unit_query.iter() {
        update_unit_targeted_event.send(UpdateUnitTargetedEvent(entity));

        commands.entity(entity).remove::<Clicked>();
        // info!("DEBUG: {} remove clicked to be targeted", _name);
    }
}

#[deprecated(
    since = "0.0.3",
    note = "please use `select_unit_by_mouse` and `target_selection` instead"
)]
pub fn target_random_system(
    mut commands: Commands,

    mut button_system: Query<
        (Entity, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<ButtonTargeting>),
    >,

    combat_unit_query: Query<(Entity, &Name), (With<InCombat>, Without<Targeted>)>,
    targeted_unit: Query<Entity, With<Targeted>>,

    mut update_unit_targeted_event: EventWriter<UpdateUnitTargetedEvent>,
) {
    for (_button, interaction, mut color) in &mut button_system {
        match *interaction {
            Interaction::Clicked => {
                for (npc, _name) in combat_unit_query.iter() {
                    // target the first one on the list
                    if let Ok(targeted) = targeted_unit.get_single() {
                        commands.entity(targeted).remove::<Targeted>();
                    }
                    // DEBUG: TEMPORARY TARGET
                    update_unit_targeted_event.send(UpdateUnitTargetedEvent(npc));

                    break;
                }

                *color = PRESSED_BUTTON.into();
            }
            // TODO: feature - preview
            // Store the previous selected in the temp and restore it when none
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

// -------------------------- UI Updates --------------------------

/// Event Handler of UpdateUnitSelectedEvent
///
/// There can only be one entity selected.
/// After completed a action with one, you can reselect it.
pub fn update_selected_unit(
    mut commands: Commands,

    mut event_query: EventReader<UpdateUnitSelectedEvent>,

    combat_unit_query: Query<(Entity, &Name), (Without<Selected>, With<InCombat>)>,
    selected_unit_query: Query<(Entity, &Name), With<Selected>>,

    mut transition_phase_event: EventWriter<TransitionPhaseEvent>,
) {
    for event in event_query.iter() {
        match combat_unit_query.get(event.0) {
            Err(e_combat) => match selected_unit_query.get(event.0) {
                Err(e_select) => warn!(
                    "The entity selected is invalid: {:?} --//-- {:?}",
                    e_combat, e_select
                ),
                // Don't change the entity selected
                // TOTEST: This case only happens when the player reselect a entity by his.her will
                Ok(_) => {
                    transition_phase_event.send(TransitionPhaseEvent(CombatState::SelectionSkills));
                }
            },
            // Wasn't already selected
            Ok((character, _name)) => {
                commands.entity(character).insert(Selected);
                info!("{} selected", _name);

                // remove from previous entity the selected component
                for (selected, _) in selected_unit_query.iter() {
                    commands.entity(selected).remove::<Selected>();
                }

                transition_phase_event.send(TransitionPhaseEvent(CombatState::SelectionSkills));
            }
        }
    }
}

/// Event Handler of UpdateUnitSelectedEvent
///
/// # Note
///
/// REFACTOR: maybe merge Targeted with Selected
/// Differentiation only when selecting a skill
pub fn update_targeted_unit(
    mut commands: Commands,

    mut event_query: EventReader<UpdateUnitTargetedEvent>,

    combat_unit_query: Query<(Entity, &Name), With<InCombat>>,

    mut combat_panel_query: Query<(Entity, &mut CombatPanel)>,
    // DEBUG DISPLAYER
    // unit_selected_query: Query<(Entity, &Name, &Selected)>,
) {
    for event in event_query.iter() {
        // REFACTOR: ? does this match is mandatory ? can just add Targeted to the unit. XXX
        // same in update_seleted_unit
        match combat_unit_query.get(event.0) {
            Err(e) => warn!("The entity targeted is invalid: {:?}", e),
            Ok((character, target_name)) => {
                commands.entity(character).insert(Targeted);
                info!("{} targeted", target_name);

                let (_, mut combat_panel) = combat_panel_query.single_mut();
                let mut last_action = combat_panel.history.pop().unwrap();

                // TODO: impl change target/skill in the Vec<Action>
                // Possibility to target multiple depending to the skill selected
                last_action.targets = match last_action.targets {
                    None => Some(vec![character]),
                    Some(mut targets) => {
                        if targets.len() < last_action.skill.target_number {
                            targets.push(character);
                        } else if targets.len() > last_action.skill.target_number {
                            // abrsurd, should not happen
                            // FIXME: error Handling -> back to a length acceptable
                            warn!(
                                "The number of target is exceeded {}/{}",
                                targets.len(),
                                last_action.skill.target_number
                            );
                            while targets.len() > last_action.skill.target_number {
                                targets.pop();
                            }
                        }
                        // Number of target = max targetable
                        else {
                            // 'replace' the first one by the newly targeted
                            // ^^^^^^^^^---- Remove the first and push the new one
                            if let Some((first_target, rem_targets)) = targets.split_first_mut() {
                                commands.entity(*first_target).remove::<Targeted>();
                                targets = rem_targets.to_vec();
                                targets.push(character);
                            }
                        }
                        Some(targets)
                    }
                };
                combat_panel.history.push(last_action.clone());
            }
        }
    }
}

// ---------------------------- UI Logs ----------------------------

/// Display the current phase
///
/// # Note
///
/// DEBUG
pub fn update_combat_phase_displayer(
    mut combat_panel_query: Query<
        (Entity, &CombatPanel, &mut Text),
        Or<(Added<CombatPanel>, Changed<CombatPanel>)>,
    >,
) {
    if let Ok((_, combat_panel, mut text)) = combat_panel_query.get_single_mut() {
        let phase_display = format!("Combat Phase: {}", combat_panel.phase);
        text.sections[0].value = phase_display;
    }
}

/// Display the modified action
///
/// # Note
///
/// DEBUG
pub fn last_action_displayer(
    mut combat_panel_query: Query<(Entity, &CombatPanel), Changed<CombatPanel>>,
    unit_combat_query: Query<(Entity, &Name), With<InCombat>>,
    mut action_displayer_query: Query<&mut Text, With<ActionHistoryDisplayer>>,
) {
    if let Ok((_, combat_panel)) = combat_panel_query.get_single_mut() {
        let mut action_displayer_text = action_displayer_query.single_mut();

        let mut history = String::from("---------------\nActions:");
        // println!("Actions:");
        let mut number = 1;
        for action in combat_panel.history.iter() {
            if let Ok((_, caster_name)) = unit_combat_query.get(action.caster) {
                let mut targets_name = String::new();
                match &action.targets {
                    None => targets_name = "None".to_string(),
                    Some(targets) => {
                        for target in targets {
                            if targets.len() > 1 {
                                targets_name.push_str(" and ");
                            }
                            match unit_combat_query.get(*target) {
                                Err(_) => targets_name.push_str("Target Err"),
                                Ok((_, name)) => targets_name.push_str(name),
                            }
                        }
                    }
                }
                let action_display = if action.initiative == -1 {
                    format!(
                        "\n{}. {} do {} to {}",
                        number, caster_name, action.skill.name, targets_name
                    )
                } else {
                    format!(
                        "\n{}. {}: {} do {} to {}",
                        number, action.initiative, caster_name, action.skill.name, targets_name
                    )
                };
                // let action_display = format!(
                //     "{}: {} do {} to {}",
                //     action.initiative, caster_name, action.skill.name, target_name
                // );
                // println!("{}. {}", number, action_display);
                history.push_str(&action_display);
                number += 1;
            }
        }
        action_displayer_text.sections[0].value = history;
    }
}
