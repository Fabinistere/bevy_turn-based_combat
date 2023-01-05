//! Constants
//!
//! 1 == one pixel
//! 0.6 == characters' pixel
//! magical number = ratio

pub const BACKGROUND_COLOR: bevy::render::color::Color = bevy::render::color::Color::Rgba {
    red: 58.0 / 256.0,
    green: 36.0 / 256.0,
    blue: 48.0 / 256.0,
    alpha: 1.0,
};

pub const CLEAR: bevy::render::color::Color = bevy::render::color::Color::rgb(0.1, 0.1, 0.1);

pub const FIXED_TIME_STEP: f32 = 1.0 / 60.0;

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 1.;

pub mod character {

    use super::TILE_SIZE;

    pub const CHAR_SCALE: f32 = 0.6 * TILE_SIZE;

    pub const CHAR_HITBOX_HEIGHT: f32 = 1.5 * CHAR_SCALE;
    pub const CHAR_HITBOX_WIDTH: f32 = 5. * CHAR_SCALE;
    pub const CHAR_HITBOX_Y_OFFSET: f32 = -8.5 * CHAR_SCALE;

    pub const KARMA_MIN: i32 = -100;
    pub const KARMA_MAX: i32 = 100;

    pub mod npc {

        pub const NPC_SCALE: f32 = super::CHAR_SCALE;

        pub const NPC_Z_BACK: f32 = 2.;
        pub const NPC_Z_FRONT: f32 = 8.;

        pub const ADMIRAL_STARTING_ANIM: usize = 0;
        pub const OLF_STARTING_ANIM: usize = 16;
        pub const HUGO_STARTING_ANIM: usize = 36;
        pub const FABICURION_STARTING_ANIM: usize = 40;

        pub mod dialog {
            pub const RANDOM_DIALOG: &str = "# Fabien\n
- Enfant, j'ai eu un poney
- Mais j'ai toujours voulu un agneau\n";
            pub const OLF_DIALOG: &str = "# Olf

- Il faut absolument sauver les Fabien du Chien Geant

## Morgan

- ... | None

### Olf

- Il me faut donc obtenir le trône

#### Morgan

- ... | None
- et de l'$ | None

##### Olf

- Et de l'$
- C'est essentiel

##### Olf

- C'est essentiel\n";
            pub const FABIEN_DIALOG: &str =
            "# Fabien

- Hello

## Morgan

- Hey | None
- No Hello | None
- Want to share a flat ? | None

### Fabien

- :)

### Fabien

- :O

### Fabien

- Sure\n";
        }

        pub mod movement {
            use crate::TILE_SIZE;

            pub const REST_TIMER: u64 = 3;
            // TODO adjust EVASION_TIMER / FAIR_PLAY_TIMER
            pub const EVASION_TIMER: u64 = 5;

            pub const NPC_SPEED_LEADER: f32 = 70. * TILE_SIZE;
            pub const NPC_SPEED: f32 = 50. * TILE_SIZE; // -> Speed::default()

            pub const ADMIRAL_POSITION: (f32, f32, f32) = (30.0, 10.0, 2.0);
            pub const HUGO_POSITION: (f32, f32, f32) = (30.0, -20.0, 2.0);
            pub const FABICURION_POSITION: (f32, f32, f32) = (70.0, 10.0, 2.0);
            pub const OLF_POSITION: (f32, f32, f32) = (70.0, -20.0, 2.0);
        }
    }

    pub mod player {

        pub const PLAYER_STARTING_ANIM: usize = 4;

        pub const PLAYER_SCALE: f32 = super::CHAR_SCALE;
        pub const PLAYER_Z: f32 = 6.;

        pub const PLAYER_HP: i32 = 50;
        pub const PLAYER_MANA: i32 = 100;
        pub const PLAYER_INITIATIVE: i32 = 40;
        pub const PLAYER_ATTACK: i32 = 10;
        pub const PLAYER_ATTACK_SPE: i32 = 30;
        pub const PLAYER_DEFENSE: i32 = 0;
        pub const PLAYER_DEFENSE_SPE: i32 = 10;
    }
}

pub mod combat {
    pub mod team {
        pub const TEAM_MC: i32 = 0;
        pub const TEAM_OLF: i32 = 1;
        pub const TEAM_FABICURION: i32 = 2;
    }

    pub mod skill {
        pub const BAM: i32 = 150;
    }

    pub mod buff {
    }
}

pub mod ui {
    pub mod dialogs {
        use bevy::prelude::Color;

        pub const DIALOG_BOX_ANIMATION_OFFSET: f32 = -1000.0;
        pub const DIALOG_BOX_UPDATE_DELTA_S: f32 = 0.05;
        pub const DIALOG_BOX_ANIMATION_TIME_MS: u64 = 500;
        pub const SCROLL_SIZE: (f32, f32) = (490.0, 11700.0 / 45.0);
        pub const SCROLL_ANIMATION_DELTA_S: f32 = 0.1;
        pub const SCROLL_ANIMATION_FRAMES_NUMBER: usize = 45;

        pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
        pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
        pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
    }
}
