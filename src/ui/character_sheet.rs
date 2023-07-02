//! Display the Character Sheet
//! All Stats and Skills

use bevy::prelude::*;

use crate::{
    characters::{FabiensInfos, PersonalInfos},
    combat::{
        alterations::AlterationAction,
        phases::TransitionPhaseEvent,
        skills::Skill,
        stats::{Attack, AttackSpe, Defense, DefenseSpe, Hp, Initiative, Mana, Shield},
        stuff::{Equipement, Equipements, Job, JobsMasteries, MasteryTier, SkillTiers, WeaponType},
        ActionCount, CombatResources, CombatState, CurrentAlterations, InCombat, Recruted, Skills,
    },
    constants::ui::{dialogs::*, hud_wall::combat::*},
    ui::{
        combat_panel::{SkillBar, SkillDisplayer, TargetMeter},
        combat_system::{HpMeter, MpMeter, Selected, Targeted},
    },
};

use super::{
    combat_panel::{AllyCharacterSheet, CharacterSheetAssociations, Portrait, WeaponDisplayer},
    combat_system::UpdateUnitSelectedEvent,
};

/* -------------------------------------------------------------------------- */
/*                              Character's Sheet                             */
/* -------------------------------------------------------------------------- */

/// DOC: Event
pub struct FocusCharacterSheet(pub usize);

/// DOC: Event
pub struct UnFocusCharacterSheet(pub usize);

/// Marker for Character Sheet which is in foreground
///
/// One or less at the same time
#[derive(Component)]
pub struct Focused;

/// Happens when
/// - The combat start,
/// in Initialisation phase,
///
/// Read in
/// - ui:character_sheet::update_headers()
///   - update Portrait, Title, Name, Job
pub struct InitializeCharacterSheetHeaders;

/// Happens when
/// - The combat start,
/// in Initialisation phase,
///
/// Read in
/// - ui:character_sheet::skill_visibility()
///   - update the # of skill displayed and link
pub struct InitializeCharacterSheetSkills;

// /// Happens when
// /// - The combat start,
// /// in Initialisation phase,
// ///
// /// Read in
// /// - ui:character_sheet::ally_character_sheet_visibility()
// ///   - update the number of allies' Sheet displayed
// pub struct InitializeCharacterSheetVisibility;
pub fn initialisation(
    allies_query: Query<(Entity, &InCombat, &Name), With<Recruted>>,
    mut ally_character_sheet: ResMut<CharacterSheetAssociations>,

    mut initialisation_headers_event: EventWriter<InitializeCharacterSheetHeaders>,
    mut initialisation_skills_event: EventWriter<InitializeCharacterSheetSkills>,
    mut transition_phase_event: EventWriter<TransitionPhaseEvent>,
) {
    info!("Combat Initialisation");
    for (ally, id, name) in allies_query.iter() {
        info!("{} is now associated by the key: {}", name, id.0);
        ally_character_sheet.get_mut(id).unwrap().fighter = Some(ally);
    }

    initialisation_headers_event.send(InitializeCharacterSheetHeaders);
    initialisation_skills_event.send(InitializeCharacterSheetSkills);
    transition_phase_event.send(TransitionPhaseEvent(CombatState::default()));
}

