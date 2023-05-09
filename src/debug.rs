use bevy::prelude::*;

// RegisterInspectable, 
use bevy_inspector_egui::quick::{
    // FilterQueryInspectorPlugin,
    ResourceInspectorPlugin,
    WorldInspectorPlugin,
};
// // use bevy_inspector_egui::prelude::*;

use crate::{
    combat::{
        // Alterations,
        alterations::{Alteration, AlterationAction},
        skills::{
            // Skill,
            SkillType,
            TargetSide,
        },
        stats::{Hp, Mana, Shield, Initiative, Attack, AttackSpe, Defense, DefenseSpe}, CombatState,
    },
    npc::NPC,
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new())
                // .add_plugin(FilterQueryInspectorPlugin::<With<TextureAtlasSprite>>::default())
                .add_plugin(ResourceInspectorPlugin::<Time>::default())

                .register_type::<NPC>()

                .register_type::<CombatState>()

                .register_type::<Alteration>()
                // .register_type::<Alterations>()
                .register_type::<AlterationAction>()
                .register_type::<TargetSide>()
                
                // .register_type::<Skill>()
                .register_type::<SkillType>()
                
                // stats
                
                .register_type::<Hp>()
                .register_type::<Mana>()
                .register_type::<Shield>()
                .register_type::<Initiative>()
                .register_type::<Attack>()
                .register_type::<AttackSpe>()
                .register_type::<Defense>()
                .register_type::<DefenseSpe>()

                // UI
                ;
        }
    }
}

// TODO: Create debug log kind
// Combat Debug
// Movement Debug
// Dialog Debug
// ...

// make it clear in the global log (different files ?)
//   - global log file
//   - specific (Combat/Movement/Dialog) log file
// ask for sending logs and data to *me* when game crash

// TODO: Create Custom Lint Rule
// function using query not being added to a plugin
// event ...
// plugin ...

// TODO: Create Contribution Example
// for
// - fn
// - struct
//   - Component
//   - Event
//   - Plugin
// - Module
