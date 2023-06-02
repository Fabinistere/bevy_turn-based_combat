//! List all the technic possible

use crate::combat::skills::TargetOption;

use super::alterations::{Alteration, AlterationAction};

impl Alteration {
    // ------ Debuff ------

    // --- IO / InflictSuffer ---

    pub fn honte() -> Self {
        Alteration {
            action: AlterationAction::StatsPercentage,
            duration: 2,
            target_option: TargetOption::Enemy(1),
            damage_suffered: 25,
            description: String::from("25% dmg subie en +"),
            name: String::from("Honte"),
            ..Default::default()
        }
    }

    // ------ Buff ------

    // --- IO / InflictSuffer ---

    pub fn harmonize() -> Self {
        Alteration {
            action: AlterationAction::StatsPercentage,
            duration: 2,
            target_option: TargetOption::Enemy(1),
            heal_received: 25,
            description: String::from("25% de soin reçu en +"),
            name: String::from("Harmonize"),
            ..Default::default()
        }
    }

    // --- Heal ---

    pub fn regenerate() -> Self {
        Alteration {
            action: AlterationAction::Dots,
            duration: 3,
            target_option: TargetOption::Ally(1),
            hp: 10,
            description: String::from("10hp per turn for 3turns"),
            name: String::from("Regenerate"),
            ..Default::default()
        }
    }

    // --- Stats ---

    pub fn swiftness() -> Self {
        Alteration {
            action: AlterationAction::StatsFlat,
            duration: 3,
            target_option: TargetOption::Ally(1),
            initiative: 30,
            description: "Grant +30initiative for 3turns".to_string(),
            name: "Swiftness".to_string(),
            ..Default::default()
        }
    }

    pub fn hardness() -> Self {
        Alteration {
            action: AlterationAction::StatsFlat,
            duration: 3,
            target_option: TargetOption::Ally(1),
            initiative: 30,
            defense: 15,
            description: "Grant +15defense for 3turns".to_string(),
            name: "Hardness".to_string(),
            ..Default::default()
        }
    }
}
