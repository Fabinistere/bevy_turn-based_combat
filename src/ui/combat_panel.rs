use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    prelude::*,
    utils::HashMap,
};

use crate::{
    combat::{
        skills::Skill,
        stats::{Attack, AttackSpe, Defense, DefenseSpe, Hp, Initiative, Mana, Shield},
        stuff::Job,
    },
    constants::ui::{dialogs::*, hud_wall::combat::*, style::*},
    ui::{
        combat_system::{HpMeter, MpMeter},
        player_interaction::{EndOfTurnButton, ScrollingList},
    },
};

use super::combat_system::{
    ActionHistoryDisplayer, ActionsLogsDisplayer, LastActionHistoryDisplayer,
};

/* -------------------------------------------------------------------------- */
/*                                UI Components                               */
/* -------------------------------------------------------------------------- */

/// DOC : new name ? CombatWallAssetsResources
#[derive(Resource)]
pub struct CombatWallResources {
    pub base_combat_wall: Handle<Image>,
    pub pack_of_scroll: Handle<Image>,
    pub weapons: Handle<Image>,
}

impl FromWorld for CombatWallResources {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        CombatWallResources {
            base_combat_wall: asset_server.load("textures/UI/HUD/combat/wall/base_combat_wall.png"),
            pack_of_scroll: asset_server.load("textures/UI/HUD/combat/wall/pack_of_scroll.png"),
            weapons: asset_server.load("textures/UI/HUD/combat/wall/stuffs.png"),
        }
    }
}

/// DOC : new name ? CharacterSheetAssetsResources
#[derive(Resource)]
pub struct CharacterSheetResources {
    // pub base_scroll: Handle<Image>,
    // pub base_headers: Handle<Image>,
    pub base_full_scroll: Handle<Image>,
    pub top_left_corner: Handle<Image>,
    pub weapon_frame: Handle<Image>,
}

impl FromWorld for CharacterSheetResources {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        CharacterSheetResources {
            // base_scroll: asset_server.load("textures/UI/HUD/combat/character_sheet/base_scroll.png"),
            // base_headers: asset_server.load("textures/UI/HUD/combat/character_sheet/base_headers.png"),
            // TODO: rename to base_scroll
            base_full_scroll: asset_server
                .load("textures/UI/HUD/combat/character_sheet/base_full_scroll.png"),
            top_left_corner: asset_server
                .load("textures/UI/HUD/combat/character_sheet/top_left_corner.png"),
            weapon_frame: asset_server.load("textures/UI/border/border_05_nobackground.png"),
        }
    }
}

/// XXX: Useless component used to differentiate Hp/MpMeters of a target or a caster
#[derive(Component)]
pub struct TargetMeter;

#[derive(Component)]
pub struct CombatStateDisplayer;

#[derive(Default, Component, Reflect, Deref, DerefMut)]
pub struct ActionDisplayer(pub usize);

/* -------------------------------------------------------------------------- */
/*                              Character's Sheet                             */
/* -------------------------------------------------------------------------- */

/// REFACTOR: Is the id still needed ? Associate entities at the Combat Initiation - no, we want to get from the CSh and from the fighter.
///
/// Contains its id
#[derive(Default, Component, Reflect, Deref, DerefMut)]
pub struct AllyCharacterSheet(pub usize);

/// Points to the unique scroll shared to display all the bestiary
#[derive(Component)]
pub struct EnemyCharacterSheet;

/// REFACTOR: give ez access to hud_position instead of: (CharacterSheetElements, Vec3)
///
/// Associate Allies with their respective sheet
#[derive(Debug, Resource, Reflect, Deref, DerefMut, Clone)]
pub struct CharacterSheetAssociations(pub HashMap<usize, CharacterSheetElements>);

#[derive(PartialEq, Eq, Hash, Clone, Debug, Reflect)]
pub struct CharacterSheetElements {
    /// DOC: naming for fighter
    /// Associated with the CharSheet
    pub fighter: Option<Entity>,
    pub character_sheet: Entity,

    // /// will be converted to (f32, f32, f32),
    // pub hud_position: Option<(i32, i32, i32)>,
    pub portrait: Entity,
    pub name: Entity,
    pub title: Entity,
    pub job: Entity,
    pub weapon: Entity,
    pub health: Entity,
    pub mana: Entity,
    pub shield: Entity,
    pub initiative: Entity,
    pub attack: Entity,
    pub attack_spe: Entity,
    pub defense: Entity,
    pub defense_spe: Entity,
    pub base_skills: Entity,
    pub tier_2_skills: Entity,
    pub tier_1_skills: Entity,
    pub tier_0_skills: Entity,
}

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

