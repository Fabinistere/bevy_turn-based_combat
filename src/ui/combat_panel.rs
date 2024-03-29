use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    prelude::*,
};

use crate::{
    combat::{
        skills::Skill,
        stats::{Attack, AttackSpe, Defense, DefenseSpe, Hp, Initiative, Mana, Shield},
        stuff::Job,
        CombatResources,
    },
    constants::{
        combat::FIRST_ENEMY_ID,
        ui::{dialogs::*, style::*, *},
    },
    ui::{
        combat_system::{HpMeter, MpMeter},
        player_interaction::{EndOfTurnButton, ScrollingList},
    },
};

use super::log_cave::CombatLogResources;

/* -------------------------------------------------------------------------- */
/*                                UI Resources                                */
/* -------------------------------------------------------------------------- */

/// DOC : new name ? CombatWallAssetsResources
#[derive(Resource)]
pub struct CombatWallResources {
    pub base_combat_wall: Handle<Image>,
    pub pack_of_scroll: Handle<Image>,
    pub weapons: Handle<Image>,
    pub allies_scroll: Vec<Handle<Image>>,
}

impl FromWorld for CombatWallResources {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let mut allies_scroll = Vec::new();
        for i in 0..6 {
            allies_scroll.push(asset_server.load(format!(
                "textures/UI/HUD/combat/wall/sheets/sheet_{}.png",
                i
            )));
        }

        CombatWallResources {
            base_combat_wall: asset_server.load("textures/UI/HUD/combat/wall/base_combat_wall.png"),
            pack_of_scroll: asset_server.load("textures/UI/HUD/combat/wall/scrolls_pack.png"),
            weapons: asset_server.load("textures/UI/HUD/combat/wall/stuffs.png"),
            allies_scroll,
        }
    }
}

/// DOC : new name ? CharacterSheetAssetsResources
#[derive(Resource)]
pub struct CharacterSheetAssetsResources {
    // pub base_scroll: Handle<Image>,
    // pub base_headers: Handle<Image>,
    /// DOC: rename to base_scroll
    pub base_full_scroll: Handle<Image>,
    pub top_left_corner: Handle<Image>,
    pub weapon_frame: Handle<Image>,
}

impl FromWorld for CharacterSheetAssetsResources {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        CharacterSheetAssetsResources {
            // base_scroll: asset_server.load("textures/UI/HUD/combat/character_sheet/base_scroll.png"),
            // base_headers: asset_server.load("textures/UI/HUD/combat/character_sheet/base_headers.png"),
            base_full_scroll: asset_server
                .load("textures/UI/HUD/combat/character_sheet/base_full_scroll.png"),
            top_left_corner: asset_server
                .load("textures/UI/HUD/combat/character_sheet/top_left_corner.png"),
            weapon_frame: asset_server.load("textures/UI/border/border_05_nobackground.png"),
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                                UI Components                               */
/* -------------------------------------------------------------------------- */

#[derive(Component)]
pub struct UIScene;

#[derive(Component)]
pub struct Ladder;

#[derive(Component)]
pub struct HUDWall;

/// XXX: Useless component used to differentiate Hp/MpMeters of a target or a caster
#[derive(Component)]
pub struct TargetMeter;

#[derive(Component)]
pub struct CombatStateDisplayer;

#[derive(Default, Component, Reflect, Deref, DerefMut)]
pub struct ActionDisplayer(pub usize);

/* -------------------------------------------------------------------------- */
/*                               Character Sheet                              */
/* -------------------------------------------------------------------------- */

#[derive(Component)]
pub struct CharacterSheet;

/// REFACTOR: Is the id still needed ? Associate entities at the Combat Initiation - no, we want to get from the CSh and from the fighter.
///
/// Still image showing a mini character sheet. (6 of allies and the pack of scrolls)
/// Contains its id
#[derive(Default, Component, Reflect, Deref, DerefMut)]
pub struct MiniCharacterSheet(pub usize);

#[derive(Default, Component, Reflect, Deref, DerefMut)]
pub struct SkillDisplayer(pub usize);

#[derive(Component)]
pub struct WeaponDisplayer;

/// REFACTOR: SkillBar Structure
#[derive(Component, Reflect, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum SkillBar {
    Base,
    Tier2,
    Tier1,
    Tier0,
    /// TODO: ShouldHave - Job's Skills
    Job,
    /// TODO: PostDemo - Unlock by the Job XpTree
    Extra,
}

#[derive(Component)]
pub struct Portrait;

#[derive(Component)]
pub struct FabienName;

#[derive(Component)]
pub struct Title;

/// # Note
///
/// I have to insert the resource
/// (init from_world is a bad idea: it's stupid to have a character sheet outside of the hud combat)
/// and to avoid abstract one level up with `CharacterSheetResource(Option<CharacterSheetElements>)`
/// (but in a case of a HashMap association to implement minis charsheets, it's mandatory to abstract)
/// I choose to put all field to `Option<Entity>`.
#[derive(Resource, Default, PartialEq, Eq, Hash, Clone, Debug, Reflect)]
pub struct CharacterSheetElements {
    pub character_sheet: Option<Entity>,

