use super::{CleanUp, GameMode, GameState, TextLabel, TextScale};
use crate::animation::AnimationEvent;

use bevy::prelude::*;

#[derive(Component)]
pub struct GameMenu;

impl GameMenu {
    fn enter(
        mut commands: Commands,
        mut writer: EventWriter<AnimationEvent>,
        server: Res<AssetServer>,
    ) {
        writer.send(AnimationEvent);
        commands
            // root node
            .spawn_bundle(NodeBundle {
                style: Style {
                    // build ui from top to bottom
                    flex_direction: FlexDirection::ColumnReverse,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    ..Default::default()
                },
                visibility: Visibility { is_visible: false },
                ..Default::default()
            })
            .insert(Self)
            .with_children(|parent| {
                [
                    // build each botton
                    (Label::Mode3x3, "3x3"),
                    (Label::Mode4x4, "4x4"),
                    (Label::Back, "Back"),
                ]
                .into_iter()
                .for_each(|(label, text)| {
                    parent
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                margin: Rect::all(Val::Auto),
                                size: Size::new(Val::Percent(50.0), Val::Percent(20.0)),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(label)
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(TextBundle {
                                    text: Text::with_section(
                                        text,
                                        TextStyle {
                                            color: Color::OLIVE,
                                            font: server.load("fonts/VictorMono-BoldItalic.ttf"),
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
        mut commands: Commands,
        mut state: ResMut<State<GameState>>,
        mut query: Query<(&Interaction, &Label, &mut UiColor), (Changed<Interaction>, With<Label>)>,
    ) {
        query.for_each_mut(|(interaction, label, mut color)| match interaction {
            Interaction::Clicked => {
                // set game state
                state
                    .set(match label {
                        Label::Mode3x3 => {
                            commands.insert_resource(GameMode(3));
                            GameState::Game
                        }
                        Label::Mode4x4 => {
                            commands.insert_resource(GameMode(4));
                            GameState::Game
                        }
                        Label::Back => GameState::default(),
                    })
                    .unwrap();
            }
            Interaction::Hovered => *color = Color::GOLD.into(),
            Interaction::None => *color = Color::YELLOW.into(),
        });
    }
}

impl CleanUp<Self> for GameMenu {}

impl Plugin for GameMenu {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Menu).with_system(Self::enter))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(Self::update))
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(Self::exit));
    }
}

// labels for each botton
#[derive(Component)]
enum Label {
    Mode3x3,
    Mode4x4,
    Back,
}
