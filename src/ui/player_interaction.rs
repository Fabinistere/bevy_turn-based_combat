//! All dialog method handler related with the player input directly

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    window::PrimaryWindow,
};

use crate::{
    combat::{
        phases::TransitionPhaseEvent,
        skills::{Skill, TargetOption},
        Action, ActionCount, CombatPanel, CombatState,
    },
    constants::ui::dialogs::*,
    ui::{
        combat_panel::{ActionDisplayer, SkillDisplayer},
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

#[derive(Component, Default)]
pub struct ScrollingList {
    position: f32,
}

/// # Note
///
/// TODO: Unsynchronise lists (scroll only if the cursor is on the list in question)
///
/// TODO: Customise the mouse scrolling system for actions (could also work with the skills menu overflow)
/// TODO: (Prevent) Only allow scrolling on visible actions
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

// /// TODO: couldhave - Hover Unit = Preview Combat Page
// /// Give Hovered which is prioritized to be displayed if it exists
// pub fn hover_unit_by_mouse() {}

/// Adds the Component 'Clicked' to a valid Entity
///
/// # Note
///
/// TODO: couldhave - can drag unit just to cancel the click = avoid missclick by dragging
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
/// - TODO: couldhave - Skill dropped
///   - To a possible target: Confirm
///   - To something else: Cancel (or just back to skill clicked)
pub fn select_skill(
    mut combat_panel: ResMut<CombatPanel>,

    mut interaction_query: Query<
        (&Interaction, &Skill, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>, With<SkillDisplayer>),
    >,

    mut text_query: Query<&mut Text>,

    unit_selected_query: Query<(Entity, &Name, &ActionCount), With<Selected>>,
    mut transition_phase_event: EventWriter<TransitionPhaseEvent>,
) {
    for (interaction, skill, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();

        // XXX: Tempo the command which give a Selected after cancel_input a selfcast skill in SelectionCaster
        if let Err(_) = unit_selected_query.get_single() {
            warn!("No Selected in SelectionSkill");
            continue;
        }

        // if this system can run
        // we are in SelectionSkill or SelectionTarget
        // so there is a selected unit.
        let (caster, _caster_name, action_count) = unit_selected_query.single();

        match *interaction {
            Interaction::Clicked => {
                // <=
                if action_count.current == 0 {
                    text.sections[0].value = String::from("0ac Left");
                    *color = INACTIVE_BUTTON.into();
                    continue;
                }

                // BUG: XXX: Weird "Bug" Event/GameState related handle
                // Prevent the Trigger of the "double press"
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
                    // FIXME: Select Bam/Swing instantly into select solo will create two action "solo"
                    // caster stay the same
                    last_action.skill = skill.clone();
                    last_action.targets = None;

                    // This transitionEvent will trigger all the verification about skill selected (selfcast, etc)
                    transition_phase_event.send(TransitionPhaseEvent(CombatState::SelectionTarget));

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
/// TODO: feature - Confirm The EndOfTurn
/// BUG: End of turn in SelectionSkill: trigger a double press
/// @see [`ui::player_interaction::confirm_action_button()`] to check: correct target number
pub fn end_of_turn_button(
    mut combat_panel: ResMut<CombatPanel>,

    mut interaction_query: Query<
        (&Interaction, &Children),
        (Changed<Interaction>, With<Button>, With<EndOfTurnButton>),
    >,

    mut text_query: Query<&mut Text>,

    mut transition_phase_event: EventWriter<TransitionPhaseEvent>,
) {
    for (interaction, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                if let Some(last_action) = combat_panel.history.last() {
                    if !last_action.is_correct(combat_panel.number_of_fighters.clone()) {
                        combat_panel.history.pop();
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
///
/// Many operation are processed in `combat::phases::phase_transition()`
///
/// # Note
///
/// Can be an [Exclusive System](https://github.com/bevyengine/bevy/blob/1c5c94715cb17cda5ae209eef12a938501de90b5/examples/ecs/ecs_guide.rs#L198)
pub fn cancel_last_input(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut combat_panel: ResMut<CombatPanel>,

    selected_unit_query: Query<(Entity, &Name), With<Selected>>,
    mut caster_query: Query<(Entity, &mut ActionCount)>,

    mut transition_phase_event: EventWriter<TransitionPhaseEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        let current_phase = combat_panel.phase.clone();
        info!("Esc in {}", current_phase);

        match current_phase {
            CombatState::SelectionSkill => {
                let (selected, name) = selected_unit_query.single();

                commands.entity(selected).remove::<Selected>();
                info!("{} is no longer selected", name);

                transition_phase_event.send(TransitionPhaseEvent(CombatState::SelectionCaster));
            }
            CombatState::SelectionCaster | CombatState::SelectionTarget => {
                // Remove last targeted and modify the last action
                match combat_panel.history.last_mut() {
                    None => {
                        if current_phase == CombatState::SelectionTarget {
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
                            } else {
                                info!("{}:{:?} is selected", name, selected);
                            }
                        }
                        // XXX: this command take too long to be processed if we transi to SelectionSkill
                        // [Hard Sync Point](https://github.com/bevyengine/bevy/issues/1613) required to stop waiting the command to be executed
                        // IDEA: New TempoPhase to wait for commands to be processed.
                        commands.entity(last_action.caster).insert(Selected);
                        info!("{:?} is now selected", last_action.caster);

                        match &mut last_action.targets {
                            None => {
                                combat_panel.history.pop();

                                transition_phase_event
                                    .send(TransitionPhaseEvent(CombatState::SelectionSkill));
                            }
                            Some(ref mut targets) => {
                                let old_target = targets.pop().unwrap();
                                commands.entity(old_target).remove::<Targeted>();
                                if targets.len() == 0 {
                                    last_action.targets = None;
                                }

                                if current_phase == CombatState::SelectionCaster {
                                    let mut action_count = caster_query
                                        .get_component_mut::<ActionCount>(last_action.caster)
                                        .unwrap();
                                    action_count.current += 1;
                                    info!("action given back, left: {}", action_count.current);

                                    match last_action.skill.target_option {
                                        TargetOption::OneSelf
                                        | TargetOption::All
                                        | TargetOption::AllAlly
                                        | TargetOption::AllEnemy => {
                                            combat_panel.history.pop();

                                            transition_phase_event.send(TransitionPhaseEvent(
                                                CombatState::SelectionSkill,
                                            ));
                                        }
                                        _ => {
                                            transition_phase_event.send(TransitionPhaseEvent(
                                                CombatState::SelectionTarget,
                                            ));
                                        }
                                    }
                                } else {
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

/// Button interaction system for ActionDisplayer
///
/// # Behavior
///
/// - Clicked
/// put the one clicked as last (downward the action to be accessed easly)
/// - TODO: Hover Action
/// Visualize action effect
///
/// # Note
///
/// TODO: feat UI - simplify deleting an action by adding a cross to do so.
pub fn action_button(
    mut commands: Commands,
    mut combat_panel: ResMut<CombatPanel>,

    mut interaction_query: Query<
        (&Interaction, &ActionDisplayer),
        (Changed<Interaction>, With<Button>),
    >,
    selected_query: Query<Entity, With<Selected>>,
    targeted_query: Query<Entity, With<Targeted>>,
    mut transition_phase_event: EventWriter<TransitionPhaseEvent>,
) {
    for (interaction, action_displayer) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                info!("Action {} clicked", action_displayer.0);

                if combat_panel.history.len() <= action_displayer.0 {
                    warn!(
                        "Action {} is visible even if it shouldn't: {}/{}",
                        action_displayer.0,
                        action_displayer.0,
                        combat_panel.history.len()
                    )
                } else if let Some(last_action) = combat_panel.history.last() {
                    // don't bother to do anything if there is only one action
                    // or if the action clicked was already the last
                    if 1 != combat_panel.history.len()
                        && action_displayer.0 + 1 != combat_panel.history.len()
                    {
                        if !last_action.is_correct(combat_panel.number_of_fighters.clone()) {
                            info!("Abort current action (wasn't complete)");
                            combat_panel.history.pop().unwrap();
                        }

                        // use of remove() to preserve order
                        let action = combat_panel.history.remove(action_displayer.0);
                        combat_panel.history.push(action.clone());

                        // --- Clean Up ---
                        if let Ok(selected) = selected_query.get_single() {
                            if action.clone().caster != selected {
                                commands.entity(selected).remove::<Selected>();
                            }
                        }
                        commands.entity(action.clone().caster).insert(Selected);

                        transition_phase_event
                            .send(TransitionPhaseEvent(CombatState::SelectionSkill));

                        for targeted in targeted_query.iter() {
                            commands.entity(targeted).remove::<Targeted>();
                        }
                    }
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

// TODO: equip stuffs
