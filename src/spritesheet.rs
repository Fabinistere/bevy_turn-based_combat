use bevy::prelude::*;

pub struct FabienPlugin;

impl Plugin for FabienPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreStartup,
            (load_character_spritesheet, load_vfx_spritesheet),
        );
    }
}

#[derive(Clone, Resource)]
pub struct FabienSheet(pub Handle<TextureAtlas>);

#[derive(Clone, Resource)]
pub struct VFXSheet(pub Handle<TextureAtlas>);

#[derive(Component)]
pub struct SpriteSheetAnimation {
    pub index: SpriteSheetIndex,
    pub timer: Timer,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SpriteSheetIndex {
    pub start_index: usize,
    pub end_index: usize,
}

impl SpriteSheetIndex {
    pub fn new(start_index: usize, end_index: usize) -> Self {
        SpriteSheetIndex {
            start_index,
            end_index,
        }
    }
}

fn load_character_spritesheet(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("textures/character/big_sprite_sheet.png");
    let atlas = TextureAtlas::from_grid(image, Vec2::splat(34.), 4, 12, None, None);

    let atlas_handle = texture_atlases.add(atlas);

    commands.insert_resource(FabienSheet(atlas_handle));
}

fn load_vfx_spritesheet(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("textures/vfx/VFX.png");
    let atlas = TextureAtlas::from_grid(image, Vec2::splat(48.), 16, 2, None, None);

    let atlas_handle = texture_atlases.add(atlas);

    commands.insert_resource(VFXSheet(atlas_handle));
}
