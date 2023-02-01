use bevy::prelude::*;

use crate::{
    combat::{
        stats::{Hp, Mana},
        InCombat,
    },
    constants::ui::dialogs::*,
    ui::combat_panel::{CasterMeter, TargetMeter},
};

use super::player_interaction::Clicked;

#[derive(Component)]
pub struct ButtonSelection;

#[derive(Component)]
pub struct ButtonTargeting;

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct Targeted;

#[derive(Component)]
pub struct HpMeter;

#[derive(Component)]
pub struct MpMeter;

/// DOC
pub struct UpdateUnitSelectedEvent(pub Entity);

/// DOC
pub struct UpdateUnitTargetedEvent(pub Entity);

/// # Note
/// TODO: Unit Clicked (Selected)
pub fn caster_selection(
    mut commands: Commands,

    selectable_unit_query: Query<(Entity, &Name), (With<Clicked>, With<InCombat>)>,

    mut update_unit_selected_event: EventWriter<UpdateUnitSelectedEvent>,
) {
    for (entity, _name) in selectable_unit_query.iter() {
        update_unit_selected_event.send(UpdateUnitSelectedEvent(entity));

        commands.entity(entity).remove::<Clicked>();
    }
}

/// # Note
/// TODO: Unit Clicked (Targeted)
pub fn target_selection(
    mut commands: Commands,

    targetable_unit_query: Query<(Entity, &Name), (With<Clicked>, With<InCombat>)>,

    mut update_unit_targeted_event: EventWriter<UpdateUnitTargetedEvent>,
) {
    for (entity, _name) in targetable_unit_query.iter() {
        update_unit_targeted_event.send(UpdateUnitTargetedEvent(entity));

        commands.entity(entity).remove::<Clicked>();
    }
}

