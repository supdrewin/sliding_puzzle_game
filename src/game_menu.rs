use super::{GameMode, GameState};
use bevy::prelude::*;

#[derive(Component)]
pub struct GameMenu;

impl GameMenu {
    fn enter(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands
            // root node
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
                [
                    // build each botton
                    (Label::Mode3x3, "3x3"),
                    (Label::Mode4x4, "4x4"),
                    (Label::Back, "Back"),
                ]
                .into_iter()
                .for_each(|(mode, text)| {
                    parent
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Percent(50.0), Val::Percent(20.0)),
                                margin: Rect::all(Val::Auto),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(TextBundle {
                                    text: Text::with_section(
                                        text,
                                        TextStyle {
                                            color: Color::OLIVE,
                                            font: asset_server
                                                .load("fonts/VictorMono-BoldItalic.ttf"),
                                            font_size: 32.0,
                                        },
                                        Default::default(),
                                    ),
                                    ..Default::default()
                                })
                                .insert(GameMenu);
                        })
                        .insert(GameMenu)
                        .insert(mode);
                });
            })
            .insert(GameMenu);
    }

    fn update(
        mut game_state: ResMut<State<GameState>>,
        mut game_mode: ResMut<Option<GameMode>>,
        mut query: Query<
            (&Label, &Interaction, &mut UiColor),
            (Changed<Interaction>, With<Button>, With<GameMenu>),
        >,
    ) {
        query.for_each_mut(|(label, interaction, mut color)| match interaction {
            Interaction::Clicked => {
                let (state, mode) = match label {
                    Label::Mode3x3 => (GameState::Game, Some(GameMode(3))),
                    Label::Mode4x4 => (GameState::Game, Some(GameMode(4))),
                    Label::Back => (GameState::Start, None),
                };
                // set game state
                game_state.set(state).unwrap();
                // set game mode
                *game_mode = mode;
            }
            Interaction::Hovered => *color = Color::GOLD.into(),
            Interaction::None => *color = Color::YELLOW.into(),
        });
    }

    // despawn all entity current state when exit
    fn exit(mut commands: Commands, query: Query<Entity, With<GameMenu>>) {
        query.for_each(|entity| commands.entity(entity).despawn());
    }
}

impl Plugin for GameMenu {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Menu).with_system(GameMenu::enter))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(GameMenu::update))
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(GameMenu::exit));
    }
}

// labels for each botton
#[derive(Component)]
enum Label {
    Mode3x3,
    Mode4x4,
    Back,
}
