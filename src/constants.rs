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

    /// BUG: Must Adapt to resolution (see bug in `ui::player_interaction::select_unit_by_mouse()`).
    /// The size is offset when in a different resolution/Window Size than (1920,1080)
    pub const SPRITE_SIZE: (f32, f32) = (25.0, 40.0);

    pub const KARMA_MIN: i32 = -100;
    pub const KARMA_MAX: i32 = 100;

    pub mod npc {

        pub const NPC_SCALE: f32 = super::CHAR_SCALE;

        pub const NPC_Z_BACK: f32 = 2.;
        pub const NPC_Z_FRONT: f32 = 8.;

        pub const ADMIRAL_STARTING_ANIM: usize = 0;
        pub const MORGAN_STARTING_ANIM: usize = 4;
        pub const FABIEN_STARTING_ANIM: usize = 8;
        pub const OLF_STARTING_ANIM: usize = 16;
        pub const HUGO_STARTING_ANIM: usize = 36;
        pub const FABICURION_STARTING_ANIM: usize = 40;
    }
}

pub mod combat {

    pub const BASE_ACTION_COUNT: usize = 1;
    pub const MAX_PARTY: usize = 6;
    pub const FIRST_ALLY_ID: usize = 0;
    pub const FIRST_ENEMY_ID: usize = MAX_PARTY;

    pub mod team {
        pub const TEAM_MC: i32 = 0;
        pub const TEAM_OLF: i32 = 1;
        pub const TEAM_FABICURION: i32 = 2;
    }

    pub mod skill {
        use crate::spritesheet::SpriteSheetIndex;

        pub const BAM: i32 = 150;

        pub const HOLY_SPELL_01_START_INDEX: usize = 16;
        pub const HOLY_SPELL_01_END_INDEX: usize = 22;
        pub const HOLY_SPELL_02_START_INDEX: usize = 0;
        pub const HOLY_SPELL_02_END_INDEX: usize = 15;

        pub const HOLY_SPELL_01: SpriteSheetIndex = SpriteSheetIndex {
            start_index: HOLY_SPELL_01_START_INDEX,
            end_index: HOLY_SPELL_01_END_INDEX,
        };
        pub const HOLY_SPELL_02: SpriteSheetIndex = SpriteSheetIndex {
            start_index: HOLY_SPELL_02_START_INDEX,
            end_index: HOLY_SPELL_02_END_INDEX,
        };
    }

    pub mod alteration {
        pub const SIZE_ALTERATION_ICON: f32 = 5.;
    }
}

pub mod ui {

    pub const DRAGGED_ENTITY_Z: f32 = 100.0;
    pub const FIGHTING_HALL_WIDTH: f32 = 56.;
    pub const INITIATIVE_BAR_WIDTH: f32 = 8.;
    pub const HUD_WALL_WIDTH: f32 = 100. - (FIGHTING_HALL_WIDTH + INITIATIVE_BAR_WIDTH);

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
                color: Color::WHITE, // rgb(0.9, 0.9, 0.9),
            }
        }

        // NOTE: Style Constant or Style Method ? (see: https://discord.com/channels/691052431525675048/1119426776033140879)
        // --- Style Constant ---
        // pub const TEXT_STYLE: Style = {
        //     let mut style = Style::DEFAULT;
        //     style.flex_shrink = 0.;
        //     style.width = Val::Px(0.);
        //     style.height = Val::Px(20.);
        //     style.margin = UiRect {
        //         left: Val::Auto,
        //         right: Val::Auto,
        //         ..UiRect::DEFAULT
        //     };
        //     style
        // };
        // --- Style Method ---
        // pub const TEXT_STYLE: Style = text_style();
        // pub const fn text_style() -> Style {
        //     Style {
        //         flex_shrink: 0.,
        //         width: Val::Px(0.),
        //         height: Val::Px(20.),
        //         margin: UiRect {
        //             left: Val::Auto,
        //             right: Val::Auto,
        //             ..UiRect::DEFAULT
        //         },
        //         ..Style::DEFAULT
        //     }
        // }

        pub const TEXT_STYLE: Style = {
            let mut style = Style::DEFAULT;
            style.flex_shrink = 0.;
            style.height = Val::Px(20.);
            style.margin = UiRect {
                left: Val::Auto,
                right: Val::Auto,
                ..UiRect::DEFAULT
            };
            style
        };

        pub const LIST_HIDDEN_OVERFLOW_STYLE: Style = {
            let mut style = Style::DEFAULT;
            style.flex_direction = FlexDirection::Column;
            style.align_self = AlignSelf::Stretch;
            style.overflow = Overflow::clip_y();
            style
        };

        pub const MOVING_PANEL_STYLE: Style = {
            let mut style = Style::DEFAULT;
            style.flex_direction = FlexDirection::Column;
            style.flex_wrap = FlexWrap::NoWrap;
            style.align_items = AlignItems::FlexStart;
            style
        };

        pub const SKILL_BUTTON_STYLE: Style = {
            let mut style = Style::DEFAULT;
            style.width = Val::Px(150.0);
            style.height = Val::Px(65.0);
            // center button
            style.margin = UiRect::all(Val::Auto);
            // horizontally center child text
            style.justify_content = JustifyContent::Center;
            // vertically center child text
            style.align_items = AlignItems::Center;
            style
        };

        pub const ACTION_BUTTON_STYLE: Style = {
            let mut style = Style::DEFAULT;
            style.width = Val::Px(154.); // Val::Percent(100.);
            style.height = Val::Px(103.);
            style.justify_content = JustifyContent::Center;
            style.flex_direction = FlexDirection::ColumnReverse;
            style.flex_wrap = FlexWrap::NoWrap;
            style.align_items = AlignItems::Center;
            style
        };

        pub const ALLIES_SHEET_STYLE: Style = {
            let mut style = Style::DEFAULT;
            style.flex_shrink = 0.;
            style.flex_direction = FlexDirection::Column;
            style.width = Val::Percent(100.);
            style.height = Val::Percent(50.);
            // gap between the two rows
            style.row_gap = Val::Percent(8.);
            style
        };

        pub const ROW_SHEETS_STYLE: Style = {
            let mut style = Style::DEFAULT;
            style.flex_shrink = 0.;
            style.flex_direction = FlexDirection::Row;
            style.height = Val::Percent(50.);
            // gap between the three scrolls
            style.column_gap = Val::Percent(2.3);
            style
        };

        pub const MINI_CHARACTER_SHEET_STYLE: Style = {
            let mut style = Style::DEFAULT;
            style.width = Val::Percent(23.);
            style.height = Val::Percent(96.);
            style.left = Val::Percent(16.8);
            style.top = Val::Percent(16.);
            style
        };
    }
}
