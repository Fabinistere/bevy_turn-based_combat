//! Display the Character Sheet
//! All Stats and Skills

use bevy::prelude::*;

use crate::{
    combat::{
        phases::TransitionPhaseEvent,
        skills::Skill,
        stats::{Hp, Mana},
        stuff::{Equipement, Equipements, SkillTiers},
        Action, CombatPanel, CombatState, InCombat, Skills,
    },
    ui::{
        combat_panel::{CasterMeter, SkillDisplayer, TargetMeter},
        combat_system::{HpMeter, MpMeter, Selected, Targeted},
    },
};

/// Action for each Interaction of the skill button
///
/// # Note
///
/// DOC: Move select_skill() in player_interaction
pub fn select_skill(
    mut interaction_query: Query<
        (&Interaction, &Skill, &Children),
        (
            Changed<Interaction>,
            With<Button>,
            // Without<ButtonTargeting>,
        ),
    >,

    mut text_query: Query<&mut Text>,

    mut combat_panel_query: Query<(Entity, &mut CombatPanel)>,

    unit_selected_query: Query<(Entity, &Name, &Selected)>,
    mut transition_phase_event: EventWriter<TransitionPhaseEvent>,
) {
    for (interaction, skill, children) in &mut interaction_query {
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

                    // let (_, caster_name, _) = unit_selected_query.single();
                    // info!("DEBUG: action = {} do {} to None", caster_name, skill.name);

                    // info!("rewrite last action");

                    // and we're still in TargetSelection phase
                } else {
                    // if this system can run
                    // we are in SelectionSkill or SelectionTarget
                    // so there is a selected unit.
                    // FIXME: Crash - Esc bug, after cancel an action but still in selectionSkill with no action left
                    let (caster, _caster_name, _) = unit_selected_query.single();

                    transition_phase_event.send(TransitionPhaseEvent(CombatState::SelectionTarget));

                    let action = Action::new(caster, skill.clone(), None);
                    combat_panel.history.push(action);

                    // info!("DEBUG: action = {} do {} to None", _caster_name, skill.name);
                    // info!("new action");
                }

                let display = skill.name.replace("a", "o").replace("A", "O");
                text.sections[0].value = display;

                info!("Skill {} selected", skill.name);
            }
            Interaction::Hovered => {
                // TODO: feature - Hover Skill - Preview possible Target

                text.sections[0].value = skill.name.clone();
            }
            Interaction::None => {
                text.sections[0].value = skill.name.clone();
            }
        }
    }
}

/// # Note
///
/// DEBUG
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

    mut select_removals: RemovedComponents<Selected>,

    mut hp_query: Query<(Entity, &HpMeter, &mut Text), (Without<MpMeter>, With<CasterMeter>)>,
    mut mp_query: Query<(Entity, &MpMeter, &mut Text), (Without<HpMeter>, With<CasterMeter>)>,
) {
    let (_, _, mut hp_text) = hp_query.single_mut();
    let (_, _, mut mp_text) = mp_query.single_mut();

    for _ in select_removals.iter() {
        let hp_display = String::from("Caster hp: ??");
        let mp_display = String::from("Caster mp: ??");

        hp_text.sections[0].value = hp_display;
        mp_text.sections[0].value = mp_display;
    }

    if let Ok((_, _, name, hp, mana)) = selected_query.get_single() {
        let hp_display = format!("Caster {} hp: {}", name, &hp.current.to_string());
        let mp_display = format!("Caster {} mp: {}", name, &mana.current.to_string());

        hp_text.sections[0].value = hp_display;
        mp_text.sections[0].value = mp_display;
    }
}

