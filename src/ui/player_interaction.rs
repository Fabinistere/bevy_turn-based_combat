//! All dialog method handler related with the player input directly

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    window::PrimaryWindow,
};

use crate::{
    combat::{
        phases::TransitionPhaseEvent,
        skills::{Skill, TargetSide},
        Action, ActionCount, CombatPanel, CombatState,
    },
    constants::ui::dialogs::*,
    ui::{
        combat_panel::SkillDisplayer,
        combat_system::{Selected, Targeted},
    },
};

/* -------------------------------------------------------------------------- */
/*                          ----- UI Components -----                         */
/* -------------------------------------------------------------------------- */

pub const SPRITE_SIZE: (f32, f32) = (25.0, 40.0);

#[derive(Component)]
pub struct Hoverable;
// {
//     hovered: bool
// }

#[derive(Component)]
pub struct Hovered;

#[derive(Component)]
pub struct Clickable;

#[derive(Component)]
pub struct Clicked;

#[derive(Component)]
pub struct Draggable;
// {
//     pub dragged: bool,
//     pub dropped: bool,
// }

#[derive(Component)]
pub struct Dragged;
// old_z

#[derive(Component)]
pub struct Dropped;

#[derive(Component)]
pub struct SpriteSize {
    pub width: f32,
    pub height: f32,
}

/* -------------------------------------------------------------------------- */
/*                        ----- Global UI systems -----                       */
/* -------------------------------------------------------------------------- */

