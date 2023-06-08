//! Characters

use bevy::{prelude::*, utils::HashMap};

pub mod npcs;

#[derive(Debug, Reflect, Clone, Hash)]
pub struct PersonalInfos {
    pub title: String,
    pub sprite_path: String,
}

/// Link a Name with the asset path of each idle sprite
///
/// # Note
///
/// - This aHashMap is not designed for cryptoSecurity but for performance (from bevy)
/// - REFACTOR: Temporary until we can put spritesheet in a UiElement
#[derive(Debug, Resource, Reflect, Deref, DerefMut, Clone)]
pub struct FabiensInfos(pub HashMap<String, PersonalInfos>);

/// Correspond with the default for the initiation of the resource
impl FromWorld for FabiensInfos {
    fn from_world(_world: &mut World) -> Self {
        let mut fabiens_infos = FabiensInfos(HashMap::new());

        fabiens_infos.insert(
            String::from("NPC Fabien"),
            PersonalInfos {
                title: String::from("Fabien Loyal"),
                sprite_path: String::from("textures/character/idle/idle_Fabien_Loyal.png"),
            },
        );
        fabiens_infos.insert(
            String::from("NPC Fabien Disloyal"),
            PersonalInfos {
                title: String::from("Fabien Disloyal"),
                sprite_path: String::from("textures/character/idle/idle_Fabien_disloyal.png"),
            },
        );
        fabiens_infos.insert(
            String::from("Player Morgan"),
            PersonalInfos {
                title: String::from("Fabien l'informaticien"),
                sprite_path: String::from("textures/character/idle/idle_Morgan.png"),
            },
        );
        fabiens_infos.insert(
            String::from("NPC Admiral"),
            PersonalInfos {
                // Tigrours de guerre fabinique
                title: String::from("Fabien l'Amiral"),
                sprite_path: String::from("textures/character/idle/idle_Admiral.png"),
            },
        );
        fabiens_infos.insert(
            String::from("NPC Enzo"),
            PersonalInfos {
                title: String::from("Fabien de Ferdinand"),
                sprite_path: String::from("textures/character/idle/idle_Enzo.png"),
            },
        );
        fabiens_infos.insert(
            String::from("NPC Fabicurion 0"),
            PersonalInfos {
                title: String::from("Fabicurion"),
                sprite_path: String::from("textures/character/idle/idle_Fabicurion.png"),
            },
        );
        fabiens_infos.insert(
            String::from("NPC Fabicurion 1"),
            PersonalInfos {
                title: String::from("Fabicurion"),
                sprite_path: String::from("textures/character/idle/idle_Fabicurion.png"),
            },
        );
        fabiens_infos.insert(
            String::from("NPC Mae"),
            PersonalInfos {
                title: String::from("Fabien de Ferdinand"),
                sprite_path: String::from("textures/character/idle/idle_General.png"),
            },
        );
        fabiens_infos.insert(
            String::from("NPC Ieud"),
            PersonalInfos {
                title: String::from("Fabien le Dieu Suprème"),
                sprite_path: String::from("textures/character/idle/idle_Ieud.png"),
            },
        );
        fabiens_infos.insert(
            String::from("NPC Hugo"),
            PersonalInfos {
                title: String::from("Fabien le Ministre de la Culture"),
                sprite_path: String::from("textures/character/idle/idle_Nurse.png"),
            },
        );
        fabiens_infos.insert(
            String::from("NPC Olf"),
            PersonalInfos {
                title: String::from("Fabien du Divin Goulag"),
                sprite_path: String::from("textures/character/idle/idle_Olf.png"),
            },
        );
        fabiens_infos.insert(
            String::from("NPC Olf Ghost"),
            PersonalInfos {
                title: String::from("Fabien le Souvenir Oublié"),
                sprite_path: String::from("textures/character/idle/idle_Olf_Ghost.png"),
            },
        );
        fabiens_infos.insert(
            String::from("NPC Vampire"),
            PersonalInfos {
                title: String::from("Fabien le Fabancelier"),
                sprite_path: String::from("textures/character/idle/idle_Vampire.png"),
            },
        );

        fabiens_infos
    }
}
