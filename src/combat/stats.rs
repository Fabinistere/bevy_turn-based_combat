//! Implement all Combat stats

use bevy::prelude::*;
use rand::Rng;

use crate::npc::NPC;

/// Each entity which can be involved in a combat has this Bundle
#[derive(Bundle)]
pub struct StatBundle {
    pub hp: Hp,
    pub mana: Mana,
    pub shield: Shield,
    pub initiative: Initiative,
    pub attack: Attack,
    pub attack_spe: AttackSpe,
    /// Physical Resistance
    pub defense: Defense,
    /// Magical Resistance
    pub defense_spe: DefenseSpe,
}

impl Default for StatBundle {
    fn default() -> Self {
        StatBundle {
            hp: Hp::default(),
            mana: Mana::default(),
            shield: Shield::default(),
            initiative: Initiative::default(),
            attack: Attack::default(),
            attack_spe: AttackSpe::default(),
            defense: Defense::default(),
            defense_spe: DefenseSpe::default(),
        }
    }
}

/// ----------Hp----------
///
/// Start of the Game: 50hp -> End of the Game: 1 000hp.
///
/// Can be modified by level, item, buff, debuff, technics.
///
/// # Note
///
/// At the moment, current_hp <= max_hp
#[derive(Component)]
pub struct Hp {
    pub current_hp: i32,
    pub max_hp: i32,
}

impl Default for Hp {
    fn default() -> Self {
        Hp {
            current_hp: 50,
            max_hp: 50,
        }
    }
}

// TODO: a hp bar close to the entity
pub fn show_hp(npc_query: Query<(&Hp, &Name), With<NPC>>) {
    for (npc_hp, npc_name) in npc_query.iter() {
        info!(
            "DEBUG: {}'s Hp: {}/{},",
            npc_name, npc_hp.current_hp, npc_hp.max_hp
        );
    }
}

/// ----------Mana----------
///
/// Start of the Game: 0-100mana -> End of the Game: 10 000mana.
///
/// Can be modified by level, item, buff, debuff, technics.
///
/// # Note
///
/// At the moment, current_mana <= max_mana
#[derive(Component)]
pub struct Mana {
    pub current_mana: i32,
    pub max_mana: i32,
}

impl Default for Mana {
    fn default() -> Self {
        Mana {
            current_mana: 50,
            max_mana: 50,
        }
    }
}

// TODO: a mana bar close to the entity
pub fn show_mana(npc_query: Query<(&Mana, &Name), With<NPC>>) {
    for (npc_mana, npc_name) in npc_query.iter() {
        info!(
            "DEBUG: {}'s Mana: {}/{},",
            npc_name, npc_mana.current_mana, npc_mana.max_mana
        );
    }
}

/// ----------Shield----------
///
/// Start of the Game: 0-100shield -> End of the Game: 10 000shield.
///
/// Can be modified by level, item, buff, debuff, technics.
#[derive(Component)]
pub struct Shield (pub i32);

impl Default for Shield {
    fn default() -> Self {
        Shield(0)
    }
}

/// ----------Attack----------
///
/// Start of the Game: 10-20 -> End of the Game: ~.
///
/// Can be modified by level, item, buff, debuff, technics.
///
/// This statistic is fix, it increment the martial technic's power.
#[derive(Component)]
pub struct Attack(pub i32);

impl Default for Attack {
    fn default() -> Self {
        Attack(10)
    }
}

/// ----------Attack Spe----------
/// 
/// Start of the Game: 0-30 -> End of the Game: ~
/// 
/// Can be modified by level, item, buff, debuff, technics.
/// 
/// This statistic is fix, it increment the magic technic's power.
#[derive(Component)]
pub struct AttackSpe(pub i32);

impl Default for AttackSpe {
    fn default() -> Self {
        AttackSpe(0)
    }
}

/// ----------Defense----------
/// 
/// Start of the Game: 0-10 -> End of the Game: ~
/// 
/// Can be modified by level, item, buff, debuff, technics.
/// 
/// This statistic has a logarithmic behavior.
/// 
/// Used to calculate the reduced damage (in percentage)
/// taken from basic attacks and abilities that deal physical damage.
/// 
/// Calculated by armor ?? (armor + 100).
#[derive(Component)]
pub struct Defense(pub i32);

impl Default for Defense {
    fn default() -> Self {
        Defense(10)
    }
}

/// ----------Defense Spe----------
/// 
/// Start of the Game: 0-10 -> End of the Game: ~
/// 
/// Can be modified by level, item, buff, debuff, technics.
/// 
/// This statistic has a logarithmic behavior.
/// 
/// Used to calculate the reduced damage (in percentage)
/// taken from basic attacks and abilities that deal magical damage.
/// 
/// Calculated by MR ?? (MR + 100).
#[derive(Component)]
pub struct DefenseSpe(pub i32);

impl Default for DefenseSpe {
    fn default() -> Self {
        DefenseSpe(0)
    }
}

/// ----------INITIATIVE----------
/// 
/// Minimun initiative: 0 -> Maximun initiative: 100
/// 
/// Indicate the speed of initiative, the entity has.
/// The more they has, the more likly they will start their turn first.
#[derive(Component)]
pub struct Initiative(pub i32);

impl Default for Initiative {
    fn default() -> Self {
        Initiative(20)
    }
}

/// Roll for each entity a d100 ranged into +-20 initiative
/// ALso Display the final score
///
/// Sort the result in a nice table
/// In case of egality: pick the higher initiative boyo to be on top
pub fn roll_initiative(npc_query: Query<&Initiative, With<NPC>>) {
    let mut initiatives: Vec<i32> = Vec::new();

    for npc_init in npc_query.iter() {
        let npc_number;

        if npc_init.0 - 20 <= 0 {
            npc_number = rand::thread_rng().gen_range(0..npc_init.0 + 20);
        } else if npc_init.0 == 100 {
            npc_number = 100;
        } else if npc_init.0 + 20 >= 100 {
            npc_number = rand::thread_rng().gen_range(npc_init.0 - 20..100);
        } else {
            npc_number = rand::thread_rng().gen_range(npc_init.0 - 20..npc_init.0 + 20);
        }

        // insert these number in a vector
        initiatives.push(npc_number);
    }

    initiatives.sort();

    info!("DEBUG: Initiative: {:?}", initiatives);
}

/// ----------ACCURACY----------
/// 
/// Used to calculate if the technic will hit (in percentage).
#[derive(Component)]
pub struct Accuracy(pub i32);

impl Default for Accuracy {
    fn default() -> Self {
        Accuracy(95)
    }
}

/// ----------CRITICAL----------
/// 
/// Used to calculate if the technic will be critical (in percentage).
/// 
/// A Critical technic has its dmg inflicted multiplied by 300%
/// 
/// ONLY allow critics on hit
#[derive(Component)]
pub struct Critical(pub i32);

impl Default for Critical {
    fn default() -> Self {
        Critical(1)
    }
}
