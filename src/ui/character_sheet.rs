//! Display the Character Sheet
//! All Stats and Skills

use bevy::prelude::*;

use crate::{
    characters::{FabiensInfos, PersonalInfos},
    combat::{
        alterations::AlterationAction,
        skills::Skill,
        stats::{Attack, AttackSpe, Defense, DefenseSpe, Hp, Initiative, Mana, Shield},
        stuff::{Equipement, Equipements, Job, JobsMasteries, MasteryTier, SkillTiers, WeaponType},
        ActionCount, Alterations, InCombat, Skills,
    },
    constants::ui::dialogs::*,
    ui::{
        combat_panel::{SkillBar, SkillDisplayer, TargetMeter},
        combat_system::{HpMeter, MpMeter, Selected, Targeted},
    },
};

use super::combat_panel::{FabienName, Portrait, Title, WeaponDisplayer};

/* -------------------------------------------------------------------------- */
/*                                   Headers                                  */
/* -------------------------------------------------------------------------- */

/// Update the character's sheet with the current selected, name, sprite.
pub fn update_headers(
    asset_server: Res<AssetServer>,
    fabiens_infos: Res<FabiensInfos>,

    caster_name_query: Query<
        (&Job, &Name, &TextureAtlasSprite),
        (Changed<Selected>, With<InCombat>),
    >,

    mut portrait_query: Query<(&mut UiImage, &mut Visibility), (With<Portrait>, Without<InCombat>)>,
    mut fabine_name_query: Query<
        &mut Text,
        (
            With<FabienName>,
            Without<Title>,
            Without<Job>,
            Without<InCombat>,
        ),
    >,
    mut title_query: Query<
        &mut Text,
        (
            With<Title>,
            Without<Job>,
            Without<FabienName>,
            Without<InCombat>,
        ),
    >,
    mut job_text_query: Query<
        &mut Text,
        (
            With<Job>,
            Without<Title>,
            Without<FabienName>,
            Without<InCombat>,
        ),
    >,
) {
    if let Ok((caster_job, caster_name, _caster_sprite)) = caster_name_query.get_single() {
        let (mut portrait, mut portrait_visibility) = portrait_query.single_mut();
        let mut fabien_name_text = fabine_name_query.single_mut();
        let mut title_text = title_query.single_mut();
        let mut job_text = job_text_query.single_mut();

        // portrait.index = caster_sprite.index;
        if let Some(PersonalInfos { title, sprite_path }) =
            fabiens_infos.get(&caster_name.to_string())
        {
            title_text.sections[0].value = title.to_string();
            job_text.sections[0].value = format!("{:?}", caster_job);
            portrait.texture = asset_server.load(sprite_path);
        } else {
            warn!("{} Not Found/Associated in the FabienDataBase", caster_name);
            title_text.sections[0].value = "Fabien".to_string();
            job_text.sections[0].value = "Chill".to_string();
            portrait.texture = asset_server.load("textures/character/idle/idle_Fabien_Loyal.png");
        };
        fabien_name_text.sections[0].value = caster_name.replace("NPC ", "").replace("Player ", "");
        *portrait_visibility = Visibility::Inherited;
    }
}

/* -------------------------------------------------------------------------- */
/*                                    Stats                                   */
/* -------------------------------------------------------------------------- */

