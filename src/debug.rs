use bevy::prelude::*;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

use crate::{
    combat::{
        alterations::{Alteration, AlterationAction},
        skills::{TargetSide, Skill, SkillType},
        Alterations, stats::{Hp, Mana, Shield, Initiative, Attack, AttackSpe, Defense, DefenseSpe}, CombatState,
    },
    npc::NPC,
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new())
                .register_inspectable::<NPC>()

                .register_inspectable::<CombatState>()

                .register_inspectable::<Alteration>()
                .register_inspectable::<Alterations>()
                .register_inspectable::<AlterationAction>()
                .register_inspectable::<TargetSide>()
                
                .register_inspectable::<Skill>()
                .register_inspectable::<SkillType>()
                
                // stats
                
                .register_inspectable::<Hp>()
                .register_inspectable::<Mana>()
                .register_inspectable::<Shield>()
                .register_inspectable::<Initiative>()
                .register_inspectable::<Attack>()
                .register_inspectable::<AttackSpe>()
                .register_inspectable::<Defense>()
                .register_inspectable::<DefenseSpe>()

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
