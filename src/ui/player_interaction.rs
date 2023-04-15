//! All dialog method handler related with the player input directly

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

use crate::{
    combat::{phases::TransitionPhaseEvent, CombatPanel, CombatState},
    constants::{ui::dialogs::*, HEIGHT, RESOLUTION},
};

use super::combat_system::{Selected, Targeted};

// ----- UI Components -----

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

// ----- Global UI systems -----

/// Change color depending of Interaction
///
/// # Note
///
/// REFACTOR: seperate color management button from specific command button system
pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
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
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Children, &Node)>,
    query_item: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for (mut scrolling_list, mut style, children, uinode) in &mut query_list {
            let items_height: f32 = children
                .iter()
                .map(|entity| query_item.get(*entity).unwrap().size().y)
                .sum();
            let panel_height = uinode.size().y;
            let max_scroll = (items_height - panel_height).max(0.);
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

// ----- Specific UI systems -----

/// Adds the Component 'Clicked' to a valid Entity
pub fn select_unit_by_mouse(
    mut commands: Commands,

    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,

    selectable_unit_query: Query<
        (Entity, &Transform, &SpriteSize, &Name),
        (With<Clickable>, Without<Clicked>),
    >,
    // mut update_unit_selected_event: EventWriter<UpdateUnitSelectedEvent>,
) {
    let window = windows.get_primary().unwrap();

    if let Some(position) = window.cursor_position() {
        if buttons.just_pressed(MouseButton::Left) {
            // info!("({}, {})", position.x, position.y);
            let window_height = HEIGHT;
            let window_width = window_height * RESOLUTION;

            // TODO: Magical Number...
            let transform_height = 100.0;
            let transform_width = 180.0;

            // in translation : 0,0 = center
            // in cursor pos : 0,0 = bottom left

            // in transform: 180 wide ? 100 height
            let ratio_x = -transform_width / window_width;
            let ratio_y = transform_height / window_height;

            for (unit, transform, sprite_size, _name) in selectable_unit_query.iter() {
                // TODO: Too big
                let half_width = (sprite_size.width * transform.scale.x) / 2.0;
                let half_height = (sprite_size.height * transform.scale.y) / 2.0;

                // info!("{} - transform: ({},{}) - half_width: {} - half_height: {}", name, transform.translation.x, transform.translation.y, half_width, half_height);
                // info!("mouse pos: ({}, {})", position.x, position.y);

                // cursor_pos_in_tranform
                let cursor_transform_x = position.x * ratio_x + transform_width / 2.0;
                let cursor_transform_y = position.y * ratio_y - transform_height / 2.0;

                if transform.translation.x - half_width < cursor_transform_x
                    && transform.translation.x + half_width > cursor_transform_x
                    && transform.translation.y - half_height < cursor_transform_y
                    && transform.translation.y + half_height > cursor_transform_y
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

#[derive(Component)]
pub struct EndOfTurnButton;

/// # Note
///
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

                // allow pass with no action in the history
                if let Some(last_action) = combat_panel.history.pop() {
                    // TODO: Check correct target number
                    if last_action.targets != None {
                        // reput the last_action in the pool
                        combat_panel.history.push(last_action);
                    }
                }

                transition_phase_event.send(TransitionPhaseEvent(CombatState::RollInitiative));

                text.sections[0].value = "CAN'T UNDO".to_string();
            }
            Interaction::Hovered => {
                // TODO: feature - Hover Skill - Preview possible Target

                text.sections[0].value = "End of Turn".to_string();
            }
            Interaction::None => {
                text.sections[0].value = "End of Turn".to_string();
            }
        }
    }
}

// TODO: ui feature - pressing `esc` when in SlectionTarget will supp the last entityTargeted from the Targets List

#[derive(Component)]
pub struct ConfirmActionButton;

/// # CONFIRM Action
///
/// ONLY if the player ask to complete this action,
/// and if the action is correct:
///
/// Phase transi to the default Phase (here: SelectionCaster)
///
/// ## Note
///
/// At the moment, you have to press `Confirm Action` before ending your turn.
/// I won't create shortcut,
/// when pressing `EndOfTurn` and when the action seems to be complete,
/// which auto Confirm the pending Action.
///
/// This kind of UI pitfalls will be fixed at the refactor time.
///
/// REFACTOR: UI Actions - Fluidity in UI actions
/// ^^^^^^^^^---- Auto confirmation but can modify previous action with input (ex: `esc`)
///
/// IDEA: bypass the confirmation when targetting for a mono-target action
/// ^^^^^--- find a way to accept that (by options, etc)
///
/// [`ui::player_interaction::confirm_action_button()`]!
#[deprecated(since = "0.0.4", note = "This action is automatic now")]
pub fn confirm_action_button(
    mut interaction_query: Query<
        (&Interaction, &Children),
        (
            Changed<Interaction>,
            With<Button>,
            With<ConfirmActionButton>,
        ),
    >,

    mut text_query: Query<&mut Text>,

    combat_panel_query: Query<&CombatPanel>,
    mut transition_phase_event: EventWriter<TransitionPhaseEvent>,
) {
    for (interaction, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                let combat_panel = combat_panel_query.single();
                // combat_panel.history.len() > 0
                // ^^-- cause this method is only called in SelectionTarget
                //      And on phase: SelectionTarget there will always be at least one action.
                let last_action = combat_panel.history[combat_panel.history.len() - 1].clone();

                let skill = last_action.skill;
                let number_of_targets_required = skill.target_number;
                let current_number_of_targets = last_action.targets.unwrap().len();
                // transi to SelectionCaster only if # target = max possible/# required
                if number_of_targets_required == current_number_of_targets {
                    transition_phase_event.send(TransitionPhaseEvent(CombatState::SelectionCaster));
                    text.sections[0].value = "CONFIRMED".to_string();
                } else {
                    // vvv--- target number is fluide ---vvv
                    // transition_phase_event.send(TransitionPhaseEvent(CombatState::SelectionCaster));
                    text.sections[0].value = format!(
                        "NOT ENOUGH TARGETS: {}/{}",
                        current_number_of_targets, number_of_targets_required
                    );
                }
            }
            Interaction::Hovered => {
                // TODO: feature - Hover Confirm button - Preview the current action (on field)

                text.sections[0].value = "Confirm Action".to_string();
            }
            Interaction::None => {
                text.sections[0].value = "Confirm Action".to_string();
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

    selected_unit_query: Query<Entity, With<Selected>>,

    mut combat_panel_query: Query<&mut CombatPanel>,
    mut transition_phase_event: EventWriter<TransitionPhaseEvent>,
) {
    let mut combat_panel = combat_panel_query.single_mut();

    if keyboard_input.just_pressed(KeyCode::Escape) {
        info!("Esc in {}", combat_panel.phase);
        match combat_panel.phase {
            CombatState::SelectionSkills => {
                let selected = selected_unit_query.single();
                commands.entity(selected).remove::<Selected>();
                transition_phase_event.send(TransitionPhaseEvent(CombatState::SelectionCaster));
            }
            CombatState::SelectionCaster | CombatState::SelectionTarget => {
                // Remove last targeted and modify the last action
                match combat_panel.history.pop() {
                    None => {
                        if combat_panel.phase == CombatState::SelectionTarget {
                            warn!("In TargetSelectionPhase, it should have at least a Target")
                        } else {
                            // Nothing to undo
                        }
                    }
                    Some(ref mut last_action) => {
                        // remove last targeted
                        match &mut last_action.targets {
                            None => {
                                // undo the skill selection and go back to SelectionSkill
                                // ^^^^--- by not placing the action in the hisotry
                                transition_phase_event
                                    .send(TransitionPhaseEvent(CombatState::SelectionSkills));
                            }
                            Some(ref mut targets) => {
                                let old_target = targets.pop().unwrap();
                                commands.entity(old_target).remove::<Targeted>();
                                if targets.len() == 0 {
                                    last_action.targets = None;
                                }
                                // combat_panel.history.push((*last_action).clone());
                                combat_panel.history.push(last_action.clone());

                                if combat_panel.phase == CombatState::SelectionCaster {
                                    transition_phase_event
                                        .send(TransitionPhaseEvent(CombatState::SelectionTarget));
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