/* -------------------------------------------------------------------------- */
/*                                  UI Setup                                  */
/* -------------------------------------------------------------------------- */

/// REFACTOR: Upgrade UiImage to spritesheet UI when [Available](https://github.com/bevyengine/bevy/pull/5070)
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    character_sheet_resources: Res<CharacterSheetResources>,
    combat_wall_resources: Res<CombatWallResources>,
    mut character_sheet_associations: ResMut<CharacterSheetAssociations>,
) {
    /* -------------------------------------------------------------------------- */
    /*                              Characters' Sheet                             */
    /* -------------------------------------------------------------------------- */

    let mut characters_sheets = Vec::new();
    // TODO: replace this max limit with the number of actual allies
    for i in 0..12 {
        /* -------------------------------------------------------------------------- */
        /*                                 UI Elements                                */
        /* -------------------------------------------------------------------------- */

        // TODO: Upgrade when Available - use Spritesheet
        // TODO: Apply a mask to conceal the legs of the portrait (or simply change asset but meh.)
        // Or put the portrait between the baseScroll and the base Header (but doesn't work)
        // FIXME: commands.spawn won't work, parent will spawn at the wrong place (in HUD Wall not in the intended location)
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
                TextBundle::from_section(
                    format!("Mana: ???/???"),
                    get_text_style(&asset_server, 20.),
                )
                .with_style(TEXT_STYLE),
                Label,
                Name::new("Mana"),
                Mana::default(),
            ))
            .id();

        let shield = commands
            .spawn((
                TextBundle::from_section(
                    format!("Shield: ???"),
                    get_text_style(&asset_server, 20.),
                )
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
                TextBundle::from_section(
                    format!("Attack: ???"),
                    get_text_style(&asset_server, 20.),
                )
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
                TextBundle::from_section(
                    format!("Defense: ???"),
                    get_text_style(&asset_server, 20.),
                )
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
                        size: Size::all(Val::Px(50.)),
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
                        // size: Size::height(Val::Percent(42.)),
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
                        // size: Size::height(Val::Percent(42.)),
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
                                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                                    // center button
                                    margin: UiRect::all(Val::Auto),
                                    // horizontally center child text
                                    justify_content: JustifyContent::Center,
                                    // vertically center child text
                                    align_items: AlignItems::Center,
                                    position: UiRect::default(),
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
                        // size: Size::height(Val::Percent(42.)),
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
                                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                                    // center button
                                    margin: UiRect::all(Val::Auto),
                                    // horizontally center child text
                                    justify_content: JustifyContent::Center,
                                    // vertically center child text
                                    align_items: AlignItems::Center,
                                    position: UiRect::default(),
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
                        // size: Size::height(Val::Percent(42.)),
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
                                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                                    // center button
                                    margin: UiRect::all(Val::Auto),
                                    // horizontally center child text
                                    justify_content: JustifyContent::Center,
                                    // vertically center child text
                                    align_items: AlignItems::Center,
                                    position: UiRect::default(),
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
                ImageBundle {
                    image: character_sheet_resources.base_full_scroll.clone().into(),
                    style: Style {
                        size: Size::width(Val::Percent(100.)),
                        position: UiRect {
                            right: Val::Percent(
                                CHARACTER_SHEET_OFFSET_X + CHARACTER_SHEET_WIDTH * (i as f32 + 1.),
                            ),
                            bottom: if i < 3 {
                                Val::Percent(CHARACTER_SHEET_FIRST_ROW_Y)
                            } else {
                                Val::Percent(CHARACTER_SHEET_SECOND_ROW_Y)
                            },
                            ..default()
                        },
                        // flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    // depends on the number of ally
                    visibility: Visibility::Hidden,
                    transform: Transform::from_scale(Vec3::splat(0.2)),
                    ..default()
                },
                Name::new(format!("Character's Sheet {}", i)),
                Interaction::default(),
                AllyCharacterSheet(i),
            ))
            .with_children(|parent| {
                // Headers
                parent
                    .spawn((
                        NodeBundle {
                            style: Style {
                                size: Size::height(Val::Percent(20.)),
                                flex_direction: FlexDirection::Row,
                                ..default()
                            },
                            // background_color: Color::DARK_GRAY.into(),
                            ..default()
                        },
                        Name::new("Headers"),
                    ))
                    .with_children(|parent| {
                        parent
                            .spawn((
                                NodeBundle {
                                    style: Style {
                                        size: Size::width(Val::Percent(30.)),
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    // background_color: Color::CRIMSON.into(),
                                    ..default()
                                },
                                Name::new("Sprite Border"),
                            ))
                            .push_children(&[portrait]);

                        parent
                            .spawn((
                                NodeBundle {
                                    style: Style {
                                        size: Size::width(Val::Percent(70.)),
                                        flex_direction: FlexDirection::Column,
                                        ..default()
                                    },
                                    // background_color: Color::GRAY.into(),
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
                                                size: Size::height(Val::Percent(50.)),
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
                                                size: Size::height(Val::Percent(50.)),
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
                                size: Size::height(Val::Percent(40.)),
                                flex_direction: FlexDirection::Row,
                                ..default()
                            },
                            // background_color: Color::CRIMSON.into(),
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
                                        size: Size::width(Val::Percent(60.)),
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    // background_color: Color::GRAY.into(),
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
                                        size: Size::width(Val::Percent(40.)),
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
                                                size: Size::all(Val::Px(100.)),
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
                                size: Size::height(Val::Percent(40.)),
                                flex_direction: FlexDirection::Column,
                                // align_content: AlignContent::SpaceAround,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            // background_color: Color::AZURE.into(),
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
                //                 size: Size {
                //                     height: Val::Percent(100.),
                //                     width: Val::Percent(100.),
                //                 },
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
                //                     size: Size {
                //                         width: Val::Px(500.),
                //                         height: Val::Percent(100.),
                //                     },
                //                     ..default()
                //                 },
                //                 ..default()
                //             },
                //             Name::new("Decoration - Top Left Corner"),
                //         ));
                //     });
            })
            .id();

        character_sheet_associations.insert(
            i,
            CharacterSheetElements {
                fighter: None,
                character_sheet,
                // hud_position: if i < 6 { Some((0, 0, 0)) } else { None },
                portrait,
                name,
                title,
                job,
                weapon,
                health,
                mana,
                shield,
                initiative,
                attack,
                attack_spe,
                defense,
                defense_spe,
                base_skills,
                tier_2_skills,
                tier_1_skills,
                tier_0_skills,
            },
        );

        characters_sheets.push(character_sheet);
    }

    /* -------------------------------------------------------------------------- */
    /*                                  UI Scene                                  */
    /* -------------------------------------------------------------------------- */

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::width(Val::Percent(100.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            },
            Name::new("UI Scene"),
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
                            size: Size::width(Val::Percent(56.)),
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
                                    size: Size::new(Val::Px(200.0), Val::Px(65.0)),
                                    margin: UiRect::all(Val::Auto),
                                    position: UiRect {
                                        top: Val::Percent(5.),
                                        ..default()
                                    },
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
                                    position: UiRect {
                                        top: Val::Percent(5.),
                                        ..default()
                                    },
                                    flex_direction: FlexDirection::Column,
                                    flex_grow: 1.0,
                                    max_size: Size::UNDEFINED,
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
                                    size: Size::new(Val::Undefined, Val::Px(20.)),
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
                            size: Size::width(Val::Percent(8.)),
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
            /* -------------------------------------------------------------------------- */
            parent
                .spawn((
                    // NodeBundle {
                    //     style: Style {
                    //         size: Size::width(Val::Percent(36.)),
                    //         // REFACTOR: For multiple charsheet, Row could be better
                    //         flex_direction: FlexDirection::Column,
                    //         ..default()
                    //     },
                    //     background_color: Color::ANTIQUE_WHITE.into(),
                    //     ..default()
                    // },
                    ImageBundle {
                        image: combat_wall_resources.base_combat_wall.clone().into(),
                        style: Style {
                            size: Size::width(Val::Percent(36.)),
                            // REFACTOR: For multiple charsheet, Row could be better
                            // flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("HUD Wall"),
                ))
                .push_children(&characters_sheets)
                .with_children(|parent| {
                    /* First Side of the HUD Wall
                     * - TODO: Each Ally CharacterSheet (SubPanels)
                     *   - First Sub-Panel
                     *     - Headers: Sprite, Name, Title, Job
                     *     - Stats, Weapon Equiped
                     *     - Skill Menu²
                     * - TODO: "Bestiary" (Book of Enemy's characterSheet)
                     * - TODO: Logs
                     * - TODO: Team's Inventory
                     */

                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    size: Size::height(Val::Percent(20.)),
                                    flex_direction: FlexDirection::Column,
                                    align_self: AlignSelf::Center,
                                    overflow: Overflow::Hidden,
                                    ..default()
                                },
                                background_color: Color::GRAY.into(),
                                visibility: Visibility::Hidden,
                                ..default()
                            },
                            Name::new("Logs"),
                        ))
                        .with_children(|parent| {
                            // Moving panel
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: MOVING_PANEL_STYLE,
                                        ..default()
                                    },
                                    ScrollingList::default(),
                                    AccessibilityNode(NodeBuilder::new(Role::List)),
                                    Name::new("Moving Panel"),
                                ))
                                .with_children(|parent| {
                                    // TODO: UI - Title that's stick to the top while scrolling their section
                                    // List items

                                    parent.spawn((
                                        TextBundle::from_section(
                                            format!("Combat Phase: ???"),
                                            get_text_style(&asset_server, 20.),
                                        )
                                        .with_style(
                                            Style {
                                                flex_shrink: 0.,
                                                size: Size::new(Val::Undefined, Val::Px(20.)),
                                                margin: UiRect {
                                                    left: Val::Auto,
                                                    right: Val::Auto,
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                        ),
                                        CombatStateDisplayer,
                                        Name::new("Combat Phase"),
                                        // -- UI --
                                        // Because this is a distinct label widget and
                                        // not button/list item text, this is necessary
                                        // for accessibility to treat the text accordingly.
                                        Label,
                                        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                    ));

                                    parent.spawn((
                                        TextBundle::from_section(
                                            format!("---------------\nActions:"),
                                            get_text_style(&asset_server, 20.),
                                        )
                                        .with_style(
                                            Style {
                                                flex_wrap: FlexWrap::Wrap,
                                                // flex_shrink: 0.,
                                                size: Size::new(Val::Auto, Val::Auto),
                                                // margin: UiRect {
                                                //     left: Val::Auto,
                                                //     right: Val::Auto,
                                                //     ..default()
                                                // },
                                                ..default()
                                            },
                                        ),
                                        ActionHistoryDisplayer,
                                        Name::new("Actions History"),
                                        // -- UI --
                                        Label,
                                        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                    ));

                                    parent.spawn((
                                        TextBundle::from_section(
                                            format!("---------------\nLast Actions:"),
                                            get_text_style(&asset_server, 20.),
                                        )
                                        .with_style(
                                            Style {
                                                flex_wrap: FlexWrap::Wrap,
                                                // flex_shrink: 0.,
                                                size: Size::new(Val::Auto, Val::Auto),
                                                // margin: UiRect {
                                                //     left: Val::Auto,
                                                //     right: Val::Auto,
                                                //     ..default()
                                                // },
                                                ..default()
                                            },
                                        ),
                                        LastActionHistoryDisplayer,
                                        Name::new("Last Actions History"),
                                        // -- UI --
                                        Label,
                                        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                    ));

                                    parent.spawn((
                                        TextBundle::from_section(
                                            format!("---------------\nActions Logs:"),
                                            get_text_style(&asset_server, 20.),
                                        )
                                        .with_style(
                                            Style {
                                                flex_wrap: FlexWrap::Wrap,
                                                // flex_shrink: 0.,
                                                size: Size::new(Val::Auto, Val::Auto),
                                                // margin: UiRect {
                                                //     left: Val::Auto,
                                                //     right: Val::Auto,
                                                //     ..default()
                                                // },
                                                ..default()
                                            },
                                        ),
                                        ActionsLogsDisplayer,
                                        Name::new("Actions Logs"),
                                        // -- UI --
                                        Label,
                                        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                    ));

                                    parent.spawn((
                                        TextBundle::from_section(
                                            format!("---------------"),
                                            get_text_style(&asset_server, 20.),
                                        )
                                        .with_style(
                                            Style {
                                                flex_wrap: FlexWrap::Wrap,
                                                size: Size::new(Val::Auto, Val::Auto),
                                                ..default()
                                            },
                                        ),
                                        Name::new("----"),
                                        // -- UI --
                                        Label,
                                        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                    ));
                                });
                        });

                    // TODO: add UILocation::CharacterSheet(Ally | Enemy) | Logs | CombatHUB | etc
                    // TODO: Hide Panels on the right side (and add anim)
                });
        });
}
