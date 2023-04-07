//! List all the technic possible

use crate::combat::skills::TargetSide;

use super::alterations::{Alteration, AlterationAction};

impl Alteration {
    pub fn honte() -> Self {
        Alteration {
            action: AlterationAction::StatsPercentage,
            duration: 2,
            target_option: (TargetSide::Enemy, 1),
            damage_suffered: 25,
            description: String::from("25% dmg subie en +"),
            name: String::from("Honte"),
            ..Default::default()
        }
    }

    pub fn harmonize() -> Self {
        Alteration {
            action: AlterationAction::StatsPercentage,
            duration: 2,
            target_option: (TargetSide::Enemy, 1),
            heal_received: 25,
            description: String::from("25% de soin reçu en +"),
            name: String::from("Harmonize"),
            ..Default::default()
        }
    }
}
