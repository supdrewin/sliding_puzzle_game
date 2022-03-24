use super::{Game, GameState};
use crate::{TextLabel, TextScale};
use bevy::prelude::*;

pub fn setup(mut commands: Commands, server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                position: Rect {
                    // right top
                    right: Val::Percent(2.0),
                    top: Val::Percent(2.0),
                    ..Default::default()
                },
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        // as a part of game
        .insert(Game)
        .with_children(|parent| {
            [(Label::Reset, "Reset"), (Label::Back, "Back")]
                .into_iter()
                .for_each(|(label, text)| {
                    parent // space item
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size {
                                    height: Val::Px(16.0),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            visibility: Visibility { is_visible: false },
                            ..Default::default()
                        });
                    parent // button
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                padding: Rect::all(Val::Percent(1.0)),
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
                                .insert(TextLabel::with_section(TextScale::new(0.04, 0.08)));
                        });
                });
        });
}

pub fn interaction(
    mut writer: EventWriter<ShouldBeRestored>,
    mut state: ResMut<State<GameState>>,
    mut query: Query<(&Interaction, &Label, &mut UiColor), (Changed<Interaction>, With<Label>)>,
) {
    query.for_each_mut(|(interaction, label, mut color)| match interaction {
        Interaction::Clicked => match label {
            Label::Reset => writer.send(ShouldBeRestored),
            Label::Back => state.set(GameState::Menu).unwrap(),
        },
        Interaction::Hovered => *color = Color::GOLD.into(),
        Interaction::None => *color = Color::YELLOW.into(),
    });
}

#[repr(transparent)]
pub struct ShouldBeRestored;

// buttons
#[derive(Component)]
pub enum Label {
    Reset,
    Back,
}
