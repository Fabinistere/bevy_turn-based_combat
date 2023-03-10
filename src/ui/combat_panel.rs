use bevy::prelude::*;

use crate::{
    combat::{CombatPhase, CombatState},
    constants::ui::dialogs::*,
    ui::combat_system::{ButtonSelection, ButtonTargeting, HpMeter, MpMeter},
    ui::player_interaction::ScrollingList,
};

use super::player_interaction::{Draggable, Clickable};

/// XXX: Useless component used to differentiate Hp/MpMeters of a target or a caster
#[derive(Component)]
pub struct TargetMeter;

/// XXX: Useless component used to differentiate Hp/MpMeters of a target or a caster
#[derive(Component)]
pub struct CasterMeter;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // BAM SKILL
    commands
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
                    position: UiRect {
                        right: Val::Percent(-20.0),
                        top: Val::Percent(-33.0),
                        ..default()
                    },
                    ..default()
                },
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            Name::new("BAM Skill"),
            Draggable,
            Clickable,
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

    // SELECT BUTTON
    commands
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
                    position: UiRect {
                        right: Val::Percent(-22.0),
                        top: Val::Percent(-41.0),
                        ..default()
                    },
                    ..default()
                },
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            ButtonSelection,
            Name::new("SelectButton")
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "SelectUnit",
                TextStyle {
                    font: asset_server.load("fonts/dpcomic.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
    
    // TARGET BUTTON
    commands
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
                    ..default()
                },
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            ButtonTargeting,
            Name::new("TargetButton")
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "TargetUnit",
                TextStyle {
                    font: asset_server.load("fonts/dpcomic.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });

    // STATS
    // List with hidden overflow
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_self: AlignSelf::Center,
                    size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
                    overflow: Overflow::Hidden,
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                ..default()
            },
            Name::new("Scrolling Panel"),
        ))
        .with_children(|parent| {
            // Moving panel
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
                    ScrollingList::default(),
                    // TODO: Move it somewhere else
                    // CombatPhase(CombatState::Initiation),
                    CombatPhase(CombatState::SelectionCaster),
                    Name::new("Moving Panel"),
                ))
                .with_children(|parent| {
                    // List items
                    parent.spawn((
                        TextBundle::from_section(
                            format!("Caster hp: HP"),
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
                            format!("Caster mp: MP"),
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

                    // TODO: display targeted entity
                    parent.spawn((
                        TextBundle::from_section(
                            format!("Target hp: HP"),
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
                            format!("Target mp: MP"),
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
}