/// # Note
///
/// DEBUG
/// REFACTOR: XXX: Stats Displayer
/// TODO: Add Damage Multiplier (Suffered/Inflicted)
pub fn update_caster_stats_panel(
    selected_query: Query<
        (
            &Hp,
            &Mana,
            &Shield,
            &Initiative,
            &Attack,
            &AttackSpe,
            &Defense,
            &DefenseSpe,
            &Alterations,
            // &Equipements,
        ),
        (
            Or<(Changed<Selected>, Changed<Hp>, Changed<Mana>)>,
            With<Selected>,
            With<InCombat>,
        ),
    >,

    mut select_removals: RemovedComponents<Selected>,

    mut hp_text_query: Query<
        &mut Text,
        (
            With<Hp>,
            Without<Mana>,
            Without<Shield>,
            Without<Initiative>,
            Without<Attack>,
            Without<AttackSpe>,
            Without<Defense>,
            Without<DefenseSpe>,
        ),
    >,
    mut mp_text_query: Query<
        &mut Text,
        (
            With<Mana>,
            Without<Hp>,
            Without<Shield>,
            Without<Initiative>,
            Without<Attack>,
            Without<AttackSpe>,
            Without<Defense>,
            Without<DefenseSpe>,
        ),
    >,
    mut shield_text_query: Query<
        &mut Text,
        (
            With<Shield>,
            Without<Hp>,
            Without<Mana>,
            Without<Initiative>,
            Without<Attack>,
            Without<AttackSpe>,
            Without<Defense>,
            Without<DefenseSpe>,
        ),
    >,
    mut initiative_text_query: Query<
        &mut Text,
        (
            With<Initiative>,
            Without<Hp>,
            Without<Mana>,
            Without<Shield>,
            Without<Attack>,
            Without<AttackSpe>,
            Without<Defense>,
            Without<DefenseSpe>,
        ),
    >,
    mut attack_text_query: Query<
        &mut Text,
        (
            With<Attack>,
            Without<Hp>,
            Without<Mana>,
            Without<Shield>,
            Without<Initiative>,
            Without<AttackSpe>,
            Without<Defense>,
            Without<DefenseSpe>,
        ),
    >,
    mut attack_spe_text_query: Query<
        &mut Text,
        (
            With<AttackSpe>,
            Without<Hp>,
            Without<Mana>,
            Without<Shield>,
            Without<Initiative>,
            Without<Attack>,
            Without<Defense>,
            Without<DefenseSpe>,
        ),
    >,
    mut defense_text_query: Query<
        &mut Text,
        (
            With<Defense>,
            Without<Hp>,
            Without<Mana>,
            Without<Shield>,
            Without<Initiative>,
            Without<Attack>,
            Without<AttackSpe>,
            Without<DefenseSpe>,
        ),
    >,
    mut defense_spe_text_query: Query<
        &mut Text,
        (
            With<DefenseSpe>,
            Without<Hp>,
            Without<Mana>,
            Without<Shield>,
            Without<Initiative>,
            Without<Attack>,
            Without<AttackSpe>,
            Without<Defense>,
        ),
    >,
) {
    let mut hp_text = hp_text_query.single_mut();
    let mut mp_text = mp_text_query.single_mut();
    let mut shield_text = shield_text_query.single_mut();
    let mut initiative_text = initiative_text_query.single_mut();
    let mut attack_text = attack_text_query.single_mut();
    let mut attack_spe_text = attack_spe_text_query.single_mut();
    let mut defense_text = defense_text_query.single_mut();
    let mut defense_spe_text = defense_spe_text_query.single_mut();

    for _ in select_removals.iter() {
        hp_text.sections[0].value = String::from("Health: ???/???");
        mp_text.sections[0].value = String::from("Mana: ???/???");
        shield_text.sections[0].value = String::from("Shield: ???");
        initiative_text.sections[0].value = String::from("Initiative: ???/???");
        attack_text.sections[0].value = String::from("Attack: ???/???");
        attack_spe_text.sections[0].value = String::from("AttackSpe: ???/???");
        defense_text.sections[0].value = String::from("Defense: ???/???");
        defense_spe_text.sections[0].value = String::from("DefenseSpe: ???/???");
    }

    if let Ok((
        caster_hp,
        caster_mp,
        caster_shield,
        caster_initiative,
        caster_attack,
        caster_attack_spe,
        caster_defense,
        caster_defense_spe,
        caster_alterations,
        // caster_equipments,
    )) = selected_query.get_single()
    {
        let mut attack_multiplier: f32 = 100.;
        let mut attack_spe_multiplier: f32 = 100.;
        let mut defense_multiplier: f32 = 100.;
        let mut defense_spe_multiplier: f32 = 100.;

        for alt in caster_alterations.iter() {
            match alt.action {
                AlterationAction::StatsPercentage | AlterationAction::StatsFlat => {
                    attack_multiplier += alt.attack as f32;
                    attack_spe_multiplier += alt.attack_spe as f32;
                    defense_multiplier += alt.defense as f32;
                    defense_spe_multiplier += alt.defense_spe as f32;
                }
                _ => {}
            }
        }

        hp_text.sections[0].value = format!("Health: {}/{}", caster_hp.current, caster_hp.max);
        mp_text.sections[0].value = format!("Mana: {}/{}", caster_mp.current, caster_mp.max);
        shield_text.sections[0].value = format!("Shield: {}", caster_shield.0);
        initiative_text.sections[0].value = format!("Initiative: {}", (caster_initiative.0 as f32));
        attack_text.sections[0].value = format!(
            "Attack: {}",
            (caster_attack.base as f32) * attack_multiplier / 100.
        );
        attack_spe_text.sections[0].value = format!(
            "AttackSpe: {}",
            (caster_attack_spe.base as f32) * attack_spe_multiplier / 100.
        );
        defense_text.sections[0].value = format!(
            "Defense: {}",
            (caster_defense.base as f32) * defense_multiplier / 100.
        );
        defense_spe_text.sections[0].value = format!(
            "DefenseSpe: {}",
            (caster_defense_spe.base as f32) * defense_spe_multiplier / 100.
        );
    }
}

