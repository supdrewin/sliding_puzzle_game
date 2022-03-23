use bevy::prelude::*;

pub struct Floating;

impl Floating {
    fn setup(mut commands: Commands, server: Res<AssetServer>) {
        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::ColumnReverse,
                    position: Rect {
                        // bottom right
                        bottom: Val::Px(16.0),
                        right: Val::Px(16.0),
                        ..Default::default()
                    },
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Px(32.0), Val::Px(96.0)),
                    ..Default::default()
                },
                visibility: Visibility { is_visible: false },
                ..Default::default()
            })
            .with_children(|parent| {
                // add help and info
                [(Label::Help, 'H'), (Label::Info, 'i')]
                    .into_iter()
                    .for_each(|(label, text)| {
                        parent
                            .spawn_bundle(ButtonBundle {
                                style: Style {
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    margin: Rect::all(Val::Auto),
                                    size: Size::new(Val::Px(32.0), Val::Px(32.0)),
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .insert(label)
                            .with_children(|parent| {
                                parent.spawn_bundle(TextBundle {
                                    text: Text::with_section(
                                        text,
                                        TextStyle {
                                            color: Color::BLACK,
                                            font: server.load("fonts/VictorMono-BoldItalic.ttf"),
                                            font_size: 32.0,
                                        },
                                        Default::default(),
                                    ),
                                    ..Default::default()
                                });
                            });
                    });
            });
    }

    fn info_system() {
        // pop-up box
    }

    fn help_system() {
        // pop-up box
    }

    fn update(
        server: Res<AssetServer>,
        mut query: Query<(&Interaction, &Label, &mut UiImage), (Changed<Interaction>, With<Label>)>,
    ) {
        query.for_each_mut(|(interaction, label, mut image)| match interaction {
            Interaction::None => *image = server.load("images/button_blue.png").into(),
            Interaction::Hovered => *image = server.load("images/button_blue_hovered.png").into(),
            Interaction::Clicked => match label {
                Label::Help => Self::help_system(),
                Label::Info => Self::info_system(),
            },
        });
    }
}

impl Plugin for Floating {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::setup).add_system(Self::update);
    }
}

#[derive(Component)]
enum Label {
    Help,
    Info,
}
