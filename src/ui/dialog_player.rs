//! All dialog method handler related with the player input directly

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

use crate::{
    constants::ui::dialogs::*,
    combat::skills::Skill,
};

use super::dialog_combat::{ButtonSelection, UnitTargeted, UnitSelected};

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

pub fn cursor_position(
    windows: Res<Windows>,
) {
    // Games typically only have one window (the primary window).
    // For multi-window applications, you need to use a specific window ID here.
    let window = windows.get_primary().unwrap();

    if let Some(_position) = window.cursor_position() {
        // cursor is inside the window, position given
    } else {
        // cursor is not inside the window
    }
}

/// Action for each Interaction of the button
pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>, Without<ButtonSelection>),
    >,

    mut text_query: Query<&mut Text>,

    unit_selected_query: Query<
        (Entity, &UnitSelected)
    >,
    unit_targeted_query: Query<
        (Entity, &UnitTargeted)
    >,

    mut execute_skill_event: EventWriter<ExecuteSkillEvent>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                let (_, caster) = unit_selected_query.single();
                let (_, target) = unit_targeted_query.single();

                let bam_skill = Skill::bam();

                // TODO: send event to inflict the skill to the entity contained in UnitTargeted
                execute_skill_event.send(
                    ExecuteSkillEvent {
                        skill: bam_skill,
                        caster: caster.0.unwrap(),
                        target: target.0.unwrap()
                    }
                );

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
