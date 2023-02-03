use bevy::prelude::*;

use crate::{combat::{InCombat, CombatPanel, CombatState}, constants::ui::dialogs::*, ui::player_interaction::Clicked};

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
        info!("{} remove clicked to be selected", _name);
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
        info!("{} remove clicked to be targeted", _name);
    }
}

pub fn target_random_system(
    mut commands: Commands,

    mut button_system: Query<
        (Entity, &Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            With<Button>,
            With<ButtonTargeting>,
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

    mut combat_panel_query: Query<(Entity, &mut CombatPanel)>,
) {
    for event in event_query.iter() {
        match combat_unit_query.get(event.0) {
            Err(e) => warn!(
                "The entity selected is invalid or already selected: {:?}",
                e
            ),
            Ok((character, _name)) => {
                commands.entity(character).insert(Selected);
                info!("{} selected", _name);

                // remove from previous entity the selected component
                for (selected, _) in selected_unit_query.iter() {
                    commands.entity(selected).remove::<Selected>();
                }

                let (_, mut combat_panel) = combat_panel_query.single_mut();
                    combat_panel.phase = CombatState::SelectionSkills;
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

    mut combat_panel_query: Query<(Entity, &mut CombatPanel)>,
) {
    for event in event_query.iter() {
        // REFACTOR: ? does this match is mandatory ? can just add Selected to the unit. XXX
        // same in update_seleted_unit
        match combat_unit_query.get(event.0) {
            Err(e) => warn!("The entity targeted is invalid: {:?}", e),
            Ok((character, _name)) => {
                commands.entity(character).insert(Targeted);
                info!("{} targeted", _name);

                // TODO: feature - possibility to target multiple depending to the skill selected
                // ^^--play with run criteria

                // remove from previous entity the targeted component
                for (targeted, _) in targeted_unit_query.iter() {
                    commands.entity(targeted).remove::<Targeted>();
                }

                let (_, mut combat_panel) = combat_panel_query.single_mut();
                    combat_panel.phase = CombatState::SelectionSkills;
            }
        }
    }
}

/// Display the current phase
/// 
/// # Note
/// 
/// DEBUG
pub fn update_combat_phase_displayer(
    mut combat_panel_query: Query<(Entity, &CombatPanel, &mut Text), Or<(Added<CombatPanel>, Changed<CombatPanel>)>>,
) {
    if let Ok((_, combat_panel, mut text)) = combat_panel_query.get_single_mut() {
        let phase_display = format!("Combat Phase: {}", combat_panel.phase);
        text.sections[0].value = phase_display;
    }
}
