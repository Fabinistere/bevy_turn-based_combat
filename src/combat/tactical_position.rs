//! All systems about Tactical Position and their transform

// TODO: Move Unit / Swap

use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};

use crate::{
    combat::{InCombat, Player, Recruted, TacticalPlace, TacticalPosition},
    constants::ui::fighting_hall_position::*,
};

/// Happens when:
/// - the window (size, etc) changed
/// - one of the TacticalPosition changed
///
/// Read in:
/// - tactical_position::update_character_position()
///   - Adapt OnScreenPosition for each InCombat character
pub struct UpdateCharacterPositionEvent;

/// Detect Window change (size) or
/// Detect Change in the component TacticalPosition in an entity
///
/// At each change detected, will send an event to adapt OnScreenPosition
///
/// # Note
///
/// TODO: Active system in combat::mod
/// FIXME: prevent repetitive Event send
pub fn detect_window_tactical_pos_change(
    resize_event: Res<Events<WindowResized>>,
    characters_query: Query<(Changed<TacticalPosition>, With<InCombat>, With<Transform>)>,

    mut update_char_pos_event: EventWriter<UpdateCharacterPositionEvent>,
) {
    let mut reader = resize_event.get_reader();
    for _ in reader.iter(&resize_event) {
        info!("Window Resized");
        update_char_pos_event.send(UpdateCharacterPositionEvent);
    }
    for _ in characters_query.iter() {
        info!("Tactical Pos Change");
        update_char_pos_event.send(UpdateCharacterPositionEvent);
    }
}

/// Adapt transform depending their tactical position and the window size
///
/// # Note
///
/// FIXME: The transformation window's coordinates -> transform only work with window's size = 1920/1280
pub fn update_character_position(
    // mut update_char_pos_event: EventReader<UpdateCharacterPositionEvent>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    ally_query: Query<Or<(With<Recruted>, With<Player>)>>,
    mut characters_query: Query<
        (Entity, &mut Transform, &TacticalPosition, &Name),
        (Changed<TacticalPosition>, With<InCombat>),
    >,
) {
    // for _ in update_char_pos_event.iter() {
    let window = window_query.get_single().unwrap();

    let width = window.width();
    let height = window.height();

    // 56% = width of the Fighting Hall
    // 17 = number of box / line
    // 2 = half of the box (to point to the center)
    let x = (width * 0.56) / 17.;
    let y = (height * 1.) / 17.;

    for (character, mut transform, tactical_position, name) in characters_query.iter_mut() {
        // if recruted or player == Ally
        let (x_offset, y_offset) = if ally_query.contains(character) {
            match tactical_position {
                TacticalPosition::FrontLine(place) => match place {
                    TacticalPlace::Left => ALLY_FRONTLINE_LEFT,
                    TacticalPlace::Middle => ALLY_FRONTLINE_MIDDLE,
                    TacticalPlace::Right => ALLY_FRONTLINE_RIGHT,
                },
                TacticalPosition::MiddleLine(place) => match place {
                    TacticalPlace::Left => ALLY_MIDDLELINE_LEFT,
                    TacticalPlace::Middle => ALLY_MIDDLELINE_MIDDLE,
                    TacticalPlace::Right => ALLY_MIDDLELINE_RIGHT,
                },
                TacticalPosition::BackLine(place) => match place {
                    TacticalPlace::Left => ALLY_BACKLINE_LEFT,
                    TacticalPlace::Middle => ALLY_BACKLINE_MIDDLE,
                    TacticalPlace::Right => ALLY_BACKLINE_RIGHT,
                },
            }
        } else {
            match tactical_position {
                TacticalPosition::FrontLine(place) => match place {
                    TacticalPlace::Left => ENEMY_FRONTLINE_LEFT,
                    TacticalPlace::Middle => ENEMY_FRONTLINE_MIDDLE,
                    TacticalPlace::Right => ENEMY_FRONTLINE_RIGHT,
                },
                TacticalPosition::MiddleLine(place) => match place {
                    TacticalPlace::Left => ENEMY_MIDDLELINE_LEFT,
                    TacticalPlace::Middle => ENEMY_MIDDLELINE_MIDDLE,
                    TacticalPlace::Right => ENEMY_MIDDLELINE_RIGHT,
                },
                TacticalPosition::BackLine(place) => match place {
                    TacticalPlace::Left => ENEMY_BACKLINE_LEFT,
                    TacticalPlace::Middle => ENEMY_BACKLINE_MIDDLE,
                    TacticalPlace::Right => ENEMY_BACKLINE_RIGHT,
                },
            }
        };

        info!("{}", name);

        info!("width: {}, x: {}", width, x);
        info!("height: {}, y: {}", height, y);
        info!("x_offset: {}, y_offset: {}", x_offset, y_offset);

        let window_coordinates = (x * x_offset, y * y_offset);
        info!(
            "x_w: {}, y_w: {}",
            window_coordinates.0, window_coordinates.1
        );

        // TODO: to be in the center of the box = win_cor - ((width * 0.56) / 17.) / 2.

        let transform_coordinates = (
            window_coordinates.0 * (90. / (width / 2.)) - 90.,
            window_coordinates.1 * (50. / (height / 2.)) - 50.,
        );
        info!(
            "x_t: {}, y_t: {}",
            transform_coordinates.0, transform_coordinates.1
        );

        transform.translation.x = transform_coordinates.0;
        transform.translation.y = transform_coordinates.1;

        info!("---------------");
    }
    // }
}
