use bevy::prelude::*;

use crate::{
    combat::{
        phases::TransitionPhaseEvent, skills::TargetOption, CombatPanel, CombatState, InCombat,
        Team,
    },
    ui::player_interaction::Clicked,
};

/* -------------------------------------------------------------------------- */
/*                                UI Components                               */
/* -------------------------------------------------------------------------- */

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
pub struct CombatStateDisplayer;

/// Current Action History
#[derive(Resource, Debug, Reflect, Deref, DerefMut, Clone)]
pub struct ActionHistory(pub String);

/// Points to the UI Text which display Current Action History
#[derive(Component)]
pub struct ActionHistoryDisplayer;

/// Last turn Action History
#[derive(Resource, Debug, Reflect, Deref, DerefMut, Clone)]
pub struct LastTurnActionHistory(pub String);

/// Points to the UI Text which display Last Turn Action History
#[derive(Component)]
pub struct LastActionHistoryDisplayer;

/// Logs Action History
#[derive(Resource, Debug, Reflect, Deref, DerefMut, Clone)]
pub struct ActionsLogs(pub String);

/// Points to the UI Text which display Last Turn Actions Precise Logs
#[derive(Component)]
pub struct ActionsLogsDisplayer;

/// DOC
pub struct UpdateUnitSelectedEvent(pub Entity);

/// DOC
pub struct UpdateUnitTargetedEvent(pub Entity);

/* -------------------------------------------------------------------------- */
/*                                 UI Systems                                 */
/* -------------------------------------------------------------------------- */

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

/* -------------------------------------------------------------------------- */
/*                                 UI Updates                                 */
/* -------------------------------------------------------------------------- */

