use bevy::prelude::*;

use crate::{
    combat::{
        phases::TransitionPhaseEvent, skills::TargetOption, CombatPanel, CombatState, InCombat,
    },
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
    mut transition_phase_event: EventWriter<TransitionPhaseEvent>,
    // DEBUG DISPLAYER
    // unit_selected_query: Query<(Entity, &Name, &Selected)>,
) {
    for event in event_query.iter() {
        match combat_unit_query.get(event.0) {
            Err(e) => warn!("The entity targeted is invalid: {:?}", e),
            Ok((character, target_name)) => {
                commands.entity(character).insert(Targeted);
                info!("{} targeted", target_name);

                let (_, mut combat_panel) = combat_panel_query.single_mut();
                let last_action = combat_panel.history.last_mut().unwrap();

                // TODO: ?? - impl change target/skill in the Vec<Action>
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