/// Change color depending of Interaction
///
/// Does not affect Skill Button
/// (color management is different: if no action no color.)
pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>, Without<Skill>),
    >,
) {
    for (interaction, mut color, _children) in &mut interaction_query {
        // let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

// # Note
//
// TODO: feature - can drag unit just to cancel the click
// avoid missclick by dragging
//
// TODO: feature - Skill dropped
// To a possible target: Confirm
// To something else: Cancel (or just back to skill clicked)

// # Note
//
// TODO: feature - Hover Unit - Preview Combat Page

#[derive(Component, Default)]
pub struct ScrollingList {
    position: f32,
}

pub fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for (mut scrolling_list, mut style, parent, list_node) in &mut query_list {
            let items_height = list_node.size().y;
            let container_height = query_node.get(parent.get()).unwrap().size().y;

            let max_scroll = (items_height - container_height).max(0.);

            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };

            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.position.top = Val::Px(scrolling_list.position);
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                       ----- Specific UI systems -----                      */
/* -------------------------------------------------------------------------- */

/// Adds the Component 'Clicked' to a valid Entity
pub fn select_unit_by_mouse(
    mut commands: Commands,

    primary_query: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    // With<MainCamera>
    camera_q: Query<(&Camera, &GlobalTransform)>,
    buttons: Res<Input<MouseButton>>,

    selectable_unit_query: Query<
        (Entity, &Transform, &SpriteSize, &Name),
        (With<Clickable>, Without<Clicked>),
    >,
    // mut update_unit_selected_event: EventWriter<UpdateUnitSelectedEvent>,
) {
    let Ok(primary) = primary_query.get_single() else {
        return;
    };
    let (camera, camera_transform) = camera_q.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = primary
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        // eprintln!("World coords: {}/{}", world_position.x, world_position.y);
        if buttons.just_pressed(MouseButton::Left) {
            for (unit, transform, sprite_size, _name) in selectable_unit_query.iter() {
                let half_width = (sprite_size.width * transform.scale.x) / 2.0;
                let half_height = (sprite_size.height * transform.scale.y) / 2.0;

                if transform.translation.x - half_width < world_position.x
                    && transform.translation.x + half_width > world_position.x
                    && transform.translation.y - half_height < world_position.y
                    && transform.translation.y + half_height > world_position.y
                {
                    // info!("{} clicked", _name);
                    commands.entity(unit).insert(Clicked);
                    // v-- instead of --^
                    // update_unit_selected_event.send(UpdateUnitSelectedEvent(unit));
                }
            }
        }
    } else {
        // cursor is not inside the window
    }
}

/// Action for each Interaction of the skill button
///
/// # Note
///
/// - skill_color(): Skill color will update at caster/action_count change
pub fn select_skill(
    mut interaction_query: Query<
        (&Interaction, &Skill, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>, With<SkillDisplayer>),
    >,

    mut text_query: Query<&mut Text>,

    mut combat_panel_query: Query<(Entity, &mut CombatPanel)>,

    unit_selected_query: Query<(Entity, &Name, &ActionCount), With<Selected>>,
    mut transition_phase_event: EventWriter<TransitionPhaseEvent>,
) {
    for (interaction, skill, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();

        // if this system can run
        // we are in SelectionSkill or SelectionTarget
        // so there is a selected unit.
        // FIXME: Crash - Esc bug, after cancel an action but still in selectionSkill with no action left
        let (caster, _caster_name, action_count) = unit_selected_query.single();

        match *interaction {
            Interaction::Clicked => {
                // <=
                if action_count.current == 0 {
                    text.sections[0].value = String::from("0ac Left");
                    *color = INACTIVE_BUTTON.into();
                    continue;
                }

                let (_, mut combat_panel) = combat_panel_query.single_mut();

                // BUG: XXX: Weird "Bug" Event/GameState related handle
                if let Some(last_action) = combat_panel.history.last() {
                    if last_action.skill == skill.clone() && last_action.targets == None {
                        // warn!("Same Skill Selected Event handled twice");
                        continue;
                    }
                }

                *color = PRESSED_BUTTON.into();

                // Change last action saved to the new skill selected
                if combat_panel.phase == CombatState::SelectionTarget {
                    info!("Skill changed for {}", skill.name);
                    // we already wrote the waiting skill in the actions history
                    // cause we're in the TargetSelection phase

                    let last_action = combat_panel.history.last_mut().unwrap();
                    last_action.skill = skill.clone();
                    last_action.targets = if skill.target_side == TargetSide::OneSelf {
                        transition_phase_event.send(TransitionPhaseEvent(CombatState::default()));
                        Some(vec![caster])
                    } else {
                        // and we're still in TargetSelection phase
                        None
                    };

                    // info!("DEBUG: action = {} do {} to None", caster_name, skill.name);

                    // info!("rewrite last action");
                } else {
                    transition_phase_event.send(TransitionPhaseEvent(CombatState::SelectionTarget));

                    let action = Action::new(caster, skill.clone(), None);
                    combat_panel.history.push(action);

                    // info!("DEBUG: action = {} do {} to None", _caster_name, skill.name);
                    // info!("new action");
                }

                let display = skill.name.replace("a", "o").replace("A", "O");
                text.sections[0].value = display;

                info!("Skill {} selected", skill.name);
            }
            Interaction::Hovered => {
                // TODO: feature - Hover Skill - Preview possible Target

                text.sections[0].value = skill.name.clone();
                *color = if action_count.current == 0 {
                    INACTIVE_HOVERED_BUTTON.into()
                } else {
                    HOVERED_BUTTON.into()
                };
            }
            Interaction::None => {
                text.sections[0].value = skill.name.clone();

                *color = if action_count.current == 0 {
                    INACTIVE_BUTTON.into()
                } else {
                    NORMAL_BUTTON.into()
                };
            }
        }
    }
}

#[derive(Component)]
pub struct EndOfTurnButton;

/// # Note
///
/// TODO: Check ActionCount ? x
/// FIXME: End of turn in SelectionSkill: trigger a double press
/// @see [`ui::player_interaction::confirm_action_button()`] to check: correct target number
pub fn end_of_turn_button(
    mut interaction_query: Query<
        (&Interaction, &Children),
        (Changed<Interaction>, With<Button>, With<EndOfTurnButton>),
    >,

    mut text_query: Query<&mut Text>,

    mut combat_panel_query: Query<(Entity, &mut CombatPanel)>,
    mut transition_phase_event: EventWriter<TransitionPhaseEvent>,
) {
    for (interaction, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                let (_, mut combat_panel) = combat_panel_query.single_mut();

                if let Some(last_action) = combat_panel.history.pop() {
                    // TODO: Check correct target number
                    // atm we can partially confirm an action by pressing "end_of_turn"
                    if last_action.targets != None {
                        // reput the last_action in the pool
                        combat_panel.history.push(last_action);
                    }
                } else {
                    // allow pass with no action in the history
                }

                // Pressed
                info!("End of Turn - Requested");

                transition_phase_event.send(TransitionPhaseEvent(CombatState::RollInitiative));

                text.sections[0].value = "Next".to_string();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Can't Undo".to_string();
            }
            Interaction::None => {
                text.sections[0].value = "End of Turn".to_string();
            }
        }
    }
}

