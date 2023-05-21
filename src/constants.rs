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
pub const HEIGHT: f32 = 1080.; // 720.;
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
    pub mod dialogs {
        use bevy::prelude::Color;

        pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
        pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
        pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
    }

    pub const DRAGGED_ENTITY_Z: f32 = 100.0;
}
