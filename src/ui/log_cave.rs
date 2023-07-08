//! Handle all spawn and component whihc are only present in the LogCave

use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    prelude::*,
};

use crate::{
    combat::GameState,
    constants::ui::{dialogs::*, style::*},
    ui::{
        combat_panel::{Ladder, UIScene},
        combat_system::{HpMeter, MpMeter},
        player_interaction::{EndOfTurnButton, ScrollingList},
    },
};

use super::combat_panel::{ActionDisplayer, CombatStateDisplayer, TargetMeter};

/* -------------------------------------------------------------------------- */
/*                                UI Resources                                */
/* -------------------------------------------------------------------------- */

/// DOC : new name ? CombatLogAssetsResources
#[derive(Resource)]
pub struct CombatLogResources {
    pub base_log_cave: Handle<Image>,
    pub ladder: Handle<Image>,
}

impl FromWorld for CombatLogResources {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        CombatLogResources {
            base_log_cave: asset_server.load("textures/UI/HUD/combat/log/log_cave.png"),
            ladder: asset_server.load("textures/UI/HUD/combat/log/ladder.png"),
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                                UI Components                               */
/* -------------------------------------------------------------------------- */

#[derive(Component)]
pub struct HUDLog;

/// Points to the UI Text which display Current Action History
#[derive(Component)]
pub struct ActionHistoryDisplayer;

/// Points to the UI Text which display Last Turn Action History
#[derive(Component)]
pub struct LastActionHistoryDisplayer;

/// Points to the UI Text which display Last Turn Actions Precise Logs
#[derive(Component)]
pub struct ActionsLogsDisplayer;

/* -------------------------------------------------------------------------- */
/*                              Enter In the Log                              */
/* -------------------------------------------------------------------------- */

/// REFACTOR: Move to ui::player_interaction ?
pub fn cave_ladder(
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    ladder_query: Query<&Interaction, (Changed<Interaction>, With<Ladder>)>,
) {
    // if let ?
    if let Ok(interaction) = ladder_query.get_single() {
        match interaction {
            Interaction::Clicked => match game_state.0.clone() {
                GameState::CombatWall => {
                    next_state.set(GameState::LogCave);
                }
                GameState::LogCave => {
                    next_state.set(GameState::CombatWall);
                }
                _ => {}
            },
            // TODO: Visual - Hover = outline (see README todos)
            _ => {}
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                                 UI CleanUp                                 */
/* -------------------------------------------------------------------------- */

pub fn cleanup(mut commands: Commands, ui_scene_query: Query<Entity, With<UIScene>>) {
    let ui_scene = ui_scene_query.single();

    commands.entity(ui_scene).despawn_recursive();
}

/* -------------------------------------------------------------------------- */
/*                                  UI Setup                                  */
/* -------------------------------------------------------------------------- */

/// TODO: Must included FightingZone, InitiativeBar
/// IDEA: Spawn all FightingAre, InitiativeBar on startup, only despawn HUDWall or LogCave
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,

    combat_log_resources: Res<CombatLogResources>,
) {
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
            // TODO: Visual - Infinite scroll with the logCave sprite (like in CatDestroyer2000 cinematic)
            parent
                .spawn((
                    ImageBundle {
                        image: combat_log_resources.base_log_cave.clone().into(),
                        style: Style {
                            size: Size::width(Val::Percent(36.)),
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("HUD Log"),
                    HUDLog,
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    flex_shrink: 0.,
                                    size: Size::new(Val::Percent(82.), Val::Percent(100.)),
                                    flex_direction: FlexDirection::Column,
                                    align_self: AlignSelf::Center,
                                    overflow: Overflow::Hidden,
                                    ..default()
                                },
                                // background_color: Color::GRAY.into(),
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
                                        ImageBundle {
                                            image: UiImage {
                                                texture: combat_log_resources.ladder.clone(),
                                                flip_y: true,
                                                ..default()
                                            },
                                            style: Style {
                                                flex_shrink: 0.,
                                                // NOT QUITE RIGHT
                                                size: Size::width(Val::Percent(27.5)),
                                                position: UiRect::left(Val::Percent(-0.5)),
                                                ..default()
                                            },
                                            ..default()
                                        },
                                        Name::new("Upwards Ladder"),
                                        Interaction::default(),
                                        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                        Ladder,
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
                                                size: Size::AUTO,
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
                                                size: Size::AUTO,
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
                                                size: Size::AUTO,
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
                                                size: Size::AUTO,
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
