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
        // Action,
        alterations::{Alteration, AlterationAction},
        ActionCount,
        // CombatResources,
        CombatState,
        skills::{
            // Skill,
            SkillType,
            TargetOption,
        },
        stats::{Hp, Mana, Shield, Initiative, Attack, AttackSpe, Defense, DefenseSpe},
        stuff::{Equipements, WeaponType, MasteryTier, Job},
        TacticalPlace,
    },
    ui::combat_system::{ActionHistory, LastTurnActionHistory, ActionsLogs},
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app
                .add_plugins((
                    WorldInspectorPlugin::new(),
                    // ResourceInspectorPlugin::<Time>::default(),
                    // FilterQueryInspectorPlugin::<With<TextureAtlasSprite>>::default(),
                ))
               
                /* -------------------------------------------------------------------------- */
                /*                          --- Global Structure ---                          */
                /* -------------------------------------------------------------------------- */
                
                .register_type::<CombatState>()
                // .register_type::<CombatResources>()
                // .register_type::<Action>()
                
                .register_type::<ActionCount>()

                // .register_type::<TacticalPosition>()
                .register_type::<TacticalPlace>()
                
                /* -------------------------------------------------------------------------- */
                /*                       --- Skills and Alterations ---                       */
                /* -------------------------------------------------------------------------- */

                .register_type::<Alteration>()
                .register_type::<AlterationAction>()
                .register_type::<TargetOption>()
                
                // .register_type::<Skill>()
                .register_type::<SkillType>()
                
                /* -------------------------------------------------------------------------- */
                /*                               --- Weapons ---                              */
                /* -------------------------------------------------------------------------- */
                
                .register_type::<Equipements>()
                .register_type::<WeaponType>()
                
                .register_type::<Job>()
                .register_type::<MasteryTier>()
                // .register_type::<JobsMasteries>()
                
                /* -------------------------------------------------------------------------- */
                /*                                --- Stats ---                               */
                /* -------------------------------------------------------------------------- */
                
                .register_type::<Hp>()
                .register_type::<Mana>()
                .register_type::<Shield>()
                .register_type::<Initiative>()
                .register_type::<Attack>()
                .register_type::<AttackSpe>()
                .register_type::<Defense>()
                .register_type::<DefenseSpe>()

                /* -------------------------------------------------------------------------- */
                /*                                 --- UI ---                                 */
                /* -------------------------------------------------------------------------- */

                .register_type::<ActionHistory>()
                .register_type::<LastTurnActionHistory>()
                .register_type::<ActionsLogs>()

                .add_plugins((
                    ResourceInspectorPlugin::<ActionHistory>::default(),
                    ResourceInspectorPlugin::<LastTurnActionHistory>::default(),
                    ResourceInspectorPlugin::<ActionsLogs>::default(),
                ))
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