/// OPTIMIZE: Restrain execution to the very start (Initialisation)
pub fn ally_character_sheet_visibility(
    combat_resources: Res<CombatResources>,
    mut ally_character_sheet: Query<(&AllyCharacterSheet, &mut Visibility)>,
) {
    // TODO: once by combat (cause the total number doesn't change after)
    if combat_resources.is_changed() {
        for (sheet_number, mut visibility) in ally_character_sheet.iter_mut() {
            *visibility = if sheet_number.0 < combat_resources.number_of_fighters.ally.total {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }
    }
}

/// TODO: Zoom in on characterSheet (or just focus)
/// TODO: create a cross button to close it with the mouse
pub fn ally_character_sheet_interact(
    unfocused_interaction_query: Query<
        (&Interaction, &AllyCharacterSheet),
        (Changed<Interaction>, Without<Focused>, Without<Button>),
    >,
    char_sheet_associations: Res<CharacterSheetAssociations>,
    mut select_event: EventWriter<UpdateUnitSelectedEvent>,
) {
    for (interaction, sheet_number) in unfocused_interaction_query.iter() {
        match interaction {
            Interaction::Clicked => {
                match char_sheet_associations.get(sheet_number) {
                    None => info!("{} is not in the hashmap", sheet_number.0),
                    Some(sheet_table) => {
                        info!("{} is in the hashmap", sheet_number.0);
                        match sheet_table.fighter {
                            None => info!("No fighter associated with {}", sheet_number.0),
                            Some(fighter) => select_event.send(UpdateUnitSelectedEvent(fighter)),
                        }
                    }
                }
                // let combat_unit: Entity = char_sheet_associations
                //     .get(sheet_number)
                //     .unwrap()
                //     .fighter
                //     .unwrap();
                // select_event.send(UpdateUnitSelectedEvent(combat_unit));
            }
            Interaction::Hovered => {
                // TODO: smooth slight zoom
            }
            Interaction::None => {}
        }
    }
}

/// TODO: EventHandle for CharSheetClick / Ally Selected
/// Zoom in on characterSheet (or just focus) or if already focused, unfocus
pub fn focus_character_sheet(
    mut focus_char_sheet_event: EventReader<FocusCharacterSheet>,

    mut commands: Commands,

    focused_character_sheet: Query<&AllyCharacterSheet, With<Focused>>,
    mut character_sheet: Query<
        (
            Entity,
            &AllyCharacterSheet,
            &mut Style,
            &mut Transform,
            &mut Visibility,
        ),
        Without<Focused>,
    >,

    mut unfocus_char_sheet_event: EventWriter<UnFocusCharacterSheet>,
) {
    for FocusCharacterSheet(number) in focus_char_sheet_event.iter() {
        if *number <= 11 {
            info!("focus: {} asked", *number);
            if let Ok(sheet_number) = focused_character_sheet.get_single() {
                if sheet_number.0 == *number {
                    continue;
                }
                // There can be a transition phase with two CS Focused
                unfocus_char_sheet_event.send(UnFocusCharacterSheet(**sheet_number));
            }
            for (sheet, sheet_number, mut sheet_style, mut sheet_transform, mut sheet_visibility) in
                character_sheet.iter_mut()
            {
                if sheet_number.0 == *number {
                    commands.entity(sheet).insert(Focused);
                    // TODO: Focused added handle
                    // TODO: Focused removaldetection handle

                    sheet_style.position = UiRect::new(
                        Val::Undefined,
                        Val::Percent(100. * (*number as f32 + 1.)),
                        Val::Undefined,
                        Val::Undefined,
                    );
                    sheet_transform.scale = Vec3::splat(1.);
                    *sheet_visibility = Visibility::Inherited;
                    break;
                }
            }
        } else {
            warn!("focus: Wrong Character's Sheet Number asked {}", *number)
        }
    }
}

/// TODO: EventHandle for Return Button (ecc)
/// Unfocus the Character's Sheet asked
pub fn unfocus_character_sheet(
    mut unfocus_char_sheet_event: EventReader<UnFocusCharacterSheet>,

    mut commands: Commands,

    mut character_sheet: Query<(
        Entity,
        &AllyCharacterSheet,
        &mut Style,
        &mut Transform,
        &mut Visibility,
    )>,
) {
    for UnFocusCharacterSheet(number) in unfocus_char_sheet_event.iter() {
        if *number <= 11 {
            info!("unfocus: {} asked", *number);
            if let Ok((
                sheet,
                sheet_number,
                mut sheet_style,
                mut sheet_transform,
                mut sheet_visibility,
            )) = character_sheet.get_single_mut()
            {
                if *number <= 5 {
                    commands.entity(sheet).remove::<Focused>();

                    let left = Val::Percent(
                        CHARACTER_SHEET_OFFSET_X
                            + CHARACTER_SHEET_WIDTH * (sheet_number.0 as f32 + 1.),
                    );
                    let bottom = if sheet_number.0 < 3 {
                        Val::Percent(CHARACTER_SHEET_FIRST_ROW_Y)
                    } else {
                        Val::Percent(CHARACTER_SHEET_SECOND_ROW_Y)
                    };
                    sheet_style.position =
                        UiRect::new(Val::Undefined, left, Val::Undefined, bottom);
                    sheet_transform.scale = Vec3::splat(0.2);
                }
                // Enemy Scrolls
                else {
                    // TODO: ShouldHave - Roll back and replace in the pack of scroll
                    *sheet_visibility = Visibility::Hidden;
                }
            }
        } else {
            warn!("unfocus: Wrong Character's Sheet Number asked {}", *number)
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                                   Headers                                  */
/* -------------------------------------------------------------------------- */

/// Update all character's sheet with the infos of each ally
/// (Name, Sprite, Title, Job).
///
/// Only run once at the start of the combat.
/// These infos won't change afterwards
/// TODO: unless the sprite? - can be deadge too
pub fn update_headers(
    mut initialisation_headers_event: EventReader<InitializeCharacterSheetHeaders>,
    asset_server: Res<AssetServer>,
    fabiens_infos: Res<FabiensInfos>,

    combat_units_query: Query<(&Job, &Name, &TextureAtlasSprite, &InCombat)>, // With<Recruted>>,
    character_sheet_associations: Res<CharacterSheetAssociations>,

    mut portrait_query: Query<&mut UiImage, With<Portrait>>,
    mut text_query: Query<&mut Text>,
) {
    for _ in initialisation_headers_event.iter() {
        // sort recruted by Recruted(usize) to keep the order straight
        for (job, name, _sprite, id) in combat_units_query.iter() {
            let character_sheet = character_sheet_associations.get(id).unwrap();

            let mut portrait = portrait_query.get_mut(character_sheet.portrait).unwrap();
            let [mut fabien_name_text, mut title_text, mut job_text] = text_query
                .get_many_mut([
                    character_sheet.name,
                    character_sheet.title,
                    character_sheet.job,
                ])
                .unwrap();

            // portrait.index = sprite.index;
            if let Some(PersonalInfos { title, sprite_path }) = fabiens_infos.get(&name.to_string())
            {
                title_text.sections[0].value = title.to_string();
                portrait.texture = asset_server.load(sprite_path);
            } else {
                warn!("{} Not Found/Associated in the FabienDataBase", name);
                title_text.sections[0].value = "Fabien".to_string();
                portrait.texture =
                    asset_server.load("textures/character/idle/idle_Fabien_Loyal.png");
            };
            job_text.sections[0].value = format!("{:?}", job);
            fabien_name_text.sections[0].value = name.replace("NPC ", "").replace("Player ", "");
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                                    Stats                                   */
/* -------------------------------------------------------------------------- */

/// # Note
///
/// TODO: Add Damage Multiplier (Suffered/Inflicted)
pub fn update_caster_stats_panel(
    ally_character_sheet: Res<CharacterSheetAssociations>,

    allies_query: Query<
        (
            &Hp,
            &Mana,
            &Shield,
            &Initiative,
            &Attack,
            &AttackSpe,
            &Defense,
            &DefenseSpe,
            &CurrentAlterations,
            // &Equipements,
            &InCombat,
        ),
        (Or<(Changed<Hp>, Changed<Mana>)>, With<Recruted>),
    >,

    mut text_query: Query<&mut Text>,
) {
    for (
        hp,
        mp,
        shield,
        initiative,
        attack,
        attack_spe,
        defense,
        defense_spe,
        alterations,
        // equipments,
        id,
    ) in allies_query.iter()
    {
        let character_sheet = ally_character_sheet.get(id).unwrap();

        let [mut hp_text, mut mp_text, mut shield_text, mut initiative_text, mut attack_text, mut attack_spe_text, mut defense_text, mut defense_spe_text] =
            text_query
                .get_many_mut([
                    character_sheet.health,
                    character_sheet.mana,
                    character_sheet.shield,
                    character_sheet.initiative,
                    character_sheet.attack,
                    character_sheet.attack_spe,
                    character_sheet.defense,
                    character_sheet.defense_spe,
                ])
                .unwrap();

        let mut attack_percentage: f32 = 100.;
        let mut attack_spe_percentage: f32 = 100.;
        let mut defense_percentage: f32 = 100.;
        let mut defense_spe_percentage: f32 = 100.;

        for alt in alterations.iter() {
            match alt.action {
                AlterationAction::StatsPercentage | AlterationAction::StatsFlat => {
                    attack_percentage += alt.attack as f32;
                    attack_spe_percentage += alt.attack_spe as f32;
                    defense_percentage += alt.defense as f32;
                    defense_spe_percentage += alt.defense_spe as f32;
                }
                _ => {}
            }
        }

        hp_text.sections[0].value = format!("Health: {}/{}", hp.current, hp.max);
        mp_text.sections[0].value = format!("Mana: {}/{}", mp.current, mp.max);
        shield_text.sections[0].value = format!("Shield: {}", shield.0);
        initiative_text.sections[0].value = format!("Initiative: {}", (initiative.0 as f32));
        attack_text.sections[0].value = format!(
            "Attack: {}",
            (attack.base as f32) * attack_percentage / 100.
        );
        attack_spe_text.sections[0].value = format!(
            "AttackSpe: {}",
            (attack_spe.base as f32) * attack_spe_percentage / 100.
        );
        defense_text.sections[0].value = format!(
            "Defense: {}",
            (defense.base as f32) * defense_percentage / 100.
        );
        defense_spe_text.sections[0].value = format!(
            "DefenseSpe: {}",
            (defense_spe.base as f32) * defense_spe_percentage / 100.
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

/// Update the sprite with the weapon of the Selected,
/// at each equipement change.
///
/// REFACTOR: ? - Only run once at the start of the combat. These infos won't change afterwards (Demo)
pub fn update_weapon_displayer(
    asset_server: Res<AssetServer>,

    allies_query: Query<(&Equipements, &InCombat), (Changed<Equipements>, With<Recruted>)>,
    ally_character_sheet: Res<CharacterSheetAssociations>,

    mut weapon_displayer_query: Query<(&mut UiImage, &mut Visibility), With<WeaponDisplayer>>,
    weapon_query: Query<&Equipement, With<WeaponType>>,
) {
    for (Equipements { weapon, armor: _ }, id) in allies_query.iter() {
        let character_sheet = ally_character_sheet.get(id).unwrap();
        let (mut weapon_image, mut visibility) = weapon_displayer_query
            .get_mut(character_sheet.weapon)
            .unwrap();

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
/// OPTIMIZE: Trigger Only one time
pub fn skill_visibility(
    mut initialisation_skills_event: EventReader<InitializeCharacterSheetSkills>,

    allies_query: Query<(&Equipements, &Skills, &Job, &InCombat), With<Recruted>>,
    ally_character_sheet: Res<CharacterSheetAssociations>,

    weapon_query: Query<(&WeaponType, &SkillTiers), With<Equipement>>,

    jobs_masteries_resource: Res<JobsMasteries>,

    skill_menu: Query<&Children>,
    mut skill_bar_query: Query<
        (&SkillDisplayer, &mut Skill, &mut Visibility, &Children),
        With<SkillBar>,
    >,
    mut text_query: Query<&mut Text>,
) {
    for _ in initialisation_skills_event.iter() {
        // ------ Reset all Skill ------
        for (_, mut skill, mut visibility, children) in skill_bar_query.iter_mut() {
            // --- Text ---
            let mut text = text_query.get_mut(children[0]).unwrap();
            text.sections[0].value = "Pass".to_string();
            *skill = Skill::pass();

            // --- Visibility ---
            *visibility = Visibility::Hidden;
        }

        // ------ Set the visibility w.r.t. the newly selected caster ------
        for (Equipements { weapon, armor: _ }, skills, job, id) in allies_query.iter() {
            let character_sheet = ally_character_sheet.get(id).unwrap();
            let base_skills = skill_menu.get(character_sheet.base_skills).unwrap();

            /* -------------------------------------------------------------------------- */
            /*                               Base Skill Bar                               */
            /* -------------------------------------------------------------------------- */

            for entity in base_skills.iter() {
                let (skill_number, mut skill, mut visibility, children) =
                    skill_bar_query.get_mut(*entity).unwrap();

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
                }
            }

            /* -------------------------------------------------------------------------- */
            /*                              Weapon Skill Bar                              */
            /* -------------------------------------------------------------------------- */

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
                        let tier_2_skills = skill_menu.get(character_sheet.tier_2_skills).unwrap();
                        let tier_1_skills = skill_menu.get(character_sheet.tier_1_skills).unwrap();
                        let tier_0_skills = skill_menu.get(character_sheet.tier_0_skills).unwrap();

                        let mastery_tier: Option<&MasteryTier> =
                            jobs_masteries_resource.get(&(*job, *weapon_type));

                        info!(
                            "Job {:?} is {:?} with {:?}",
                            *job, mastery_tier, *weapon_type
                        );

                        if None == mastery_tier {
                            info!("Job {:?} is not associated with {:?}", *job, *weapon_type);
                        }

                        if Some(MasteryTier::Two) == mastery_tier.copied() {
                            for tier_2_displayer in tier_2_skills.iter() {
                                let (skill_number, mut skill, mut visibility, children) =
                                    skill_bar_query.get_mut(*tier_2_displayer).unwrap();
                                // --- Text ---
                                let mut text = text_query.get_mut(children[0]).unwrap();

                                if skill_number.0 < tier_2.len() {
                                    // --- Visibility ---
                                    *visibility = Visibility::Inherited;

                                    text.sections[0].value = tier_2[skill_number.0].clone().name;
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

                        if Some(MasteryTier::Two) == mastery_tier.copied()
                            || Some(MasteryTier::One) == mastery_tier.copied()
                        {
                            for tier_1_displayer in tier_1_skills.iter() {
                                let (skill_number, mut skill, mut visibility, children) =
                                    skill_bar_query.get_mut(*tier_1_displayer).unwrap();
                                // --- Text ---
                                let mut text = text_query.get_mut(children[0]).unwrap();

                                if skill_number.0 < tier_1.len() {
                                    // --- Visibility ---
                                    *visibility = Visibility::Inherited;

                                    text.sections[0].value = tier_1[skill_number.0].clone().name;
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
                        for tier_0_displayer in tier_0_skills.iter() {
                            let (skill_number, mut skill, mut visibility, children) =
                                skill_bar_query.get_mut(*tier_0_displayer).unwrap();
                            // --- Text ---
                            let mut text = text_query.get_mut(children[0]).unwrap();

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
        (With<Selected>, Or<(Added<Selected>, Changed<ActionCount>)>),
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