pub fn select_unit_system(
    mut commands: Commands,

    mut button_system: Query<
        (Entity, &Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            With<Button>,
            With<ButtonSelection>,
            Without<ButtonTargeting>,
        ),
    >,

    combat_unit_query: Query<(Entity, &Name), (With<InCombat>, Without<Selected>)>,
    selected_unit: Query<Entity, With<Selected>>,

    mut update_unit_selected_event: EventWriter<UpdateUnitSelectedEvent>,
) {
    for (_button, interaction, mut color) in &mut button_system {
        match *interaction {
            Interaction::Clicked => {
                for (npc, _name) in combat_unit_query.iter() {
                    // select the first one on the list

                    if let Ok(selected) = selected_unit.get_single() {
                        commands.entity(selected).remove::<Selected>();
                    }

                    // DEBUG: TEMPORARY SELECTION
                    update_unit_selected_event.send(UpdateUnitSelectedEvent(npc));

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

pub fn target_unit_system(
    mut commands: Commands,

    mut button_system: Query<
        (Entity, &Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            With<Button>,
            With<ButtonTargeting>,
            Without<ButtonSelection>,
        ),
    >,

    combat_unit_query: Query<(Entity, &Name), (With<InCombat>, Without<Targeted>)>,
    targeted_unit: Query<Entity, With<Targeted>>,

    mut update_unit_targeted_event: EventWriter<UpdateUnitTargetedEvent>,
) {
    for (_button, interaction, mut color) in &mut button_system {
        match *interaction {
            Interaction::Clicked => {
                for (npc, _name) in combat_unit_query.iter() {
                    // target the first one on the list
                    if let Ok(targeted) = targeted_unit.get_single() {
                        commands.entity(targeted).remove::<Targeted>();
                    }
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
    mut commands: Commands,

    mut event_query: EventReader<UpdateUnitSelectedEvent>,

    combat_unit_query: Query<(Entity, &Name), (Without<Selected>, With<InCombat>)>,
    selected_unit_query: Query<(Entity, &Name), With<Selected>>,
) {
    for event in event_query.iter() {
        match combat_unit_query.get(event.0) {
            Err(e) => warn!(
                "The entity selected is invalid or already selected: {:?}",
                e
            ),
            Ok((character, _name)) => {
                commands.entity(character).insert(Selected);
                // remove from previous entity the selected component
                for (selected, _) in selected_unit_query.iter() {
                    commands.entity(selected).remove::<Selected>();
                }
            }
        }
    }
}

/// Event Handler of UpdateUnitSelectedEvent
pub fn update_targeted_unit(
    mut commands: Commands,

    mut event_query: EventReader<UpdateUnitTargetedEvent>,

    combat_unit_query: Query<(Entity, &Name), With<InCombat>>,
    targeted_unit_query: Query<(Entity, &Name), With<Targeted>>,
) {
    for event in event_query.iter() {
        // REFACTOR: ? does this match is mandatory ? can just add Selected to the unit. XXX
        // same in update_seleted_unit
        match combat_unit_query.get(event.0) {
            Err(e) => warn!("The entity targeted is invalid: {:?}", e),
            Ok((character, _name)) => {
                commands.entity(character).insert(Targeted);
                
                // TODO: feature - possibility to target multiple depending to the skill selected
                // ^^--play with run criteria

                // remove from previous entity the targeted component
                for (targeted, _) in targeted_unit_query.iter() {
                    commands.entity(targeted).remove::<Targeted>();
                }
            }
        }
    }
}

pub fn update_caster_stats_panel(
    selected_query: Query<
        (Entity, &Selected, &Name, &Hp, &Mana),
        (
            Or<(
                Added<Selected>,
                Changed<Selected>,
                Changed<Hp>,
                Changed<Mana>,
            )>,
            With<InCombat>,
        ),
    >,

    select_removals: RemovedComponents<Selected>,

    mut hp_query: Query<(Entity, &HpMeter, &mut Text), (Without<MpMeter>, With<CasterMeter>)>,
    mut mp_query: Query<(Entity, &MpMeter, &mut Text), (Without<HpMeter>, With<CasterMeter>)>,
) {
    for (_, _, name, hp, mana) in selected_query.iter() {
        let (_, _, mut hp_text) = hp_query.single_mut();
        let (_, _, mut mp_text) = mp_query.single_mut();

        let hp_display = format!("Caster {} hp: {}", name, &hp.current_hp.to_string());
        hp_text.sections[0].value = hp_display;

        let mp_display = format!("Caster {} mp: {}", name, &mana.current_mana.to_string());
        mp_text.sections[0].value = mp_display;
    }

    // TODO: when Selected is removed
    for _entity in select_removals.iter() {
        let (_, _, mut hp_text) = hp_query.single_mut();
        let (_, _, mut mp_text) = mp_query.single_mut();

        let hp_display = String::from("Caster hp: ??");
        hp_text.sections[0].value = hp_display;

        let mp_display = String::from("Caster mp: ??");
        mp_text.sections[0].value = mp_display;
    }
}

/// XXX: A proper clone of update_caster_stats_panel but just for target instead of caster
pub fn update_target_stats_panel(
    targeted_query: Query<
        (Entity, &Targeted, &Name, &Hp, &Mana),
        (
            Or<(
                Added<Targeted>,
                Changed<Targeted>,
                Changed<Hp>,
                Changed<Mana>,
            )>,
            With<InCombat>,
        ),
    >,

    target_removals: RemovedComponents<Targeted>,

    mut hp_query: Query<(Entity, &HpMeter, &mut Text), (Without<MpMeter>, With<TargetMeter>)>,
    mut mp_query: Query<(Entity, &MpMeter, &mut Text), (Without<HpMeter>, With<TargetMeter>)>,
) {
    for (_, _, name, hp, mana) in targeted_query.iter() {
        let (_, _, mut hp_text) = hp_query.single_mut();
        let (_, _, mut mp_text) = mp_query.single_mut();

        let hp_display = format!("Target {} hp: {}", name, &hp.current_hp.to_string());
        hp_text.sections[0].value = hp_display;

        let mp_display = format!("Target {} mp: {}", name, &mana.current_mana.to_string());
        mp_text.sections[0].value = mp_display;
    }

    // TODO: when Targeted is removed
    for _entity in target_removals.iter() {
        let (_, _, mut hp_text) = hp_query.single_mut();
        let (_, _, mut mp_text) = mp_query.single_mut();

        let hp_display = String::from("Target hp: ??");
        hp_text.sections[0].value = hp_display;

        let mp_display = String::from("Target mp: ??");
        mp_text.sections[0].value = mp_display;
    }
}
