use super::{CleanUp, GameState, TextLabel, TextScale};
use crate::animation::AnimationEvent;
use bevy::prelude::*;

#[derive(Component)]
pub struct GameStart;

impl GameStart {
    fn enter(
        mut commands: Commands,
        mut writer: EventWriter<AnimationEvent>,
        server: Res<AssetServer>,
    ) {
        writer.send(AnimationEvent);
        commands
            // whole window context
            .spawn_bundle(NodeBundle {
                style: Style {
                    // build ui from top to bottom
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::ColumnReverse,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    ..Default::default()
                },
                visibility: Visibility { is_visible: false },
                ..Default::default()
            })
            .insert(Self)
            .with_children(|parent| {
                parent
                    // 60% top
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            size: Size::new(Val::Percent(100.0), Val::Percent(60.0)),
                            ..Default::default()
                        },
                        visibility: Visibility { is_visible: false },
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
                                        font: server.load("fonts/VictorMono-BoldItalic.ttf"),
                                        ..Default::default()
                                    },
                                    TextAlignment {
                                        horizontal: HorizontalAlign::Center,
                                        vertical: VerticalAlign::Center,
                                    },
                                ),
                                ..Default::default()
                            })
                            .insert(TextLabel::with_section(TextScale::new(0.1, 0.2)));
                    });
                parent
                    // 40 % bottom
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(50.0), Val::Percent(40.0)),
                            ..Default::default()
                        },
                        visibility: Visibility { is_visible: false },
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
                                    size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .insert(Label)
                            .with_children(|parent| {
                                parent
                                    .spawn_bundle(TextBundle {
                                        text: Text::with_section(
                                            "Start",
                                            TextStyle {
                                                color: Color::OLIVE,
                                                font: server
                                                    .load("fonts/VictorMono-BoldItalic.ttf"),
                                                ..Default::default()
                                            },
                                            Default::default(),
                                        ),
                                        ..Default::default()
                                    })
                                    .insert(TextLabel::with_section(TextScale::new(0.05, 0.1)));
                            });
                    });
            });
    }

    fn update(
        mut state: ResMut<State<GameState>>,
        mut query: Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<Label>)>,
    ) {
        query.for_each_mut(|(interaction, mut color)| match interaction {
            Interaction::Clicked => state.set(GameState::Menu).unwrap(),
            Interaction::Hovered => *color = Color::GOLD.into(),
            Interaction::None => *color = Color::YELLOW.into(),
        });
    }
}

impl CleanUp<Self> for GameStart {}

impl Plugin for GameStart {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Start).with_system(Self::enter))
            .add_system_set(SystemSet::on_update(GameState::Start).with_system(Self::update))
            .add_system_set(SystemSet::on_exit(GameState::Start).with_system(Self::exit));
    }
}

#[derive(Component)]
struct Label;