/// # Note
///
/// DEBUG
/// XXX: A proper clone of update_caster_stats_panel but just for target instead of caster
pub fn update_target_stats_panel(
    targeted_query: Query<
        (&Name, &Hp, &Mana),
        (
            Or<(Changed<Targeted>, Changed<Hp>, Changed<Mana>)>,
            With<Targeted>,
            With<InCombat>,
        ),
    >,

    mut target_removals: RemovedComponents<Targeted>,

    mut hp_query: Query<&mut Text, (With<HpMeter>, Without<MpMeter>, With<TargetMeter>)>,
    mut mp_query: Query<&mut Text, (Without<HpMeter>, With<MpMeter>, With<TargetMeter>)>,
) {
    for (name, hp, mana) in targeted_query.iter() {
        let mut hp_text = hp_query.single_mut();
        let mut mp_text = mp_query.single_mut();

        let hp_display = format!("Target {} hp: {}", name, &hp.current.to_string());
        hp_text.sections[0].value = hp_display;

        let mp_display = format!("Target {} mp: {}", name, &mana.current.to_string());
        mp_text.sections[0].value = mp_display;
    }

    for _entity in target_removals.iter() {
        let mut hp_text = hp_query.single_mut();
        let mut mp_text = mp_query.single_mut();

        let hp_display = String::from("Target hp: ??");
        hp_text.sections[0].value = hp_display;

        let mp_display = String::from("Target mp: ??");
        mp_text.sections[0].value = mp_display;
    }
}

/* -------------------------------------------------------------------------- */
/*                               Weapon Section                               */
/* -------------------------------------------------------------------------- */