/// Event Handler of UpdateUnitSelectedEvent
///
/// There can only be one entity selected.
/// After completed a action with one, you can reselect it.
///
/// # Note
///
/// FIXME: Multiple Entity can be selected if clicked simultaneous
/// REFACTOR: Directly Manage Clicked Entity in the update systems (instead of event)
pub fn update_selected_unit(
    mut commands: Commands,

    mut event_query: EventReader<UpdateUnitSelectedEvent>,

    unselected_unit_query: Query<(Entity, &Name), (Without<Selected>, With<InCombat>)>,
    selected_unit_query: Query<(Entity, &Name), (With<Selected>, With<InCombat>)>,

    mut transition_phase_event: EventWriter<TransitionPhaseEvent>,
) {
    for event in event_query.iter() {
        match unselected_unit_query.get(event.0) {
            Err(e_unselect) => match selected_unit_query.get(event.0) {
                Err(e_select) => warn!(
                    "The entity selected is invalid: {:?} --//-- {:?}",
                    e_unselect, e_select
                ),
                // Don't change the entity selected
                // TOTEST: This case only happens when the player reselect a entity by his.her will
                Ok(_) => {
                    transition_phase_event.send(TransitionPhaseEvent(CombatState::SelectionSkill));
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

                transition_phase_event.send(TransitionPhaseEvent(CombatState::SelectionSkill));
            }
        }
    }
}

/// Event Handler of UpdateUnitSelectedEvent.
/// Will accept or not a target depending of the skill currently selected.
///
/// # Note
///
/// REFACTOR: maybe merge Targeted with Selected
/// Differentiation only when selecting a skill
pub fn update_targeted_unit(
    mut commands: Commands,
    mut combat_panel: ResMut<CombatPanel>,

    mut event_query: EventReader<UpdateUnitTargetedEvent>,

    unit_selected_query: Query<(Entity, &Team), With<Selected>>,
    combat_unit_query: Query<(Entity, &Name, &Team), With<InCombat>>,

    mut transition_phase_event: EventWriter<TransitionPhaseEvent>,
) {
    for event in event_query.iter() {
        match combat_unit_query.get(event.0) {
            Err(e) => warn!("The entity targeted is invalid: {:?}", e),
            Ok((character, target_name, target_team)) => {
                let last_action = combat_panel.history.last_mut().unwrap();

                // Is it a correct target ?
                match last_action.skill.target_option {
                    TargetOption::Ally(_) => {
                        let (_, caster_team) = unit_selected_query.single();
                        if target_team != caster_team {
                            info!("The target is not an ally");
                            continue;
                        }
                    }
                    TargetOption::Enemy(_) => {
                        let (_, caster_team) = unit_selected_query.single();
                        if target_team == caster_team {
                            info!("The target is not an enemy");
                            continue;
                        }
                    }
                    TargetOption::AllyButSelf(_) => {
                        let (caster, caster_team) = unit_selected_query.single();
                        if target_team != caster_team || character == caster {
                            info!("The target is not an ally or is the caster");
                            continue;
                        }
                    }
                    _ => {}
                }

                commands.entity(character).insert(Targeted);
                info!("{} targeted", target_name);

                // Possibility to target multiple depending to the skill selected
                last_action.targets = match last_action.targets.clone() {
                    None => {
                        // Number of target = max targetable
                        match last_action.skill.target_option {
                            TargetOption::Ally(1)
                            | TargetOption::Enemy(1)
                            | TargetOption::OneSelf => transition_phase_event
                                .send(TransitionPhaseEvent(CombatState::default())),
                            _ => {}
                        }
                        Some(vec![character])
                    }
                    Some(mut targets) => {
                        match last_action.skill.target_option {
                            TargetOption::Ally(number)
                            | TargetOption::Enemy(number)
                            | TargetOption::AllyButSelf(number) => {
                                // Only work if we can target muiltiple times one entity
                                // or if the number of available target is < number asked
                                // TODO: can target less if = the max possible
                                if targets.len() < number {
                                    targets.push(character);
                                    if targets.len() == number {
                                        transition_phase_event
                                            .send(TransitionPhaseEvent(CombatState::default()));
                                    }
                                } else if targets.len() > number {
                                    warn!(
                                        "Error! The number of target is exceeded {}/{:?}",
                                        targets.len(),
                                        last_action.skill.target_option
                                    );
                                    while targets.len() > number {
                                        commands
                                            .entity(targets.pop().unwrap())
                                            .remove::<Targeted>();
                                    }
                                }
                            }
                            // managed by phase_transition() or select_skill()
                            TargetOption::OneSelf
                            | TargetOption::AllAlly
                            | TargetOption::AllEnemy
                            | TargetOption::All => {}
                        }
                        Some(targets)
                    }
                };
            }
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                                   UI Logs                                  */
/* -------------------------------------------------------------------------- */

/// Display the current phase
///
/// # Note
///
/// DEBUG: update_combat_phase_displayer()
pub fn update_combat_phase_displayer(
    combat_panel: Res<CombatPanel>,
    mut combat_state_displayer_query: Query<&mut Text, With<CombatStateDisplayer>>,
) {
    // FIXME: don't update afterwards if wasn't on the Logs Panel
    if combat_panel.is_changed() {
        if let Ok(mut text) = combat_state_displayer_query.get_single_mut() {
            text.sections[0].value = format!("Combat Phase: {}", combat_panel.phase);
        }
    }
}

/// Display all combat logs
///
/// # Note
///
/// DEBUG: actions_logs_displayer()
/// IDEA: CouldHave - Character's Event (Died, killed thingy, etc)
pub fn actions_logs_displayer(
    actions_logs: Res<ActionsLogs>,
    mut actions_logs_query: Query<
        &mut Text,
        (
            With<ActionsLogsDisplayer>,
            Without<LastActionHistoryDisplayer>,
            Without<ActionHistoryDisplayer>,
        ),
    >,
) {
    // FIXME: don't update afterwards if wasn't on the Logs Panel
    // fixed by: || in_enter(UILocation::LogsPanel)
    if actions_logs.is_changed() {
        if let Ok(mut actions_logs_text) = actions_logs_query.get_single_mut() {
            actions_logs_text.sections[0].value = actions_logs.clone().0;
        }
    }
}

/// Format the modified action
///
/// # Note
///
/// IDEA: Atm each turn it resets
pub fn current_action_formater(
    combat_panel: Res<CombatPanel>,
    mut action_history: ResMut<ActionHistory>,

    combat_units_query: Query<(Entity, &Name), With<InCombat>>,
) {
    if combat_panel.is_changed() {
        action_history.0 = String::from("---------------\nActions:");

        for (number, action) in combat_panel.history.iter().enumerate() {
            if let Ok((_, caster_name)) = combat_units_query.get(action.caster) {
                let mut targets_name = String::new();
                match &action.targets {
                    None => targets_name = "None".to_string(),
                    Some(targets) => {
                        for (i, target) in targets.iter().enumerate() {
                            if targets.len() > 1 && i != 0 {
                                targets_name.push_str(" and ");
                            }
                            match combat_units_query.get(*target) {
                                Err(_) => targets_name.push_str("Target Err"),
                                Ok((_, name)) => targets_name.push_str(name),
                            }
                        }
                    }
                }

                let action_display = if action.initiative == -1 {
                    format!(
                        "\n{}. {} do {} to {}",
                        number + 1,
                        caster_name,
                        action.skill.name,
                        targets_name
                    )
                } else {
                    format!(
                        "\n{}. {}: {} do {} to {}",
                        number + 1,
                        action.initiative,
                        caster_name,
                        action.skill.name,
                        targets_name
                    )
                };

                action_history.push_str(&action_display);
            }
        }
    }
}

/// Display all current actions
///
/// # Note
///
/// DEBUG: current_action_displayer()
pub fn current_action_displayer(
    action_history: Res<ActionHistory>,
    mut action_displayer_query: Query<&mut Text, With<ActionHistoryDisplayer>>,
) {
    // FIXME: don't update afterwards if wasn't on the Logs Panel
    // fixed by: || in_enter(UILocation::LogsPanel)
    if action_history.is_changed() {
        if let Ok(mut action_displayer_text) = action_displayer_query.get_single_mut() {
            action_displayer_text.sections[0].value = action_history.clone().0;
        }
    }
}

/// Display the last turn actions
///
/// # Note
///
/// DEBUG: last_action_displayer()
pub fn last_action_displayer(
    last_action_history: Res<LastTurnActionHistory>,
    mut last_action_displayer_query: Query<&mut Text, With<LastActionHistoryDisplayer>>,
) {
    // FIXME: don't update afterwards if wasn't on the Logs Panel
    // fixed by: || in_enter(UILocation::LogsPanel)
    if last_action_history.is_changed() {
        if let Ok(mut last_action_displayer_text) = last_action_displayer_query.get_single_mut() {
            last_action_displayer_text.sections[0].value = last_action_history.clone().0;
        }
    }
}
