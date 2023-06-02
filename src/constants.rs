//! Constants
//!
//! 1 == one pixel
//! 0.6 == characters' pixel
//! magical number = ratio

// dark purple: #3A2430
pub const BACKGROUND_COLOR: bevy::render::color::Color = bevy::render::color::Color::Rgba {
    red: 58.0 / 256.0,
    green: 36.0 / 256.0,
    blue: 48.0 / 256.0,
    alpha: 1.0,
};

pub const CLEAR: bevy::render::color::Color = bevy::render::color::Color::rgb(0.1, 0.1, 0.1);

pub const FIXED_TIME_STEP: f32 = 1.0 / 60.0;

pub const RESOLUTION: f32 = 16.0 / 9.0;
// TODO: feature - dynamic scale height (or option)
pub const HEIGHT: f32 = 1080.; // 720.; //
pub const TILE_SIZE: f32 = 1.;

pub mod character {

    use super::TILE_SIZE;

    pub const CHAR_SCALE: f32 = 0.6 * TILE_SIZE;

    pub const KARMA_MIN: i32 = -100;
    pub const KARMA_MAX: i32 = 100;

    pub mod npc {

        pub const NPC_SCALE: f32 = super::CHAR_SCALE;

        pub const NPC_Z_BACK: f32 = 2.;
        pub const NPC_Z_FRONT: f32 = 8.;

        pub const ADMIRAL_STARTING_ANIM: usize = 0;
        pub const FABIEN_STARTING_ANIM: usize = 8;
        pub const OLF_STARTING_ANIM: usize = 16;
        pub const HUGO_STARTING_ANIM: usize = 36;
        pub const FABICURION_STARTING_ANIM: usize = 40;

        pub mod movement {

            pub const ADMIRAL_POSITION: (f32, f32, f32) = (-30., 10., 2.);
            pub const HUGO_POSITION: (f32, f32, f32) = (-30., -20., 2.);
            pub const FABICURION_POSITION: (f32, f32, f32) = (-80., 10., 2.);
            pub const OLF_POSITION: (f32, f32, f32) = (-80., -20., 2.);
        }
    }
}

pub mod combat {

    pub const BASE_ACTION_COUNT: usize = 1;

    pub mod team {
        pub const TEAM_MC: i32 = 0;
        pub const TEAM_OLF: i32 = 1;
        pub const TEAM_FABICURION: i32 = 2;
    }

    pub mod skill {
        pub const MAX_PARTY: i32 = 6;
        pub const BAM: i32 = 150;
    }

    pub mod buff {}
}

pub mod ui {

    pub const DRAGGED_ENTITY_Z: f32 = 100.0;

    pub mod fighting_hall_position {
        /* -------------------------------------------------------------------------- */
        /*                               Enemy Position                               */
        /* -------------------------------------------------------------------------- */

        pub const ENEMY_FRONTLINE_LEFT: (f32, f32) = (5., 8.);
        pub const ENEMY_FRONTLINE_MIDDLE: (f32, f32) = (7., 10.);
        pub const ENEMY_FRONTLINE_RIGHT: (f32, f32) = (9., 12.);

        pub const ENEMY_MIDDLELINE_LEFT: (f32, f32) = (3., 10.);
        pub const ENEMY_MIDDLELINE_MIDDLE: (f32, f32) = (5., 12.);
        pub const ENEMY_MIDDLELINE_RIGHT: (f32, f32) = (7., 14.);

        pub const ENEMY_BACKLINE_LEFT: (f32, f32) = (1., 12.);
        pub const ENEMY_BACKLINE_MIDDLE: (f32, f32) = (3., 14.);
        pub const ENEMY_BACKLINE_RIGHT: (f32, f32) = (5., 16.);

        /* -------------------------------------------------------------------------- */
        /*                                Ally Position                               */
        /* -------------------------------------------------------------------------- */

        pub const ALLY_FRONTLINE_LEFT: (f32, f32) = (9., 5.);
        pub const ALLY_FRONTLINE_MIDDLE: (f32, f32) = (11., 7.);
        pub const ALLY_FRONTLINE_RIGHT: (f32, f32) = (13., 9.);

        pub const ALLY_MIDDLELINE_LEFT: (f32, f32) = (11., 3.);
        pub const ALLY_MIDDLELINE_MIDDLE: (f32, f32) = (13., 5.);
        pub const ALLY_MIDDLELINE_RIGHT: (f32, f32) = (15., 7.);

        pub const ALLY_BACKLINE_LEFT: (f32, f32) = (13., 1.);
        pub const ALLY_BACKLINE_MIDDLE: (f32, f32) = (15., 3.);
        pub const ALLY_BACKLINE_RIGHT: (f32, f32) = (17., 5.);
    }

    pub mod dialogs {
        use bevy::prelude::Color;

        // #3c3e40
        pub const INACTIVE_BUTTON: Color = Color::rgb(0.23, 0.24, 0.25);
        // #60666a
        pub const INACTIVE_HOVERED_BUTTON: Color = Color::rgb(0.37, 0.40, 0.41);

        pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
        pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
        pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
    }

    pub mod style {
        //, text::TextStyle, ui::*
        use bevy::prelude::*;

        pub fn get_text_style(asset_server: &Res<AssetServer>, font_size: f32) -> TextStyle {
            TextStyle {
                font: asset_server.load("fonts/dpcomic.ttf"),
                font_size,
                color: Color::rgb(0.9, 0.9, 0.9),
            }
        }

        pub const LIST_HIDDEN_OVERFLOW_STYLE: Style = Style {
            flex_direction: FlexDirection::Column,
            align_self: AlignSelf::Stretch,
            overflow: Overflow::Hidden,
            ..Style::DEFAULT
        };

        pub const MOVING_PANEL_STYLE: Style = Style {
            flex_direction: FlexDirection::Column,
            flex_wrap: FlexWrap::NoWrap,
            max_size: Size::UNDEFINED,
            align_items: AlignItems::FlexStart,
            ..Style::DEFAULT
        };

        pub const SKILL_BUTTON_STYLE: Style = Style {
            size: Size::new(Val::Px(150.0), Val::Px(65.0)),
            // center button
            margin: UiRect::all(Val::Auto),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            position: UiRect::DEFAULT,
            ..Style::DEFAULT
        };

        pub const ACTION_BUTTON_STYLE: Style = Style {
            size: Size::new(
                Val::Px(154.), // Val::Percent(100.),
                Val::Px(103.),
            ),
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::ColumnReverse,
            flex_wrap: FlexWrap::NoWrap,
            align_items: AlignItems::Center,
            position: UiRect::DEFAULT,
            ..Style::DEFAULT
        };
    }
}
