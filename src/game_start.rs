use super::GameState;
use bevy::prelude::*;

#[derive(Component)]
pub struct GameStart;

impl GameStart {
    fn enter(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands
            // whole window context
            .spawn_bundle(NodeBundle {
                color: Color::NONE.into(),
                style: Style {
                    // build ui from top to bottom
                    flex_direction: FlexDirection::ColumnReverse,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|parent| {
                parent
                    // 60% top
                    .spawn_bundle(NodeBundle {
                        color: Color::NONE.into(),
                        style: Style {
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            size: Size::new(Val::Percent(100.0), Val::Percent(60.0)),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent
                            // our honor title
                            .spawn_bundle(TextBundle {
                                text: Text::with_section(
                                    "Sliding Puzzle Game",
                                    TextStyle {
                                        color: Color::NAVY,
                                        font: asset_server.load("fonts/VictorMono-BoldItalic.ttf"),
                                        font_size: 128.0,
                                    },
                                    TextAlignment {
                                        horizontal: HorizontalAlign::Center,
                                        vertical: VerticalAlign::Center,
                                    },
                                ),
                                ..Default::default()
                            })
                            .insert(GameStart);
                    })
                    .insert(GameStart);
                parent
                    // 40 % bottom
                    .spawn_bundle(NodeBundle {
                        color: Color::NONE.into(),
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Percent(40.0)),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent
                            // a continue button
                            .spawn_bundle(ButtonBundle {
                                style: Style {
                                    align_items: AlignItems::Center,
                                    margin: Rect::all(Val::Auto),
                                    justify_content: JustifyContent::Center,
                                    size: Size::new(Val::Percent(50.0), Val::Percent(50.0)),
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                parent
                                    .spawn_bundle(TextBundle {
                                        text: Text::with_section(
                                            "Start",
                                            TextStyle {
                                                color: Color::OLIVE,
                                                font: asset_server
                                                    .load("fonts/VictorMono-BoldItalic.ttf"),
                                                font_size: 64.0,
                                            },
                                            Default::default(),
                                        ),
                                        ..Default::default()
                                    })
                                    .insert(GameStart);
                            })
                            .insert(GameStart);
                    })
                    .insert(GameStart);
            })
            .insert(GameStart);
    }

    fn update(
        mut state: ResMut<State<GameState>>,
        mut query: Query<
            (&Interaction, &mut UiColor),
            (Changed<Interaction>, With<Button>, With<GameStart>),
        >,
    ) {
        query.for_each_mut(|(interaction, mut color)| match interaction {
            Interaction::Clicked => state.set(GameState::Menu).unwrap(),
            Interaction::Hovered => *color = Color::GOLD.into(),
            Interaction::None => *color = Color::YELLOW.into(),
        });
    }

    // despawn all entity current state when exit
    fn exit(mut commands: Commands, query: Query<Entity, With<GameStart>>) {
        query.for_each(|entity| commands.entity(entity).despawn());
    }
}

impl Plugin for GameStart {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Start).with_system(GameStart::enter))
            .add_system_set(SystemSet::on_update(GameState::Start).with_system(GameStart::update))
            .add_system_set(SystemSet::on_exit(GameState::Start).with_system(GameStart::exit));
    }
}