use bevy::prelude::*;

use crate::{
    constants::ui::dialogs::*,
    combat::{
        stats::{Hp, Mana},
        InCombat
    },
};

#[derive(Component)]
pub struct ButtonSelection;

#[derive(Component)]
pub struct UnitSelected(pub Option<Entity>);

#[derive(Component)]
pub struct UnitTargeted(pub Option<Entity>);

#[derive(Component)]
pub struct HpMeter;

#[derive(Component)]
pub struct MpMeter;

pub fn select_unit_system(
    mut button_system: Query<
        (Entity, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<ButtonSelection>),
    >,

    combat_unit_query: Query<
        Entity,
        With<InCombat>
    >,

    mut selected_query: Query<
        (Entity, &mut UnitSelected)
    >,
) {
    for (_button, interaction, mut color) in &mut button_system {
        match *interaction {
            Interaction::Clicked => {
                // DEBUG: TEMPORARY SELECTION
                for npc in combat_unit_query.iter() {
                    let (_, mut unit_selected) = selected_query.single_mut();
                    unit_selected.0 = Some(npc);
            
                    break;
                }

                *color = PRESSED_BUTTON.into();
            }
            // TODO: feature - preview
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn update_stats_displayer(
    mut hp_query: Query<
        (Entity, &HpMeter, &mut Text),
        Without<MpMeter>
    >,
    mut mp_query: Query<
        (Entity, &MpMeter, &mut Text),
        Without<HpMeter>
    >,

    selected_query: Query<
        (Entity, &UnitSelected, &Children),
        Or<(Added<UnitSelected>, Changed<UnitSelected>)>
    >,

    combat_unit_query: Query<(Entity, &Name, &Hp, &Mana), With<InCombat>>,
) {
    // REFACTOR: Never Nester ? ModCheck
    for (_, unit_selected, children) in selected_query.iter() {
        match unit_selected.0 {
            None => {}
            Some(unit) => {
                match combat_unit_query.get(unit) {
                    Err(_) => {}
                    Ok((_, _, hp, mana)) => {
                        for child in children.iter() {
                            match hp_query.get_mut(*child) {
                                Err(_) => {}
                                Ok((_, _, mut text))=> {
                                    let hp_display =
                                        String::from("hp: HEALTH")
                                            .replace("HEALTH", &hp.current_hp.to_string());
                                    text.sections[0].value = hp_display;

                                    // skip next match
                                    continue;
                                }
                            }
                            match mp_query.get_mut(*child) {
                                Err(_) => {}
                                Ok((_, _, mut text)) => {
                                    let mp_display =
                                        String::from("mp: MANA")
                                            .replace("MANA", &mana.current_mana.to_string());
                                    text.sections[0].value = mp_display;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}