/// Update the sprite with the weapon of the Selected
pub fn update_weapon_displayer(
    asset_server: Res<AssetServer>,

    selected_query: Query<
        &Equipements,
        (
            Or<(Added<Selected>, Changed<Equipements>)>,
            With<Selected>,
            With<InCombat>,
        ),
    >,
    mut weapon_displayer_query: Query<(&mut UiImage, &mut Visibility), With<WeaponDisplayer>>,
    weapon_query: Query<&Equipement, With<WeaponType>>,
) {
    if let Ok(Equipements { weapon, armor: _ }) = selected_query.get_single() {
        let (mut weapon_image, mut visibility) = weapon_displayer_query.single_mut();

        match weapon {
            None => *visibility = Visibility::Hidden,
            Some(weapon_entity) => {
                *visibility = Visibility::Inherited;
                let Equipement {
                    owner: _,
                    icon_path,
                } = weapon_query.get(*weapon_entity).unwrap();

                weapon_image.texture = asset_server.load(icon_path)
            }
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                                 Skill Menu                                 */
/* -------------------------------------------------------------------------- */

/// Determine the visibility of the 6 skills
///
/// Update these values when the entity selected changed
///
/// # Note
///
/// REFACTOR: Maybe find some new ways to sequence these affectations better
pub fn skill_visibility(
    mut selection_removal_query: RemovedComponents<Selected>,
    caster_query: Query<
        (&Equipements, &Skills, &Job),
        (With<Selected>, With<InCombat>, Added<Selected>),
    >,
    weapon_query: Query<(&WeaponType, &SkillTiers), With<Equipement>>,

    jobs_masteries_resource: Res<JobsMasteries>,

    mut skill_bar_query: Query<(
        Entity,
        &SkillDisplayer,
        &SkillBar,
        &mut Skill,
        &mut Visibility,
        &Children,
    )>,
    mut text_query: Query<&mut Text>,
) {
    // If there was a transition, a changement in the one being Selected
    // ------ Reset all Skill ------
    for _ in selection_removal_query.iter() {
        for (_, _, _, mut skill, mut visibility, children) in skill_bar_query.iter_mut() {
            // --- Text ---
            let mut text = text_query.get_mut(children[0]).unwrap();
            text.sections[0].value = "Pass".to_string();
            *skill = Skill::pass();

            // --- Visibility ---
            // let old_visibility = visibility.clone();
            *visibility = Visibility::Hidden;

            // // --- Logs ---
            // if old_visibility != *visibility {
            //     // DEBUG: Skills' Visibility switcher
            //     info!(
            //         "{:?} °{} visibility switch: {:?}",
            //         skill_bar_type, skill_number.0, *visibility
            //     );
            // }
        }
    }

    // ------ Set the visibility w.r.t. the newly selected caster ------
    if let Ok((Equipements { weapon, armor: _ }, skills, job)) = caster_query.get_single() {
        // OPTIMIZE: Iterate over all skilldisplayer one time and for each non base_skill_displayer get the weapon_skills?
        // ----- Base Skill Bar -----
        for (_, skill_number, skill_bar_type, mut skill, mut visibility, children) in
            skill_bar_query.iter_mut()
        {
            if SkillBar::Base == *skill_bar_type {
                // let old_visibility = visibility.clone();
                // --- Text ---
                let mut text = text_query.get_mut(children[0]).unwrap();

                if skill_number.0 < skills.len() {
                    // --- Visibility ---
                    *visibility = Visibility::Inherited;

                    text.sections[0].value = skills[skill_number.0].clone().name;
                    *skill = skills[skill_number.0].clone();
                } else {
                    // --- Visibility ---
                    *visibility = Visibility::Hidden;

                    // vv-- "useless" --vv
                    text.sections[0].value = "Pass".to_string();
                    *skill = Skill::pass();
                };

                // // --- Logs ---
                // if old_visibility != *visibility {
                //     // DEBUG: Skills' Visibility switcher
                //     info!(
                //         "{:?} °{} visibility switch: {:?}",
                //         *skill_bar_type, skill_number.0, *visibility
                //     );
                // }
            }
        }

        match weapon {
            None => {
                info!("No weapon on the entity")
            }
            Some(weapon_entity) => {
                if let Ok((
                    weapon_type,
                    SkillTiers {
                        tier_2,
                        tier_1,
                        tier_0,
                    },
                )) = weapon_query.get(*weapon_entity)
                {
                    let mastery_tier: Option<&MasteryTier> =
                        jobs_masteries_resource.get(&(*job, *weapon_type));

                    info!(
                        "Job {:?} is {:?} with {:?}",
                        *job, mastery_tier, *weapon_type
                    );

                    for (
                        _skill_displayer_entity,
                        skill_number,
                        skill_bar_type,
                        mut skill,
                        mut visibility,
                        children,
                    ) in skill_bar_query.iter_mut()
                    {
                        if SkillBar::Base != *skill_bar_type {
                            // info!("skill displayer: {:?}", *skill_bar_type);

                            // let old_visibility = visibility.clone();
                            // --- Text ---
                            let mut text = text_query.get_mut(children[0]).unwrap();

                            // match jobs_masteries_resource.get(&(*job, *weapon_type)) {
                            //     None => warn!("There is no combinaison between {:?} and {:?}", job, weapon_type),
                            //     Some(MasteryTier::Two) => {}
                            //     Some(MasteryTier::One) => {}
                            //     Some(MasteryTier::Zero) => {}
                            // }

                            if Some(MasteryTier::Two) == mastery_tier.copied() {
                                // ----- Tier2 Skill Bar -----
                                if SkillBar::Tier2 == *skill_bar_type {
                                    if skill_number.0 < tier_2.len() {
                                        // --- Visibility ---
                                        *visibility = Visibility::Inherited;

                                        text.sections[0].value =
                                            tier_2[skill_number.0].clone().name;
                                        *skill = tier_2[skill_number.0].clone();
                                    } else {
                                        // --- Visibility ---
                                        *visibility = Visibility::Hidden;

                                        // vv-- "useless" --vv
                                        text.sections[0].value = "Pass".to_string();
                                        *skill = Skill::pass();
                                    };
                                }
                            }

                            // if and not else if cause MasteryTier::Two = all tier2 and tier1 and tier0 (resp with MasteryTier::One except tier2)
                            if Some(MasteryTier::Two) == mastery_tier.copied()
                                || Some(MasteryTier::One) == mastery_tier.copied()
                            {
                                // ----- Tier1 Skill Bar -----
                                if SkillBar::Tier1 == *skill_bar_type {
                                    if skill_number.0 < tier_1.len() {
                                        // --- Visibility ---
                                        *visibility = Visibility::Inherited;

                                        text.sections[0].value =
                                            tier_1[skill_number.0].clone().name;
                                        *skill = tier_1[skill_number.0].clone();
                                    } else {
                                        // --- Visibility ---
                                        *visibility = Visibility::Hidden;

                                        // vv-- "useless" --vv
                                        text.sections[0].value = "Pass".to_string();
                                        *skill = Skill::pass();
                                    };
                                }
                            }

                            // Two, One, Zero or None
                            // None => warn!("There is no combinaison between {:?} and {:?}", job, weapon_type),
                            // if _ == mastery_tier {
                            // ----- Tier0 Skill Bar -----
                            if SkillBar::Tier0 == *skill_bar_type {
                                if skill_number.0 < tier_0.len() {
                                    // --- Visibility ---
                                    *visibility = Visibility::Inherited;

                                    text.sections[0].value = tier_0[skill_number.0].clone().name;
                                    *skill = tier_0[skill_number.0].clone();
                                } else {
                                    // --- Visibility ---
                                    *visibility = Visibility::Hidden;

                                    // vv-- "useless" --vv
                                    text.sections[0].value = "Pass".to_string();
                                    *skill = Skill::pass();
                                };
                            }
                            // }
                            if None == mastery_tier {
                                info!("Job {:?} is not associated with {:?}", *job, *weapon_type);
                            }

                            // // --- Logs ---
                            // if old_visibility != *visibility {
                            //     // DEBUG: Skills' Visibility switcher
                            //     info!(
                            //         "{:?} °{} visibility switch: {:?}",
                            //         *skill_bar_type, skill_number.0, *visibility
                            //     );
                            // }
                        }
                    }
                }
            }
        }
    }
}

/// Updates the color of the skill,
/// whenever the Selected entity changed or their ActionCount change
pub fn skill_color(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<Interaction>, With<Button>, With<SkillDisplayer>),
    >,

    changed_selected_query: Query<
        (Entity, &Name, &ActionCount),
        (With<Selected>, Changed<ActionCount>),
    >,
) {
    if let Ok((_, _, action_count)) = changed_selected_query.get_single() {
        for (interaction, mut color) in &mut interaction_query {
            match *interaction {
                Interaction::Clicked => {
                    *color = if action_count.current == 0 {
                        INACTIVE_BUTTON.into()
                    } else {
                        PRESSED_BUTTON.into()
                    };
                }
                Interaction::Hovered => {
                    *color = if action_count.current == 0 {
                        INACTIVE_HOVERED_BUTTON.into()
                    } else {
                        HOVERED_BUTTON.into()
                    };
                }
                Interaction::None => {
                    *color = if action_count.current == 0 {
                        INACTIVE_BUTTON.into()
                    } else {
                        NORMAL_BUTTON.into()
                    };
                }
            }
        }
    }
}
