use bevy::prelude::*;

use crate::constants::ui::dialogs::*;

use super::{dialog_player::ScrollingList, dialog_combat::{UnitSelected, UnitTargeted, HpMeter, MpMeter, ButtonSelection}};

pub fn setup(
    mut commands: Commands,
    
    asset_server: Res<AssetServer>,
) {
    
    commands
        .spawn(ButtonBundle {
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
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Button",
                TextStyle {
                    font: asset_server.load("fonts/dpcomic.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });

    commands
        .spawn((ButtonBundle {
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
        ButtonSelection,
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
    // List with hidden overflow
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Center,
                size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
                overflow: Overflow::Hidden,
                ..default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
            ..default()
        })
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
                    UnitSelected(None),
                    UnitTargeted(None),
                ))
                .with_children(|parent| {
                    // List items
                    parent
                        .spawn((
                            TextBundle::from_section(
                                format!("hp: HP"),
                                TextStyle {
                                    font: asset_server
                                        .load("fonts/dpcomic.ttf"),
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
                        ));

                    parent.spawn((
                        TextBundle::from_section(
                            format!("mp: MP"),
                            TextStyle {
                                font: asset_server
                                    .load("fonts/dpcomic.ttf"),
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
                    ));
                });
            });
}
