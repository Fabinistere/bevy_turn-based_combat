use bevy::prelude::*;

use crate::{
    constants::ui::dialogs::*,
    combat::{
        stats::{Hp, Mana},
        InCombat
    },
};

use super::dialog_panel::{CasterMeter, TargetMeter};

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

/// DOC
pub struct UpdateUnitSelectedEvent(pub Entity);

/// DOC
pub struct UpdateUnitTargetedEvent(pub Entity);

pub fn select_unit_system(
    mut button_system: Query<
        (Entity, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<ButtonSelection>),
    >,

    combat_unit_query: Query<
        (Entity, &Name),
        With<InCombat>
    >,

    mut update_unit_selected_event: EventWriter<UpdateUnitSelectedEvent>,
    mut update_unit_targeted_event: EventWriter<UpdateUnitTargetedEvent>,
) {
    for (_button, interaction, mut color) in &mut button_system {
        match *interaction {
            Interaction::Clicked => {
                for (npc, _name) in combat_unit_query.iter() {

                    // select and target the first one on the list

                    // DEBUG: TEMPORARY SELECTION
                    update_unit_selected_event.send(UpdateUnitSelectedEvent(npc));

                    // DEBUG: TEMPORARY TARGET
                    update_unit_targeted_event.send(UpdateUnitTargetedEvent(npc));

                    break;
                }

                *color = PRESSED_BUTTON.into();
            }
            // TODO: feature - preview
            // Store the previous selected in the temp and restore it when none
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

/// Event Handler of UpdateUnitSelectedEvent
pub fn update_selected_unit(
    mut event_query: EventReader<UpdateUnitSelectedEvent>,

    combat_unit_query: Query<
        (Entity, &Name),
        With<InCombat>
    >,

    mut selected_query: Query<
        (Entity, &mut UnitSelected, &mut Text)
    >,
) {
    for event in event_query.iter() {
        match combat_unit_query.get(event.0) {
            Err(e) => warn!("The entity selected is invalid: {:?}", e),
            Ok((character, name)) => {
                let (_, mut unit_selected, mut unit_selected_text) = selected_query.single_mut();
                unit_selected.0 = Some(character);
                unit_selected_text.sections[0].value = format!("Unit Selected: {}", name);
                // TODO: Update stats panel
            }
        }
    }
}

/// Event Handler of UpdateUnitSelectedEvent
pub fn update_targeted_unit(
    mut event_query: EventReader<UpdateUnitTargetedEvent>,

    combat_unit_query: Query<
        (Entity, &Name),
        With<InCombat>
    >,

    mut targeted_query: Query<
        (Entity, &mut UnitTargeted, &mut Text)
    >,
) {
    for event in event_query.iter() {
        match combat_unit_query.get(event.0) {
            Err(e) => warn!("The entity targeted is invalid: {:?}", e),
            Ok((character, name)) => {
                let (_, mut unit_targeted, mut unit_targeted_text) = targeted_query.single_mut();
                unit_targeted.0 = Some(character);
                unit_targeted_text.sections[0].value = format!("Unit Targeted: {}", name);
                // TODO: Update stats panel
            }
        }
    }
}

/// FIXME: don't update when the caster/target stats changes
pub fn update_caster_stats_panel (
    selected_query: Query<
        (Entity, &UnitSelected),
        Or<(Added<UnitSelected>, Changed<UnitSelected>)>
    >,

    mut hp_query: Query<
        (Entity, &HpMeter, &mut Text),
        (Without<MpMeter>, With<CasterMeter>)
    >,
    mut mp_query: Query<
        (Entity, &MpMeter, &mut Text),
        (Without<HpMeter>, With<CasterMeter>)
    >,

    combat_unit_query: Query<(Entity, &Name, &Hp, &Mana), With<InCombat>>,
) {
    for (_, unit_selected) in selected_query.iter() {
        match unit_selected.0 {
            // instead of `warn!("No Caster attached"),`
            None => {
                let (_, _, mut hp_text) = hp_query.single_mut();
                let (_, _, mut mp_text) = mp_query.single_mut();

                let hp_display =
                    String::from("Caster hp: ??");
                hp_text.sections[0].value = hp_display;

                let mp_display =
                    String::from("Caster mp: ??");
                mp_text.sections[0].value = mp_display;
            },
            Some(unit) => {
                match combat_unit_query.get(unit) {
                    Err(_) => warn!("The selected unit is not a combat one"),
                    Ok((_, _, hp, mana)) => {

                        let (_, _, mut hp_text) = hp_query.single_mut();
                        let (_, _, mut mp_text) = mp_query.single_mut();

                        let hp_display =
                            String::from("Caster hp: HEALTH")
                                .replace("HEALTH", &hp.current_hp.to_string());
                        hp_text.sections[0].value = hp_display;

                        let mp_display =
                            String::from("Caster mp: MANA")
                                .replace("MANA", &mana.current_mana.to_string());
                        mp_text.sections[0].value = mp_display;
                    }
                }
            }
        }
    }
}

/// XXX: A proper clone of update_caster_stats_panel but just for target instead of caster
pub fn update_target_stats_panel (
    selected_query: Query<
        (Entity, &UnitTargeted),
        Or<(Added<UnitTargeted>, Changed<UnitTargeted>)>
    >,

    mut hp_query: Query<
        (Entity, &HpMeter, &mut Text),
        (Without<MpMeter>, With<TargetMeter>)
    >,
    mut mp_query: Query<
        (Entity, &MpMeter, &mut Text),
        (Without<HpMeter>, With<TargetMeter>)
    >,

    combat_unit_query: Query<(Entity, &Name, &Hp, &Mana), With<InCombat>>,
) {
    for (_, unit_selected) in selected_query.iter() {
        match unit_selected.0 {
            // instead of `warn!("No Target attached"),`
            None => {
                let (_, _, mut hp_text) = hp_query.single_mut();
                let (_, _, mut mp_text) = mp_query.single_mut();

                let hp_display =
                    String::from("Target hp: ??");
                hp_text.sections[0].value = hp_display;

                let mp_display =
                    String::from("Target mp: ??");
                mp_text.sections[0].value = mp_display;
            },
            Some(unit) => {
                match combat_unit_query.get(unit) {
                    Err(_) => warn!("The selected unit is not a combat one"),
                    Ok((_, _, hp, mana)) => {

                        let (_, _, mut hp_text) = hp_query.single_mut();
                        let (_, _, mut mp_text) = mp_query.single_mut();

                        let hp_display =
                            String::from("Target hp: HEALTH")
                                .replace("HEALTH", &hp.current_hp.to_string());
                        hp_text.sections[0].value = hp_display;

                        let mp_display =
                            String::from("Target mp: MANA")
                                .replace("MANA", &mana.current_mana.to_string());
                        mp_text.sections[0].value = mp_display;
                    }
                }
            }
        }
    }
}