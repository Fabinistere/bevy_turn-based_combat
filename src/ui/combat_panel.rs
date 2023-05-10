use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    prelude::*,
};

use crate::{
    combat::{skills::Skill, CombatPanel, CombatState},
    constants::ui::dialogs::*,
    ui::combat_system::{
        ActionHistoryDisplayer, ActionsLogs, HpMeter, LastActionHistoryDisplayer, MpMeter,
    },
    ui::player_interaction::{EndOfTurnButton, ScrollingList},
};

/// XXX: Useless component used to differentiate Hp/MpMeters of a target or a caster
#[derive(Component)]
pub struct TargetMeter;

/// XXX: Useless component used to differentiate Hp/MpMeters of a target or a caster
#[derive(Component)]
pub struct CasterMeter;

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
            // Fighting Hall - Where the npcs are
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
                                TextStyle {
                                    font: asset_server.load("fonts/dpcomic.ttf"),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });

                    // Stats - Caster / Target
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
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

                            // ----- Basic Stats -----
                            parent.spawn((
                                TextBundle::from_section(
                                    format!("Caster hp: ???"),
                                    TextStyle {
                                        font: asset_server.load("fonts/dpcomic.ttf"),
                                        font_size: 20.,
                                        color: Color::WHITE,
                                    },
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
                                HpMeter,
                                CasterMeter,
                                Name::new("Caster Hp"),
                            ));

                            parent.spawn((
                                TextBundle::from_section(
                                    format!("Caster mp: ???"),
                                    TextStyle {
                                        font: asset_server.load("fonts/dpcomic.ttf"),
                                        font_size: 20.,
                                        color: Color::WHITE,
                                    },
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
                                MpMeter,
                                CasterMeter,
                                Name::new("Caster Mp"),
                            ));

                            parent.spawn((
                                TextBundle::from_section(
                                    format!("Target hp: ???"),
                                    TextStyle {
                                        font: asset_server.load("fonts/dpcomic.ttf"),
                                        font_size: 20.,
                                        color: Color::WHITE,
                                    },
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
                                HpMeter,
                                TargetMeter,
                                Name::new("Target Hp"),
                            ));

                            parent.spawn((
                                TextBundle::from_section(
                                    format!("Target mp: ???"),
                                    TextStyle {
                                        font: asset_server.load("fonts/dpcomic.ttf"),
                                        font_size: 20.,
                                        color: Color::WHITE,
                                    },
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
                                MpMeter,
                                TargetMeter,
                                Name::new("Target Mp"),
                            ));
                        });
                });

            // Initiative Bar Order
            parent.spawn(NodeBundle {
                style: Style {
                    size: Size::width(Val::Percent(8.)),
                    ..default()
                },
                background_color: Color::OLIVE.into(),
                ..default()
            }).with_children(|parent| {
                // 24 max actions (12entities playing twice)


            });

            // HUD Wall
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            size: Size::width(Val::Percent(36.)),
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        background_color: Color::SILVER.into(),
                        ..default()
                    },
                    Name::new("HUD Wall"),
                ))
                .with_children(|parent| {
                    // Skill Menu
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::height(Val::Percent(42.)),
                                ..default()
                            },
                            background_color: Color::AZURE.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // 6 skill max

                            // SKILL 1
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
                                        ..default()
                                    },
                                    Name::new("Skill 1"),
                                    Skill::bam(),
                                    // Draggable,
                                    // Clickable,
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Bam",
                                        TextStyle {
                                            font: asset_server.load("fonts/dpcomic.ttf"),
                                            font_size: 40.0,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                    ));
                                });

                            // Gifle SKILL
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                                            margin: UiRect::all(Val::Auto),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            position: UiRect::default(),
                                            ..default()
                                        },
                                        background_color: NORMAL_BUTTON.into(),
                                        ..default()
                                    },
                                    Name::new("Skill 2"),
                                    Skill::gifle(),
                                    // Draggable,
                                    // Clickable,
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Gifle",
                                        TextStyle {
                                            font: asset_server.load("fonts/dpcomic.ttf"),
                                            font_size: 40.0,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                    ));
                                });

                            // Implosion SKILL
                            // `Deal 25dmg to 3targets` (example of multi-targets skills)
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                                            margin: UiRect::all(Val::Auto),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            position: UiRect::default(),
                                            ..default()
                                        },
                                        background_color: NORMAL_BUTTON.into(),
                                        ..default()
                                    },
                                    Name::new("Skill 3"),
                                    Skill::implosion(),
                                    // Draggable,
                                    // Clickable,
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Implosion",
                                        TextStyle {
                                            font: asset_server.load("fonts/dpcomic.ttf"),
                                            font_size: 40.0,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                    ));
                                });
                        });

                    // Logs - List with hidden overflow
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    size: Size::height(Val::Percent(58.)),
                                    flex_direction: FlexDirection::Column,
                                    align_self: AlignSelf::Center,
                                    overflow: Overflow::Hidden,
                                    ..default()
                                },
                                background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                                ..default()
                            },
                            Name::new("Logs"),
                        ))
                        .with_children(|parent| {
                            // Moving panel
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            flex_direction: FlexDirection::Column,
                                            max_size: Size::UNDEFINED,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    ScrollingList::default(),
                                    AccessibilityNode(NodeBuilder::new(Role::List)),
                                    Name::new("Moving Panel"),
                                ))
                                .with_children(|parent| {
                                    // List items

                                    parent.spawn((
                                        TextBundle::from_section(
                                            format!("Combat Phase: ???"),
                                            TextStyle {
                                                font: asset_server.load("fonts/dpcomic.ttf"),
                                                font_size: 20.,
                                                color: Color::WHITE,
                                            },
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
                                        // TODO: Move it somewhere else
                                        // CombatState::Initiation
                                        CombatPanel {
                                            phase: CombatState::SelectionCaster,
                                            history: vec![],
                                        },
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
                                            TextStyle {
                                                font: asset_server.load("fonts/dpcomic.ttf"),
                                                font_size: 20.,
                                                color: Color::WHITE,
                                            },
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
                                            TextStyle {
                                                font: asset_server.load("fonts/dpcomic.ttf"),
                                                font_size: 20.,
                                                color: Color::WHITE,
                                            },
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
                                            TextStyle {
                                                font: asset_server.load("fonts/dpcomic.ttf"),
                                                font_size: 20.,
                                                color: Color::WHITE,
                                            },
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
                                        ActionsLogs,
                                        Name::new("Actions Logs"),
                                        // -- UI --
                                        Label,
                                        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                    ));

                                    parent.spawn((
                                        TextBundle::from_section(
                                            format!("---------------"),
                                            TextStyle {
                                                font: asset_server.load("fonts/dpcomic.ttf"),
                                                font_size: 20.,
                                                color: Color::WHITE,
                                            },
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
                });
        });
}