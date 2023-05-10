//! All dialog method handler related with the player input directly

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    window::PrimaryWindow,
};

use crate::{
    combat::{phases::TransitionPhaseEvent, CombatPanel, CombatState},
    constants::ui::dialogs::*,
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

// ----- Specific UI systems -----

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