    pub portrait: Option<Entity>,
    pub name: Option<Entity>,
    pub title: Option<Entity>,
    pub job: Option<Entity>,
    pub weapon: Option<Entity>,
    pub health: Option<Entity>,
    pub mana: Option<Entity>,
    pub shield: Option<Entity>,
    pub initiative: Option<Entity>,
    pub attack: Option<Entity>,
    pub attack_spe: Option<Entity>,
    pub defense: Option<Entity>,
    pub defense_spe: Option<Entity>,
    pub base_skills: Option<Entity>,
    pub tier_2_skills: Option<Entity>,
    pub tier_1_skills: Option<Entity>,
    pub tier_0_skills: Option<Entity>,
}

/* -------------------------------------------------------------------------- */
/*                                 UI CleanUp                                 */
/* -------------------------------------------------------------------------- */

/// The Fighting Hall and Initiative Bar are preserved
pub fn cleanup(
    mut commands: Commands,
    character_sheet_query: Query<Entity, With<CharacterSheet>>,
    hud_wall_query: Query<Entity, With<HUDWall>>,
) {
    let character_sheet = character_sheet_query.single();
    let hud_wall = hud_wall_query.single();

    commands.entity(character_sheet).despawn_recursive();
    commands.entity(hud_wall).despawn_recursive();
}

/* -------------------------------------------------------------------------- */
/*                                  UI Setup                                  */
/* -------------------------------------------------------------------------- */

pub fn hud_wall_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,

    combat_resources: Res<CombatResources>,

    character_sheet_resources: Res<CharacterSheetAssetsResources>,
    combat_wall_resources: Res<CombatWallResources>,
    combat_log_resources: Res<CombatLogResources>,

    mut character_sheet_elements: ResMut<CharacterSheetElements>,

