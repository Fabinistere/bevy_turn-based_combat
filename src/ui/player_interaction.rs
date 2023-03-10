//! All dialog method handler related with the player input directly

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

use crate::{
    combat::skills::Skill,
    constants::{ui::dialogs::*, RESOLUTION},
    ui::combat_system::{ButtonSelection, Selected, Targeted},
};

use super::combat_system::ButtonTargeting;

/// Happens in
///   - ui::dialog_player::button_system
///     - BAM clicked
/// Read in
///   - ???
///     - Execute the skill with the UnitSelected's Stats
///     to the UnitTargetted
pub struct ExecuteSkillEvent {
    pub skill: Skill,
    pub caster: Entity,
    pub target: Entity,
}

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

/// Adds the Component 'Clicked' to a valid Entity
///
/// # Note
///
/// TODO: Unit Clicked (Selected)
pub fn select_unit_by_mouse(
    mut commands: Commands,

    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,

    selectable_unit_query: Query<
        (Entity, &Transform, &SpriteSize, &Name),
        (With<Clickable>, Without<Clicked>),
    >,
) {
    let window = windows.get_primary().unwrap();

    if let Some(position) = window.cursor_position() {
        if buttons.just_pressed(MouseButton::Left) {
            // info!("({}, {})", position.x, position.y);

            // TODO: Magical Number...
            let window_height = 720.0;
            let window_width = window_height * RESOLUTION;

            let transform_height = 100.0;
            let transform_width = 180.0;

            // in translation : 0,0 = center
            // in cursor pos : 0,0 = bottom left

            // in transform: 180 wide ? 100 height
            let ratio_x = -transform_width / window_width;
            let ratio_y = transform_height / window_height;

            for (unit, transform, sprite_size, name) in selectable_unit_query.iter() {
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
                    info!("{} clicked", name);
                    commands.entity(unit).insert(Clicked);
                }
            }
        }
    } else {
        // cursor is not inside the window
    }
}

// # Note
//
// TODO: feature - can drag unit just to cancel the click
// avoid missclick by dragging
//
// TODO: Skill dropped
// To a possible target: Confirm
// To something else: Cancel (or just back to skill clicked)

// # Note
//
// TODO: Hover Unit - Preview Combat Page
// TODO: Hover Skill - Preview possible target

/// Action for each Interaction of the button
pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (
            Changed<Interaction>,
            With<Button>,
            Without<ButtonSelection>,
            Without<ButtonTargeting>,
        ),
    >,

    mut text_query: Query<&mut Text>,

    unit_selected_query: Query<(Entity, &Selected)>,
    unit_targeted_query: Query<(Entity, &Targeted)>,

    mut execute_skill_event: EventWriter<ExecuteSkillEvent>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                if let Ok((caster, _)) = unit_selected_query.get_single() {
                    if let Ok((target, _)) = unit_targeted_query.get_single() {
                        let bam_skill = Skill::bam();

                        // TODO: send event to inflict the skill to the Targeted entity
                        execute_skill_event.send(ExecuteSkillEvent {
                            skill: bam_skill,
                            caster,
                            target,
                        });
                    }
                }

                text.sections[0].value = "BOM".to_string();
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                text.sections[0].value = "BAM".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "BAM".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

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