/// If the user press 'esc',
/// depending of the phase we're in,
/// will undo the previous input (predicted, not real undo)
pub fn cancel_last_input(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,

    selected_unit_query: Query<(Entity, &Name), With<Selected>>,
    // , With<Selected>
    mut caster_query: Query<(Entity, &mut ActionCount)>,

    mut combat_panel_query: Query<&mut CombatPanel>,
    mut transition_phase_event: EventWriter<TransitionPhaseEvent>,
) {
    let mut combat_panel = combat_panel_query.single_mut();

    if keyboard_input.just_pressed(KeyCode::Escape) {
        info!("Esc in {}", combat_panel.phase);
        match combat_panel.phase {
            CombatState::SelectionSkill => {
                // FIXME: Smashing esc can skip a beat and crash here
                let (selected, name) = selected_unit_query.single();

                info!("{} was selected", name);

                commands.entity(selected).remove::<Selected>();
                transition_phase_event.send(TransitionPhaseEvent(CombatState::SelectionCaster));
            }
            CombatState::SelectionCaster | CombatState::SelectionTarget => {
                // Remove last targeted and modify the last action
                match combat_panel.history.pop() {
                    None => {
                        if combat_panel.phase == CombatState::SelectionTarget {
                            warn!("In TargetSelectionPhase, it should have at least one action");
                            // DEBUG: Error Handle
                            transition_phase_event
                                .send(TransitionPhaseEvent(CombatState::SelectionCaster));
                        } else {
                            // Nothing to undo
                        }
                    }
                    Some(ref mut last_action) => {
                        // give the last_action.caster the selected component
                        if let Ok((selected, name)) = selected_unit_query.get_single() {
                            if selected != last_action.caster {
                                commands.entity(selected).remove::<Selected>();
                                info!(
                                    "{}:{:?} was selected over our last caster {:?}",
                                    name, selected, last_action.caster
                                );
                            }
                        }
                        commands.entity(last_action.caster).insert(Selected);
                        info!("{:?} is now selected", last_action.caster);

                        match &mut last_action.targets {
                            None => {
                                // undo the skill selection and go back to SelectionSkill (handled in `phase_transition()`)
                                // ^^^^--- by not placing back the action in the history

                                transition_phase_event
                                    .send(TransitionPhaseEvent(CombatState::SelectionSkill));
                            }
                            Some(ref mut targets) => {
                                // remove last targeted
                                let old_target = targets.pop().unwrap();
                                commands.entity(old_target).remove::<Targeted>();
                                if targets.len() == 0 {
                                    last_action.targets = None;
                                }

                                if combat_panel.phase == CombatState::SelectionCaster {
                                    // Give back the action
                                    let mut action_count = caster_query
                                        .get_component_mut::<ActionCount>(last_action.caster)
                                        .unwrap();
                                    action_count.current += 1;
                                    info!("action left: {}", action_count.current);

                                    if last_action.skill.target_side == TargetSide::OneSelf {
                                        transition_phase_event.send(TransitionPhaseEvent(
                                            CombatState::SelectionSkill,
                                        ));
                                    } else {
                                        transition_phase_event.send(TransitionPhaseEvent(
                                            CombatState::SelectionTarget,
                                        ));
                                        combat_panel.history.push(last_action.clone());
                                    }
                                } else {
                                    combat_panel.history.push(last_action.clone());

                                    // In SelectionTarget, the action_count should be correct.
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

// TODO: equip stuffs