    ui_scene_query: Query<Entity, With<UIScene>>,
) {
    // TODO: add UILocation::CharacterSheet(Ally | Enemy) | Logs | CombatHUB | etc
    // TODO: Hide Panels on the right side (and add anim)
    /* -------------------------------------------------------------------------- */
    /*                                 UI Elements                                */
    /* -------------------------------------------------------------------------- */

    // TODO: Upgrade when Available - use Spritesheet
    // TODO: Apply a mask to conceal the legs of the portrait (or simply change asset but meh.)
    // Or put the portrait between the baseScroll and the base Header (but doesn't work)
    let portrait = commands
        .spawn((ImageBundle::default(), Name::new("Portrait"), Portrait))
        .id();

    let name = commands
        .spawn((
            TextBundle::from_section(format!("Name"), get_text_style(&asset_server, 40.))
                .with_style(TEXT_STYLE),
            Label,
            Name::new("Name"),
            FabienName,
        ))
        .id();

    let title = commands
        .spawn((
            TextBundle::from_section(format!("Fabien"), get_text_style(&asset_server, 20.))
                .with_style(TEXT_STYLE),
            Label,
            Name::new("Title"),
            Title,
        ))
        .id();

    let job = commands
        .spawn((
            TextBundle::from_section(format!("Chill"), get_text_style(&asset_server, 20.))
                .with_style(TEXT_STYLE),
            Label,
            Name::new("Job"),
            Job::default(),
        ))
        .id();

    let health = commands
        .spawn((
            TextBundle::from_section(
                format!("Health: ???/???"),
                get_text_style(&asset_server, 20.),
            )
            .with_style(TEXT_STYLE),
            Label,
            Name::new("Health"),
            Hp::default(),
        ))
        .id();

    let mana = commands
        .spawn((
            TextBundle::from_section(format!("Mana: ???/???"), get_text_style(&asset_server, 20.))
                .with_style(TEXT_STYLE),
            Label,
            Name::new("Mana"),
            Mana::default(),
        ))
        .id();

    let shield = commands
        .spawn((
            TextBundle::from_section(format!("Shield: ???"), get_text_style(&asset_server, 20.))
                .with_style(TEXT_STYLE),
            Label,
            Name::new("Shield"),
            Shield::default(),
        ))
        .id();

    let initiative = commands
        .spawn((
            TextBundle::from_section(
                format!("Initiative: ???"),
                get_text_style(&asset_server, 20.),
            )
            .with_style(TEXT_STYLE),
            Label,
            Name::new("Initiative"),
            Initiative::default(),
        ))
        .id();

    let attack = commands
        .spawn((
            TextBundle::from_section(format!("Attack: ???"), get_text_style(&asset_server, 20.))
                .with_style(TEXT_STYLE),
            Label,
            Name::new("Attack"),
            Attack::default(),
        ))
        .id();

    let attack_spe = commands
        .spawn((
            TextBundle::from_section(
                format!("AttackSpe: ???"),
                get_text_style(&asset_server, 20.),
            )
            .with_style(TEXT_STYLE),
            Label,
            Name::new("AttackSpe"),
            AttackSpe::default(),
        ))
        .id();

    let defense = commands
        .spawn((
            TextBundle::from_section(format!("Defense: ???"), get_text_style(&asset_server, 20.))
                .with_style(TEXT_STYLE),
            Label,
            Name::new("Defense"),
            Defense::default(),
        ))
        .id();

    let defense_spe = commands
        .spawn((
            TextBundle::from_section(
                format!("DefenseSpe: ???"),
                get_text_style(&asset_server, 20.),
            )
            .with_style(TEXT_STYLE),
            Label,
            Name::new("DefenseSpe"),
            DefenseSpe::default(),
        ))
        .id();

    let weapon = commands
        .spawn((
            ImageBundle {
                style: Style {
                    width: Val::Px(50.),
                    height: Val::Px(50.),
                    align_self: AlignSelf::Center,
                    ..default()
                },
                visibility: Visibility::Hidden,
                ..default()
            },
            Name::new("Weapon"),
            WeaponDisplayer,
        ))
        .id();

    let base_skills = commands
        .spawn((
            NodeBundle {
                style: Style {
                    // height: Val::Percent(42.),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            },
            Name::new("Base Skills"),
        ))
        .with_children(|parent| {
            // 6 Base skill max

            for skill_count in 0..6 {
                parent
                    .spawn((
                        ButtonBundle {
                            style: SKILL_BUTTON_STYLE,
                            background_color: NORMAL_BUTTON.into(),
                            visibility: Visibility::Hidden,
                            ..default()
                        },
                        Name::new(format!("Skill {}", skill_count)),
                        Skill::pass(),
                        // --- UI ---
                        SkillDisplayer(skill_count),
                        SkillBar::Base,
                        // Draggable,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            format!("Skill {}", skill_count),
                            get_text_style(&asset_server, 20.),
                        ));
                    });
            }
        })
        .id();

    let tier_2_skills = commands
        .spawn((
            NodeBundle {
                style: Style {
                    // height: Val::Percent(42.),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            },
            Name::new("Tier2 Skills"),
        ))
        .with_children(|parent| {
            // 3 Tier2 skill max

            for skill_count in 0..3 {
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(150.0),
                                height: Val::Px(65.0),
                                // center button
                                margin: UiRect::all(Val::Auto),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: NORMAL_BUTTON.into(),
                            visibility: Visibility::Hidden,
                            ..default()
                        },
                        Name::new(format!("Skill {}", skill_count)),
                        Skill::pass(),
                        // --- UI ---
                        SkillDisplayer(skill_count),
                        SkillBar::Tier2,
                        // Draggable,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            format!("Skill {}", skill_count),
                            get_text_style(&asset_server, 20.),
                        ));
                    });
            }
        })
        .id();

    let tier_1_skills = commands
        .spawn((
            NodeBundle {
                style: Style {
                    // height: Val::Percent(42.),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            },
            Name::new("Tier1 Skills"),
        ))
        .with_children(|parent| {
            // 3 Tier1 skill max

            for skill_count in 0..3 {
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(150.0),
                                height: Val::Px(65.0),
                                // center button
                                margin: UiRect::all(Val::Auto),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: NORMAL_BUTTON.into(),
                            visibility: Visibility::Hidden,
                            ..default()
                        },
                        Name::new(format!("Skill {}", skill_count)),
                        Skill::pass(),
                        // --- UI ---
                        SkillDisplayer(skill_count),
                        SkillBar::Tier1,
                        // Draggable,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            format!("Skill {}", skill_count),
                            get_text_style(&asset_server, 20.),
                        ));
                    });
            }
        })
        .id();

    let tier_0_skills = commands
        .spawn((
            NodeBundle {
                style: Style {
                    // height: Val::Percent(42.),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            },
            Name::new("Tier0 Skills"),
        ))
        .with_children(|parent| {
            // 3 Tier0 skill max

            for skill_count in 0..3 {
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(150.0),
                                height: Val::Px(65.0),
                                // center button
                                margin: UiRect::all(Val::Auto),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: NORMAL_BUTTON.into(),
                            visibility: Visibility::Hidden,
                            ..default()
                        },
                        Name::new(format!("Skill {}", skill_count)),
                        Skill::pass(),
                        // --- UI ---
                        SkillDisplayer(skill_count),
                        SkillBar::Tier0,
                        // Draggable,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            format!("Skill {}", skill_count),
                            get_text_style(&asset_server, 20.),
                        ));
                    });
            }
        })
        .id();

    /* -------------------------------------------------------------------------- */
    /*                              Character' Sheet                              */
    /* -------------------------------------------------------------------------- */

    let character_sheet = commands
        .spawn((
            NodeBundle {
                // image: character_sheet_resources.base_full_scroll.clone().into(),
                style: Style {
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    flex_shrink: 0.,
                    bottom: Val::Percent(100.),
                    ..default()
                },
                visibility: Visibility::Hidden,
                ..default()
            },
            Name::new("Character Sheet"),
            Interaction::default(),
            CharacterSheet,
        ))
        .with_children(|parent| {
            // Headers
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            height: Val::Percent(20.),
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        background_color: Color::DARK_GRAY.into(),
                        ..default()
                    },
                    Name::new("Headers"),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(30.),
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                background_color: Color::CRIMSON.into(),
                                ..default()
                            },
                            Name::new("Sprite Border"),
                        ))
                        .push_children(&[portrait]);

                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(70.),
                                    flex_direction: FlexDirection::Column,
                                    ..default()
                                },
                                background_color: Color::GRAY.into(),
                                ..default()
                            },
                            Name::new("Titles"),
                        ))
                        .with_children(|parent| {
                            // TODO: Update Titles
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            height: Val::Percent(50.),
                                            flex_direction: FlexDirection::Row,
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    Name::new("Full Name"),
                                ))
                                .push_children(&[name, title]);

                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            height: Val::Percent(50.),
                                            flex_direction: FlexDirection::Row,
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    Name::new("Job Section"),
                                ))
                                .push_children(&[job]);
                        });
                });

            // Stats + weapon
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            height: Val::Percent(40.),
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        background_color: Color::CRIMSON.into(),
                        ..default()
                    },
                    Name::new("Scanner"),
                ))
                .with_children(|parent| {
                    // TODO: Update Stats and Weapon equiped
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(60.),
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                background_color: Color::GRAY.into(),
                                ..default()
                            },
                            Name::new("Stats"),
                        ))
                        .push_children(&[
                            health,
                            mana,
                            shield,
                            initiative,
                            attack,
                            attack_spe,
                            defense,
                            defense_spe,
                        ]);

                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(40.),
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                // background_color: Color::BISQUE.into(),
                                ..default()
                            },
                            Name::new("Equipements Section"),
                        ))
                        .with_children(|parent| {
                            // TODO: add frame underneath
                            parent
                                .spawn((
                                    ImageBundle {
                                        image: character_sheet_resources
                                            .weapon_frame
                                            .clone()
                                            .into(),
                                        style: Style {
                                            width: Val::Px(100.),
                                            height: Val::Px(100.),
                                            align_self: AlignSelf::Center,
                                            justify_content: JustifyContent::Center,
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    Name::new("Frame"),
                                ))
                                .push_children(&[weapon]);
                        });
                });

            // Skill Menu
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            height: Val::Percent(40.),
                            flex_direction: FlexDirection::Column,
                            // align_content: AlignContent::SpaceAround,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        background_color: Color::AZURE.into(),
                        ..default()
                    },
                    Name::new("Skill Menu"),
                ))
                // A catalogue, one row for basic skill, a row for tier2 ,etc (simplify a lot skill_visibility)
                .push_children(&[base_skills, tier_2_skills, tier_1_skills, tier_0_skills]);

            // parent
            //     .spawn((
            //         NodeBundle {
            //             style: Style {
            //                 flex_direction: FlexDirection::Column,
            //                 // flex_wrap: FlexWrap::NoWrap,
            //                 height: Val::Percent(100.),
            //                 width: Val::Percent(100.),
            //                 ..default()
            //             },
            //             ..default()
            //         },
            //         Name::new("Content"),
            //     ))
            //     .with_children(|parent| {

            //     });

            // TODO: Render Top Decoration
            // parent
            //     .spawn((NodeBundle::default(), Name::new("Top Decoration")))
            //     .with_children(|parent| {
            //         // Top Decoration
            //         parent.spawn((
            //             ImageBundle {
            //                 image: character_sheet_resources
            //                     .top_left_corner
            //                     .clone()
            //                     .into(),
            //                 style: Style {
            //                     width: Val::Px(500.),
            //                     height: Val::Percent(100.),
            //                     ..default()
            //                 },
            //                 ..default()
            //             },
            //             Name::new("Decoration - Top Left Corner"),
            //         ));
            //     });
        })
        .id();

    *character_sheet_elements = CharacterSheetElements {
        character_sheet: Some(character_sheet),
        portrait: Some(portrait),
        name: Some(name),
        title: Some(title),
        job: Some(job),
        weapon: Some(weapon),
        health: Some(health),
        mana: Some(mana),
        shield: Some(shield),
        initiative: Some(initiative),
        attack: Some(attack),
        attack_spe: Some(attack_spe),
        defense: Some(defense),
        defense_spe: Some(defense_spe),
        base_skills: Some(base_skills),
        tier_2_skills: Some(tier_2_skills),
        tier_1_skills: Some(tier_1_skills),
        tier_0_skills: Some(tier_0_skills),
    };

    let ui_scene = ui_scene_query.single();

    commands.entity(ui_scene).with_children(|parent| {
        /* -------------------------------------------------------------------------- */
        /*                                  HUD Wall                                  */
        /* -------------------------------------------------------------------------- */
        parent
            .spawn((
                ImageBundle {
                    image: combat_wall_resources.base_combat_wall.clone().into(),
                    style: Style {
                        width: Val::Percent(HUD_WALL_WIDTH),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                },
                Name::new("HUD Wall"),
                HUDWall,
            ))
            .with_children(|parent| {
                /* First Side of the HUD Wall
                 * - Each Allied CharacterSheet (SubPanels) (Fixed Image)
                 *   - First Sub-Panel
                 *     - Headers: Sprite, Name, Title, Job
                 *     - Stats, Weapon Equiped
                 *     - Skill Menu²
                 * - TODO: "Bestiary" (Book of Enemy's characterSheet)
                 * - TODO: Logs
                 * - TODO: Team's Inventory
                 * - IDEA: If we block access to a certain number of members - Show empty sheets (with no text) to represent free space
                 */

                parent
                    .spawn((
                        NodeBundle {
                            // background_color: Color::DARK_GRAY.into(),
                            style: Style {
                                flex_shrink: 0.,
                                flex_direction: FlexDirection::Column,
                                height: Val::Percent(100.),

                                ..default()
                            },
                            ..default()
                        },
                        Name::new("Interactive items"),
                    ))
                    .with_children(|parent| {
                        // REFACTOR: put the custom size directly on the sprite (gap yes but no pos on the "root")
                        parent
                            .spawn((
                                NodeBundle {
                                    // background_color: Color::GRAY.into(),
                                    style: ALLIES_SHEET_STYLE,
                                    ..default()
                                },
                                Name::new("Allies' Scroll"),
                            ))
                            .with_children(|parent| {
                                parent
                                    .spawn((
                                        NodeBundle {
                                            style: ROW_SHEETS_STYLE,
                                            ..default()
                                        },
                                        Name::new("First Row of Scrolls"),
                                    ))
                                    .with_children(|parent| {
                                        for i in 0..3 {
                                            parent.spawn((
                                                ImageBundle {
                                                    image: combat_wall_resources.allies_scroll[i]
                                                        .clone()
                                                        .into(),
                                                    visibility: if i < combat_resources
                                                        .number_of_fighters
                                                        .ally
                                                        .total
                                                    {
                                                        Visibility::Inherited
                                                    } else {
                                                        Visibility::Hidden
                                                    },
                                                    style: MINI_CHARACTER_SHEET_STYLE,
                                                    ..default()
                                                },
                                                Name::new(format!("Ally's Scroll {}", i)),
                                                Interaction::default(),
                                                MiniCharacterSheet(i),
                                            ));
                                        }
                                    });

                                parent
                                    .spawn((
                                        NodeBundle {
                                            style: ROW_SHEETS_STYLE,
                                            ..default()
                                        },
                                        Name::new("Second Row of Scrolls"),
                                    ))
                                    .with_children(|parent| {
                                        for i in 3..6 {
                                            parent.spawn((
                                                ImageBundle {
                                                    image: combat_wall_resources.allies_scroll[i]
                                                        .clone()
                                                        .into(),
                                                    visibility: if i < combat_resources
                                                        .number_of_fighters
                                                        .ally
                                                        .total
                                                    {
                                                        Visibility::Inherited
                                                    } else {
                                                        Visibility::Hidden
                                                    },
                                                    style: MINI_CHARACTER_SHEET_STYLE,
                                                    ..default()
                                                },
                                                Name::new(format!("Ally's Scroll {}", i)),
                                                Interaction::default(),
                                                MiniCharacterSheet(i),
                                            ));
                                        }
                                    });
                            });

                        parent.spawn((
                            ImageBundle {
                                image: combat_wall_resources.pack_of_scroll.clone().into(),
                                style: Style {
                                    flex_shrink: 0.,
                                    width: Val::Percent(17.),
                                    left: Val::Percent(54.),
                                    top: Val::Percent(26.6),
                                    ..default()
                                },
                                ..default()
                            },
                            // DOC: Pack of scrolls = "Bestiary"
                            Name::new("Scrolls Pack"),
                            Interaction::default(),
                            // points to the first enemy
                            MiniCharacterSheet(FIRST_ENEMY_ID),
                        ));

                        // TODO: Hide it behind the altar
                        parent.spawn((
                            ImageBundle {
                                image: combat_log_resources.ladder.clone().into(),
                                style: Style {
                                    flex_shrink: 0.,
                                    width: Val::Percent(32.),
                                    left: Val::Percent(9.7),
                                    top: Val::Percent(20.5),
                                    ..default()
                                },
                                ..default()
                            },
                            Name::new("Downwards Ladder"),
                            Interaction::default(),
                            Ladder,
                        ));

                        // parent
                        //     .spawn((
                        //         NodeBundle {
                        //             background_color: Color::DARK_GRAY.into(),
                        //             style: Style {
                        //                 flex_shrink: 0.,
                        //                 flex_direction: FlexDirection::Row,
                        //                 //  height: Val::Percent(55.2),
                        //                 ..default()
                        //             },
                        //             ..default()
                        //         },
                        //         Name::new("Lower Screen"),
                        //     ))
                        //     .with_children(|parent| {

                        //         // parent
                        //         //     .spawn((
                        //         //         NodeBundle {
                        //         //             style: Style {
                        //         //                 width: Val::Percent(50.),
                        //         //                 ..default()
                        //         //             },
                        //         //             ..default()
                        //         //         },
                        //         //         Name::new("Lower Left Screen"),
                        //         //     ))
                        //         //     .with_children(|parent| {
                        //         //     });
                        //         // parent
                        //         //     .spawn((
                        //         //         NodeBundle {
                        //         //             style: Style {
                        //         //                  width: Val::Percent(50.),
                        //         //                 ..default()
                        //         //             },
                        //         //             ..default()
                        //         //         },
                        //         //         Name::new("Lower Right Screen"),
                        //         //     ))
                        //         //     .with_children(|parent| {
                        //         //     });
                        //     });
                    });

                // TODO: Spawn the ladder at the left box of the pack of scroll
            })
            .push_children(&[character_sheet]);
    });
}

