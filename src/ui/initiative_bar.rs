//! Display the Initiative Vertical Bar
//! All Set action and interaction systems

use bevy::{prelude::*, utils::HashMap};

use crate::{
    combat::{CombatPanel, InCombat},
    // constants::character::npc::FABIEN_STARTING_ANIM,
    ui::combat_panel::ActionDisplayer,
};

/// Link a Name with the asset path of each idle sprite
///
/// # Note
///
/// REFACTOR: Temporary until we can put spritesheet in a UiElement
/// This aHashMap is not designed for cryptoSecurity but for performance (from bevy)
#[derive(Debug, Resource, Reflect, Deref, DerefMut, Clone)]
pub struct SpriteNames(pub HashMap<String, String>);

/// Correspond with the default for the initiation of the resource
impl FromWorld for SpriteNames {
    fn from_world(_world: &mut World) -> Self {
        let mut sprite_names = SpriteNames(HashMap::new());

        sprite_names.insert(
            String::from("NPC Fabien"),
            String::from("textures/character/idle/idle_Fabien_Loyal.png"),
        );
        sprite_names.insert(
            String::from("NPC Fabien Disloyal"),
            String::from("textures/character/idle/idle_Fabien_disloyal.png"),
        );
        sprite_names.insert(
            String::from("NPC Morgan"),
            String::from("textures/character/idle/idle_Morgan.png"),
        );
        sprite_names.insert(
            String::from("NPC Admiral"),
            String::from("textures/character/idle/idle_Admiral.png"),
        );
        sprite_names.insert(
            String::from("NPC Enzo"),
            String::from("textures/character/idle/idle_Enzo.png"),
        );
        sprite_names.insert(
            String::from("NPC Fabicurion 0"),
            String::from("textures/character/idle/idle_Fabicurion.png"),
        );
        sprite_names.insert(
            String::from("NPC Fabicurion 1"),
            String::from("textures/character/idle/idle_Fabicurion.png"),
        );
        sprite_names.insert(
            String::from("NPC General"),
            String::from("textures/character/idle/idle_General.png"),
        );
        sprite_names.insert(
            String::from("NPC Ieud"),
            String::from("textures/character/idle/idle_Ieud.png"),
        );
        sprite_names.insert(
            String::from("NPC Hugo"),
            String::from("textures/character/idle/idle_Nurse.png"),
        );
        sprite_names.insert(
            String::from("NPC Olf"),
            String::from("textures/character/idle/idle_Olf.png"),
        );
        sprite_names.insert(
            String::from("NPC Olf Ghost"),
            String::from("textures/character/idle/idle_Olf_Ghost.png"),
        );
        sprite_names.insert(
            String::from("NPC Vampire"),
            String::from("textures/character/idle/idle_Vampire.png"),
        );

        sprite_names
    }
}

/// Disables empty action,
/// (invisible == disable).
/// And update the text on the Button and the sprite of it.
///
/// Prevents checking a index in the action list.
pub fn action_visibility(
    combat_panel_query: Query<&CombatPanel, Changed<CombatPanel>>,
    mut action_button_query: Query<(&ActionDisplayer, &mut Visibility, &Children), With<Button>>,
    // mut action_sprite_query: Query<&mut TextureAtlasSprite, Without<InCombat>>,
    mut action_image_query: Query<&mut UiImage, Without<InCombat>>,
    mut text_query: Query<&mut Text>,
    caster_name_query: Query<(&Name, &TextureAtlasSprite), With<InCombat>>,

    asset_server: Res<AssetServer>,
    sprite_names: Res<SpriteNames>,
) {
    if let Ok(combat_panel) = combat_panel_query.get_single() {
        for (action_number, mut visibility, action_children) in action_button_query.iter_mut() {
            // let mut action_sprite = action_sprite_query.get_mut(action_children[1]).unwrap();
            let mut action_image = action_image_query.get_mut(action_children[1]).unwrap();

            let old_visibility = visibility.clone();

            let mut text = text_query.get_mut(action_children[0]).unwrap();

            *visibility = if action_number.0 < combat_panel.history.len() {
                let (caster_name, _caster_sprite) = caster_name_query
                    .get(combat_panel.history[action_number.0].caster)
                    .unwrap();
                text.sections[0].value = caster_name.to_string();

                // action_sprite.index = caster_sprite.index;
                action_image.texture =
                    if let Some(asset_path) = sprite_names.get(&caster_name.to_string()) {
                        // println!("{}", asset_path);
                        asset_server.load(asset_path)
                    } else {
                        warn!(
                            "Action Sprite Asset Not Found/Associated With {}",
                            caster_name
                        );
                        asset_server.load("textures/character/idle/idle_Fabien_Loyal.png")
                    };

                // --- Visibility ---
                Visibility::Inherited
            } else {
                // useless --vv
                text.sections[0].value = "None".to_string();
                // action_sprite.index = FABIEN_STARTING_ANIM;
                // useless --^^
                Visibility::Hidden
            };

            // --- Logs ---
            if old_visibility != *visibility {
                // DEBUG: Actions' Visibility switcher
                // info!(
                //     "action °{} visibility switch: {:?}",
                //     action_number.0, *visibility
                // );
            }
        }
    }
}

// TODO: Interaction with action in the Initiative Bar
