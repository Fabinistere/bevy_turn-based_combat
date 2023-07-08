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
    constants::ui::style::*,
    ui::{
        combat_panel::{Ladder, UIScene},
        player_interaction::ScrollingList,
    },
};

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

/// The Fighting Hall and Initiative Bar are preserved
pub fn cleanup(mut commands: Commands, log_cave_query: Query<Entity, With<HUDLog>>) {
    let log_cave = log_cave_query.single();

    commands.entity(log_cave).despawn_recursive();
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
    ui_scene_query: Query<Entity, With<UIScene>>,
) {
    let ui_scene = ui_scene_query.single();
    commands.entity(ui_scene).with_children(|parent| {
        /* -------------------------------------------------------------------------- */
        /*                                  LOG Cave                                  */
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
                                        format!("---------------\nCurrent Turn Actions:"),
                                        get_text_style(&asset_server, 20.),
                                    )
                                    .with_style(Style {
                                        flex_wrap: FlexWrap::Wrap,
                                        // flex_shrink: 0.,
                                        size: Size::AUTO,
                                        // margin: UiRect {
                                        //     left: Val::Auto,
                                        //     right: Val::Auto,
                                        //     ..default()
                                        // },
                                        ..default()
                                    }),
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
                                    .with_style(Style {
                                        flex_wrap: FlexWrap::Wrap,
                                        // flex_shrink: 0.,
                                        size: Size::AUTO,
                                        // margin: UiRect {
                                        //     left: Val::Auto,
                                        //     right: Val::Auto,
                                        //     ..default()
                                        // },
                                        ..default()
                                    }),
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
                                    .with_style(Style {
                                        flex_wrap: FlexWrap::Wrap,
                                        // flex_shrink: 0.,
                                        size: Size::AUTO,
                                        // margin: UiRect {
                                        //     left: Val::Auto,
                                        //     right: Val::Auto,
                                        //     ..default()
                                        // },
                                        ..default()
                                    }),
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
                                    .with_style(Style {
                                        flex_wrap: FlexWrap::Wrap,
                                        size: Size::AUTO,
                                        ..default()
                                    }),
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