/// REFACTOR: Upgrade UiImage to spritesheet UI when [Available](https://github.com/bevyengine/bevy/pull/5070)
pub fn global_ui_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    /* -------------------------------------------------------------------------- */
    /*                                  UI Scene                                  */
    /* -------------------------------------------------------------------------- */
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            },
            Name::new("UI Scene"),
            UIScene,
        ))
        .with_children(|parent| {
            /* -------------------------------------------------------------------------- */
            /*                                Fighting Hall                               */
            /*                             Where the npcs are                             */
            /* -------------------------------------------------------------------------- */
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(FIGHTING_HALL_WIDTH),
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        background_color: Color::rgba(0., 0., 0., 0.).into(),
                        ..default()
                    },
                    Name::new("Fighting Hall"),
                ))
                .with_children(|parent| {
                    // END OF YOUR TURN
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(200.0),
                                    height: Val::Px(65.0),
                                    margin: UiRect::all(Val::Auto),
                                    top: Val::Percent(5.),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            Name::new("EndTurn Button"),
                            EndOfTurnButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "End of Turn",
                                get_text_style(&asset_server, 40.),
                            ));
                        });

                    // Stats - Caster / Target
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    top: Val::Percent(5.),
                                    flex_direction: FlexDirection::Column,
                                    flex_grow: 1.0,
                                    ..default()
                                },
                                ..default()
                            },
                            Name::new("Stats"),
                        ))
                        .with_children(|parent| {
                            // List items

                            // ----- DEBUG: Basic Stats -----
                            parent.spawn((
                                TextBundle::from_section(
                                    format!("Target hp: ???"),
                                    get_text_style(&asset_server, 20.),
                                )
                                .with_style(TEXT_STYLE),
                                Label,
                                HpMeter,
                                TargetMeter,
                                Name::new("Target Hp"),
                            ));

                            parent.spawn((
                                TextBundle::from_section(
                                    format!("Target mp: ???"),
                                    get_text_style(&asset_server, 20.),
                                )
                                .with_style(TEXT_STYLE),
                                Label,
                                MpMeter,
                                TargetMeter,
                                Name::new("Target Mp"),
                            ));

                            parent.spawn((
                                TextBundle::from_section(
                                    format!("Combat Phase: ???"),
                                    get_text_style(&asset_server, 20.),
                                )
                                .with_style(Style {
                                    flex_shrink: 0.,
                                    width: Val::Px(0.),
                                    height: Val::Px(20.),
                                    margin: UiRect {
                                        left: Val::Auto,
                                        right: Val::Auto,
                                        ..default()
                                    },
                                    ..default()
                                }),
                                CombatStateDisplayer,
                                Name::new("Combat Phase"),
                                // -- UI --
                                // Because this is a distinct label widget and
                                // not button/list item text, this is necessary
                                // for accessibility to treat the text accordingly.
                                Label,
                            ));
                        });
                });

            /* -------------------------------------------------------------------------- */
            /*                            Initiative Bar Order                            */
            /* -------------------------------------------------------------------------- */
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(INITIATIVE_BAR_WIDTH),
                            ..default()
                        },
                        background_color: Color::OLIVE.into(),
                        ..default()
                    },
                    Name::new("Initiative Vertical Bar"),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            NodeBundle {
                                style: LIST_HIDDEN_OVERFLOW_STYLE,
                                ..default()
                            },
                            Name::new("List of Actions"),
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: MOVING_PANEL_STYLE,
                                        ..default()
                                    },
                                    Name::new("Moving Panel"),
                                    // -- UI --
                                    ScrollingList::default(),
                                    AccessibilityNode(NodeBuilder::new(Role::List)),
                                ))
                                .with_children(|parent| {
                                    // 36 max actions (12entities playing thrice)

                                    for action_count in 0..36 {
                                        // each Button contains, as child, text and its sprite (caster's head)
                                        parent
                                            .spawn((
                                                ButtonBundle {
                                                    style: ACTION_BUTTON_STYLE,
                                                    background_color: NORMAL_BUTTON.into(),
                                                    visibility: Visibility::Hidden,
                                                    ..default()
                                                },
                                                Name::new(format!("Action {}", action_count)),
                                                // or put the action in it - space but better time comp
                                                ActionDisplayer(action_count),
                                                // -- UI --
                                                AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                            ))
                                            .with_children(|parent| {
                                                parent.spawn(TextBundle::from_section(
                                                    format!("Action {}", action_count),
                                                    get_text_style(&asset_server, 20.),
                                                ));

                                                parent.spawn((
                                                    ImageBundle {
                                                        image: UiImage {
                                                            flip_x: true,
                                                            ..default()
                                                        },
                                                        ..default()
                                                    },
                                                    Name::new(format!("Sprite {}", action_count)),
                                                ));
                                            });
                                    }
                                });
                        });
                });

            /* -------------------------------------------------------------------------- */
            /*                                  HUD Wall                                  */
            /*                  Will Spawn inEnter(GameState::CombatWall)                 */
            /* -------------------------------------------------------------------------- */
        });
}
