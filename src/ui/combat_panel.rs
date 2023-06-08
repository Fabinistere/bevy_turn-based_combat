use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    prelude::*,
};

use crate::{
    combat::{skills::Skill, stats::{Hp, Mana, Shield, Initiative, Attack, AttackSpe, Defense, DefenseSpe}, stuff::Job},
    constants::ui::{dialogs::*, style::*},
    ui::{
        combat_system::{
            HpMeter, MpMeter,
        },
        player_interaction::{EndOfTurnButton, ScrollingList},
    },
};

/* -------------------------------------------------------------------------- */
/*                                UI Components                               */
/* -------------------------------------------------------------------------- */

/// XXX: Useless component used to differentiate Hp/MpMeters of a target or a caster
#[derive(Component)]
pub struct TargetMeter;

#[derive(Component)]
pub struct CombatStateDisplayer;

#[derive(Default, Component, Reflect, Deref, DerefMut)]
pub struct ActionDisplayer(pub usize);

#[derive(Default, Component, Reflect, Deref, DerefMut)]
pub struct SkillDisplayer(pub usize);

#[derive(Component)]
pub struct WeaponDisplayer;

// REFACTOR: SkillBar Structure

#[derive(Component, Reflect, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum SkillBar {
    Base,
    Tier2,
    Tier1,
    Tier0,
    /// TODO: Won'tHave (PostDemo) - Unlock by the Job XpTree
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
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Scene
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
                    NodeBundle {
                        style: Style {
                            size: Size::width(Val::Percent(36.)),
                            // Row could be better ?
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        background_color: Color::ANTIQUE_WHITE.into(),
                        ..default()
                    },
                    Name::new("HUD Wall"),
                ))
                .with_children(|parent| {
                    /* First Side of the HUD Wall
                     * - TODO: Each Ally CharacterSheet (SubPanels)
                     *   - First Sub-Panel
                     *     - TODO: Headers: Sprite, Name, Title, Job, (xp)
                     *     - TODO: Stats, Weapon Equiped
                     *     - Skill Menu
                     * - TODO: "Bestiary" (Book of Enemy's characterSheet)
                     * - TODO: Logs
                     * - TODO: Team's Inventory
                     */

                    /*
                        // // TODO: Logs - List with hidden overflow
                        // parent
                        //     .spawn((
                        //         NodeBundle {
                        //             style: Style {
                        //                 size: Size::height(Val::Percent(20.)),
                        //                 flex_direction: FlexDirection::Column,
                        //                 align_self: AlignSelf::Center,
                        //                 overflow: Overflow::Hidden,
                        //                 ..default()
                        //             },
                        //             background_color: Color::GRAY.into(),
                        //             ..default()
                        //         },
                        //         Name::new("Logs"),
                        //     ))
                        //     .with_children(|parent| {
                        //         // Moving panel
                        //         parent
                        //             .spawn((
                        //                 NodeBundle {
                        //                     style: MOVING_PANEL_STYLE,
                        //                     ..default()
                        //                 },
                        //                 ScrollingList::default(),
                        //                 AccessibilityNode(NodeBuilder::new(Role::List)),
                        //                 Name::new("Moving Panel"),
                        //             ))
                        //             .with_children(|parent| {

                        //                 // TODO: UI - Title that's stick to the top while scrolling their section
                        //                 // List items

                        //                 parent.spawn((
                        //                     TextBundle::from_section(
                        //                         format!("Combat Phase: ???"),
                        //                         get_text_style(&asset_server, 20.),
                        //                     )
                        //                     .with_style(
                        //                         Style {
                        //                             flex_shrink: 0.,
                        //                             size: Size::new(Val::Undefined, Val::Px(20.)),
                        //                             margin: UiRect {
                        //                                 left: Val::Auto,
                        //                                 right: Val::Auto,
                        //                                 ..default()
                        //                             },
                        //                             ..default()
                        //                         },
                        //                     ),
                        //                     CombatStateDisplayer,
                        //                     Name::new("Combat Phase"),
                        //                     // -- UI --
                        //                     // Because this is a distinct label widget and
                        //                     // not button/list item text, this is necessary
                        //                     // for accessibility to treat the text accordingly.
                        //                     Label,
                        //                     AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                        //                 ));

                        //                 parent.spawn((
                        //                     TextBundle::from_section(
                        //                         format!("---------------\nActions:"),
                        //                         get_text_style(&asset_server, 20.),
                        //                     )
                        //                     .with_style(
                        //                         Style {
                        //                             flex_wrap: FlexWrap::Wrap,
                        //                             // flex_shrink: 0.,
                        //                             size: Size::new(Val::Auto, Val::Auto),
                        //                             // margin: UiRect {
                        //                             //     left: Val::Auto,
                        //                             //     right: Val::Auto,
                        //                             //     ..default()
                        //                             // },
                        //                             ..default()
                        //                         },
                        //                     ),
                        //                     ActionHistoryDisplayer,
                        //                     Name::new("Actions History"),
                        //                     // -- UI --
                        //                     Label,
                        //                     AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                        //                 ));

                        //                 parent.spawn((
                        //                     TextBundle::from_section(
                        //                         format!("---------------\nLast Actions:"),
                        //                         get_text_style(&asset_server, 20.),
                        //                     )
                        //                     .with_style(
                        //                         Style {
                        //                             flex_wrap: FlexWrap::Wrap,
                        //                             // flex_shrink: 0.,
                        //                             size: Size::new(Val::Auto, Val::Auto),
                        //                             // margin: UiRect {
                        //                             //     left: Val::Auto,
                        //                             //     right: Val::Auto,
                        //                             //     ..default()
                        //                             // },
                        //                             ..default()
                        //                         },
                        //                     ),
                        //                     LastActionHistoryDisplayer,
                        //                     Name::new("Last Actions History"),
                        //                     // -- UI --
                        //                     Label,
                        //                     AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                        //                 ));

                        //                 parent.spawn((
                        //                     TextBundle::from_section(
                        //                         format!("---------------\nActions Logs:"),
                        //                         get_text_style(&asset_server, 20.),
                        //                     )
                        //                     .with_style(
                        //                         Style {
                        //                             flex_wrap: FlexWrap::Wrap,
                        //                             // flex_shrink: 0.,
                        //                             size: Size::new(Val::Auto, Val::Auto),
                        //                             // margin: UiRect {
                        //                             //     left: Val::Auto,
                        //                             //     right: Val::Auto,
                        //                             //     ..default()
                        //                             // },
                        //                             ..default()
                        //                         },
                        //                     ),
                        //                     ActionsLogsDisplayer,
                        //                     Name::new("Actions Logs"),
                        //                     // -- UI --
                        //                     Label,
                        //                     AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                        //                 ));

                        //                 parent.spawn((
                        //                     TextBundle::from_section(
                        //                         format!("---------------"),
                        //                         get_text_style(&asset_server, 20.),
                        //                     )
                        //                     .with_style(
                        //                         Style {
                        //                             flex_wrap: FlexWrap::Wrap,
                        //                             size: Size::new(Val::Auto, Val::Auto),
                        //                             ..default()
                        //                         },
                        //                     ),
                        //                     Name::new("----"),
                        //                     // -- UI --
                        //                     Label,
                        //                     AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                        //                 ));
                        //             });
                        //     });
                    */

                    // TODO: add UILocation::CharacterSheet(Ally | Enemy) | Logs | CombatHUB | etc
                    // TODO: Hide Panels on the right side (and add anim)
                    // TODO: for i in 0..6 {} and Turn off visibility for unused sheet (empty recruted)
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    size: Size::height(Val::Percent(100.)),
                                    flex_direction: FlexDirection::Column,
                                    ..default()
                                },
                                background_color: Color::SILVER.into(),
                                ..default()
                            },
                            Name::new("Character's Sheet"),
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
                                                    size: Size::width(Val::Percent(30.)),
                                                    justify_content: JustifyContent::Center,
                                                    ..default()
                                                },
                                                background_color: Color::CRIMSON.into(),
                                                ..default()
                                            },
                                            Name::new("Sprite Border"),
                                        ))
                                        .with_children(|parent| {
                                            // TODO: Upgrade when Available - use Spritesheet
                                            parent.spawn((
                                                ImageBundle {
                                                    image: UiImage {
                                                        texture: asset_server.load(
                                                            "textures/character/idle/idle_Fabien_Loyal.png",
                                                        ),
                                                        ..default()
                                                    },
                                                    ..default()
                                                } ,
                                                Name::new("Portrait"),
                                                Portrait,
                                            ));
                                        });
                                    
                                    parent
                                        .spawn((
                                            NodeBundle {
                                                style: Style {
                                                    size: Size::width(Val::Percent(70.)),
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
                                                            size: Size::height(Val::Percent(50.)),
                                                            flex_direction: FlexDirection::Row,
                                                            ..default()
                                                        },
                                                        ..default()
                                                    },
                                                    Name::new("Full Name"),
                                                ))
                                                .with_children(|parent| {
                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            format!("Name"),
                                                            get_text_style(&asset_server, 40.),
                                                        )
                                                        .with_style(TEXT_STYLE),
                                                        Name::new("Name"),
                                                        FabienName,
                                                    ));

                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            format!("Fabien"),
                                                            get_text_style(&asset_server, 20.),
                                                        )
                                                        .with_style(TEXT_STYLE),
                                                        Name::new("Title"),
                                                        Title,
                                                    ));
                                                });

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
                                                .with_children(|parent| {
                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            format!("Chill"),
                                                            get_text_style(&asset_server, 20.),
                                                        )
                                                        .with_style(TEXT_STYLE),
                                                        Name::new("Job"),
                                                        Job::default(),
                                                    ));
                                                });
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
                                                    size: Size::width(Val::Percent(60.)),
                                                    flex_direction: FlexDirection::Column,
                                                    justify_content: JustifyContent::Center,
                                                    ..default()
                                                },
                                                background_color: Color::GRAY.into(),
                                                ..default()
                                            },
                                            Name::new("Stats"),
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn((
                                                TextBundle::from_section(
                                                    format!("Health: ???/???"),
                                                    get_text_style(&asset_server, 20.),
                                                )
                                                .with_style(TEXT_STYLE),
                                                Name::new("Health"),
                                                Hp::default(),
                                            ));

                                            parent.spawn((
                                                TextBundle::from_section(
                                                    format!("Mana: ???/???"),
                                                    get_text_style(&asset_server, 20.),
                                                )
                                                .with_style(TEXT_STYLE),
                                                Name::new("Mana"),
                                                Mana::default(),
                                            ));

                                            parent.spawn((
                                                TextBundle::from_section(
                                                    format!("Shield: ???"),
                                                    get_text_style(&asset_server, 20.),
                                                )
                                                .with_style(TEXT_STYLE),
                                                Name::new("Shield"),
                                                Shield::default(),
                                            ));

                                            parent.spawn((
                                                TextBundle::from_section(
                                                    format!("Initiative: ???"),
                                                    get_text_style(&asset_server, 20.),
                                                )
                                                .with_style(TEXT_STYLE),
                                                Name::new("Initiative"),
                                                Initiative::default(),
                                            ));

                                            parent.spawn((
                                                TextBundle::from_section(
                                                    format!("Attack: ???"),
                                                    get_text_style(&asset_server, 20.),
                                                )
                                                .with_style(TEXT_STYLE),
                                                Name::new("Attack"),
                                                Attack::default(),
                                            ));

                                            parent.spawn((
                                                TextBundle::from_section(
                                                    format!("AttackSpe: ???"),
                                                    get_text_style(&asset_server, 20.),
                                                )
                                                .with_style(TEXT_STYLE),
                                                Name::new("AttackSpe"),
                                                AttackSpe::default(),
                                            ));

                                            parent.spawn((
                                                TextBundle::from_section(
                                                    format!("Defense: ???"),
                                                    get_text_style(&asset_server, 20.),
                                                )
                                                .with_style(TEXT_STYLE),
                                                Name::new("Defense"),
                                                Defense::default(),
                                            ));

                                            parent.spawn((
                                                TextBundle::from_section(
                                                    format!("DefenseSpe: ???"),
                                                    get_text_style(&asset_server, 20.),
                                                )
                                                .with_style(TEXT_STYLE),
                                                Name::new("DefenseSpe"),
                                                DefenseSpe::default(),
                                            ));
                                        });

                                    parent
                                        .spawn((
                                            NodeBundle {
                                                style: Style {
                                                    size: Size::width(Val::Percent(40.)),
                                                    justify_content: JustifyContent::Center,
                                                    ..default()
                                                },
                                                background_color: Color::BISQUE.into(),
                                                ..default()
                                            },
                                            Name::new("Equipements Section"),
                                        ))
                                        .with_children(|parent| {
                                            // TODO: add frame underneath
                                            parent
                                                .spawn((
                                                    ImageBundle {
                                                        image: UiImage {
                                                            texture: asset_server.load(
                                                                "textures/ui/border/border_05_nobackground.png",
                                                            ),
                                                            ..default()
                                                        },
                                                        style: Style {
                                                            size: Size::all(Val::Px(100.)),
                                                            align_self: AlignSelf::Center,
                                                            justify_content: JustifyContent::Center,
                                                            ..default()
                                                        },
                                                        ..default()
                                                    } ,
                                                    Name::new("Frame")
                                                ))
                                                .with_children(|parent| {
                                                    parent.spawn((
                                                        ImageBundle {
                                                            image: UiImage {
                                                                texture: asset_server.load(
                                                                    "textures/icons/weapons/fish_01b.png",
                                                                ),
                                                                ..default()
                                                            },
                                                            style: Style {
                                                                size: Size::all(Val::Px(50.)),
                                                                align_self: AlignSelf::Center,
                                                                ..default()
                                                            },
                                                            visibility: Visibility::Hidden,
                                                            ..default()
                                                        } ,
                                                        Name::new("Weapon"),
                                                        WeaponDisplayer,
                                                    ));
                                                });
                                            
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
                                        background_color: Color::AZURE.into(),
                                        ..default()
                                    },
                                    Name::new("Skill Menu"),
                                ))
                                .with_children(|parent| {
                                    // A catalogue, one row for basic skill, a row for tier2 ,etc (simplify a lot skill_visibility)
                                    parent
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
                                                            get_text_style(&asset_server, 40.),
                                                        ));
                                                    });
                                            }
                                        });

                                    parent
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
                                                                size: Size::new(
                                                                    Val::Px(150.0),
                                                                    Val::Px(65.0),
                                                                ),
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
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load("fonts/dpcomic.ttf"),
                                                                font_size: 40.0,
                                                                color: Color::rgb(0.9, 0.9, 0.9),
                                                            },
                                                        ));
                                                    });
                                            }
                                        });

                                    parent
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
                                                                size: Size::new(
                                                                    Val::Px(150.0),
                                                                    Val::Px(65.0),
                                                                ),
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
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load("fonts/dpcomic.ttf"),
                                                                font_size: 40.0,
                                                                color: Color::rgb(0.9, 0.9, 0.9),
                                                            },
                                                        ));
                                                    });
                                            }
                                        });

                                    parent
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
                                                                size: Size::new(
                                                                    Val::Px(150.0),
                                                                    Val::Px(65.0),
                                                                ),
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
                                                            TextStyle {
                                                                font: asset_server
                                                                    .load("fonts/dpcomic.ttf"),
                                                                font_size: 40.0,
                                                                color: Color::rgb(0.9, 0.9, 0.9),
                                                            },
                                                        ));
                                                    });
                                            }
                                        });
                                });
                        });
                });
        });
}