/// # Note
///
/// DEBUG
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

    mut target_removals: RemovedComponents<Targeted>,

    mut hp_query: Query<(Entity, &HpMeter, &mut Text), (Without<MpMeter>, With<TargetMeter>)>,
    mut mp_query: Query<(Entity, &MpMeter, &mut Text), (Without<HpMeter>, With<TargetMeter>)>,
) {
    for (_, _, name, hp, mana) in targeted_query.iter() {
        let (_, _, mut hp_text) = hp_query.single_mut();
        let (_, _, mut mp_text) = mp_query.single_mut();

        let hp_display = format!("Target {} hp: {}", name, &hp.current.to_string());
        hp_text.sections[0].value = hp_display;

        let mp_display = format!("Target {} mp: {}", name, &mana.current.to_string());
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

/// Determine the visibility of the 6 skills
///
/// Update these values when the entity selected changed
///
/// # Note
///
/// REFACTOR: Maybe find some new ways to sequence these affectations better
pub fn skill_visibility(
    mut selection_removal_query: RemovedComponents<Selected>,
    caster_query: Query<(&Equipements, &Skills), (With<Selected>, With<InCombat>, Added<Selected>)>,
    weapon_query: Query<&SkillTiers, With<Equipement>>,

    mut skill_displayer_query: Query<(&SkillDisplayer, &mut Skill, &mut Visibility, &Children)>,
    mut text_query: Query<&mut Text>,
) {
    // If there was a transition, a changement in the one being Selected
    // Reset all Skill
    for _ in selection_removal_query.iter() {
        for (skill_number, mut skill, mut visibility, children) in skill_displayer_query.iter_mut()
        {
            // --- Text ---
            let mut text = text_query.get_mut(children[0]).unwrap();
            text.sections[0].value = "Pass".to_string();
            *skill = Skill::pass();

            // --- Visibility ---
            let old_visibility = visibility.clone();
            *visibility = Visibility::Hidden;

            // --- Logs ---
            if old_visibility != *visibility {
                // DEBUG: Skills' Visibility switcher
                info!(
                    "skill °{} visibility switch: {:?}",
                    skill_number.0, *visibility
                );
            }
        }
    }

    // Set the visibility w.r.t. the new caster (entity selected)
    if let Ok((Equipements { weapon, armor: _ }, skills)) = caster_query.get_single() {
        let mut caster_skill_count = skills.len();

        for (skill_number, mut skill, mut visibility, children) in skill_displayer_query.iter_mut()
        {
            let old_visibility = visibility.clone();
            // --- Text ---
            let mut text = text_query.get_mut(children[0]).unwrap();

            match weapon {
                None => {}
                Some(weapon_entity) => {
                    if let Ok(SkillTiers {
                        tier_2,
                        tier_1,
                        tier_0,
                    }) = weapon_query.get(*weapon_entity)
                    {
                        // TODO: determine which tier is the caster % this weapon
                        caster_skill_count += tier_2.len() + tier_1.len() + tier_0.len();
                    }
                }
            }

            *visibility = if skill_number.0 < caster_skill_count {
                *skill = if skill_number.0 < skills.len() {
                    text.sections[0].value = skills[skill_number.0].clone().name;

                    skills[skill_number.0].clone()
                }
                // weapon's skills
                else {
                    // REFACTOR: Weapon tier displayer - Aled
                    match weapon {
                        None => Skill::pass(),
                        Some(weapon_entity) => {
                            if let Ok(SkillTiers {
                                tier_2,
                                tier_1,
                                tier_0,
                            }) = weapon_query.get(*weapon_entity)
                            {
                                // TODO: determine which tier is the caster % this weapon
                                if skill_number.0 < skills.len() + tier_2.len() {
                                    text.sections[0].value =
                                        tier_2[skill_number.0 - skills.len()].clone().name;

                                    tier_2[skill_number.0 - skills.len()].clone()
                                } else if skill_number.0
                                    < skills.len() + tier_2.len() + tier_1.len()
                                {
                                    text.sections[0].value = tier_2
                                        [skill_number.0 - skills.len() - tier_2.len()]
                                    .clone()
                                    .name;

                                    tier_1[skill_number.0 - skills.len() - tier_2.len()].clone()
                                } else if skill_number.0
                                    < skills.len() + tier_2.len() + tier_1.len() + tier_0.len()
                                {
                                    text.sections[0].value = tier_2[skill_number.0
                                        - skills.len()
                                        - tier_2.len()
                                        - tier_1.len()]
                                    .clone()
                                    .name;

                                    tier_0[skill_number.0
                                        - skills.len()
                                        - tier_2.len()
                                        - tier_1.len()]
                                    .clone()
                                } else {
                                    panic!("skill_number.0: {} >= caster_skill_count: {} AND skill_number.0 < caster_skill_count", skill_number.0, caster_skill_count);
                                    // Skill::pass()
                                }
                            } else {
                                warn!("No SkillTiers in the Weapon");
                                Skill::pass()
                            }
                        }
                    }
                };

                // --- Visibility ---
                Visibility::Inherited
            } else {
                // vv-- "useless" --vv
                text.sections[0].value = "Pass".to_string();
                *skill = Skill::pass();

                // --- Visibility ---
                Visibility::Hidden
            };

            // --- Logs ---
            if old_visibility != *visibility {
                // DEBUG: Skills' Visibility switcher
                info!(
                    "skill °{} visibility switch: {:?}",
                    skill_number.0, *visibility
                );
            }
        }
    }
}
