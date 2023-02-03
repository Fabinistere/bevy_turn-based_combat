//! Display the Character Sheet
//! All Stats and Skills

use crate::{
    combat::{
        skills::Skill,
        stats::{Hp, Mana},
        InCombat, CombatPanel, CombatState, Action,
    },
    constants::ui::dialogs::*,
    ui::{
        combat_panel::{CasterMeter, TargetMeter},
        combat_system::{ButtonTargeting, HpMeter, MpMeter, Selected, Targeted},
    },
};
use bevy::prelude::*;

/// Action for each Interaction of the skill button
///
/// # Note
pub fn select_skill(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Skill, &Children),
        (
            Changed<Interaction>,
            With<Button>,
            Without<ButtonTargeting>,
        ),
    >,

    mut text_query: Query<&mut Text>,

    mut combat_panel_query: Query<(Entity, &mut CombatPanel)>,

    unit_selected_query: Query<(Entity, &Selected)>,
) {
    for (interaction, mut color, skill, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                let (_, mut combat_panel) = combat_panel_query.single_mut();
                
                // Change last action saved to the new skill selected
                if combat_panel.phase == CombatState::SelectionTarget {
                    // we already wrote the waiting skill in the actions history
                    // cause we're in the TargetSelection phase
                    
                    let mut last_action = combat_panel.history.pop().unwrap();
                    last_action.skill = skill.clone();
                    combat_panel.history.push(last_action);

                    // info!("rewrite last action");

                    // and we're still in TargetSelection phase
                } else {
                    // if this system can run
                    // we are in SelectionSkill or SelectionTarget
                    // so there is a selected unit.
                    let (caster, _) = unit_selected_query.single();
                    combat_panel.phase = CombatState::SelectionTarget;
                    let action = Action::new(caster, skill.clone(), None);
                    combat_panel.history.push(action);

                    // info!("new action");
                }

                text.sections[0].value = "BOM".to_string();
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                // TODO: feature - Hover Skill - Preview possible Target

                text.sections[0].value = "BAM".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "BAM".to_string();
                *color = NORMAL_BUTTON.into();
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

    for _entity in target_removals.iter() {
        let (_, _, mut hp_text) = hp_query.single_mut();
        let (_, _, mut mp_text) = mp_query.single_mut();

        let hp_display = String::from("Target hp: ??");
        hp_text.sections[0].value = hp_display;

        let mp_display = String::from("Target mp: ??");
        mp_text.sections[0].value = mp_display;
    }
}
