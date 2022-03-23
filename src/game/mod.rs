use super::{CleanUp, GameMode, GameState};
use bevy::{prelude::*, utils::HashMap};

#[derive(Component)]
pub struct Game;

impl Game {
    fn enter(
        mut commands: Commands,
        mode: Res<GameMode>,
        windows: Res<Windows>,
        server: Res<AssetServer>,
    ) {
        if let Some(window) = windows.get_primary() {
            let standard = window.width().min(window.height());
            let mut board = Board {
                offset: standard * 0.1,
                size: standard * 0.8,
                slider_map: vec![Position::default(); mode.0.pow(2)],
                position_map: HashMap::default(),
                ..Default::default()
            };
            board.slider_size = board.size / mode.0 as f32;
            let scale = board.slider_size / 128.0;
            (0..mode.0)
                .flat_map(|x| (0..mode.0).map(move |y| (x, y)))
                .zip((0..board.slider_map.len()).cycle().skip(1))
                .for_each(|((y, x), num)| {
                    let pos = Position::from((x, y));
                    board.slider_map[num] = pos;
                    board.position_map.insert(pos, num);
                    commands
                        // slider image
                        .spawn_bundle(SpriteBundle {
                            texture: server.load("images/slider_default.png"),
                            transform: Transform {
                                scale: Vec3::new(scale, scale, 1.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(Slider { num })
                        .insert(Self)
                        .with_children(|parent| {
                            parent
                                // slider number
                                .spawn_bundle(Text2dBundle {
                                    text: Text::with_section(
                                        num.to_string(),
                                        TextStyle {
                                            color: Color::YELLOW,
                                            font: server.load("fonts/VictorMono-Bold.ttf"),
                                            font_size: board.slider_size / 2.0,
                                        },
                                        TextAlignment {
                                            horizontal: HorizontalAlign::Center,
                                            vertical: VerticalAlign::Center,
                                        },
                                    ),
                                    transform: Transform {
                                        translation: Vec3::new(0.0, 0.0, 1.0),
                                        ..Default::default()
                                    },
                                    visibility: Visibility {
                                        is_visible: num != 0,
                                    },
                                    ..Default::default()
                                });
                        });
                });
            commands.insert_resource(board);
        }
    }

    fn mouse_system(
        mouse: Res<Input<MouseButton>>,
        windows: Res<Windows>,
        mut board: ResMut<Board>,
    ) {
        // move following mouse
        if mouse.pressed(MouseButton::Left) {
            if let Some(window) = windows.get_primary() {
                if let Some(pos) = window.cursor_position() {
                    let origin = Position::from((
                        (pos.x - board.offset) / board.slider_size,
                        (board.size + board.offset - pos.y) / board.slider_size,
                    ));
                    if let Some(&index) = board.position_map.get(&origin) {
                        for pos in [
                            Position::new(origin.x + 1, origin.y),
                            Position::new(origin.x, origin.y + 1),
                            Position::new(origin.x - 1, origin.y),
                            Position::new(origin.x, origin.y - 1),
                        ] {
                            if let Some(0) = board.position_map.get(&pos) {
                                *board.position_map.get_mut(&pos).unwrap() = index;
                                *board.position_map.get_mut(&origin).unwrap() = 0;
                                board.slider_map.swap(0, index);
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    fn keyboard_system(keyboard: Res<Input<KeyCode>>, mut board: ResMut<Board>) {
        keyboard.get_just_released().for_each(|code| {
            let pos = board.slider_map[0];
            let origin = match code {
                KeyCode::Left | KeyCode::A => Position::new(pos.x + 1, pos.y),
                KeyCode::Right | KeyCode::D => Position::new(pos.x - 1, pos.y),
                KeyCode::Down | KeyCode::S => Position::new(pos.x, pos.y - 1),
                KeyCode::Up | KeyCode::W => Position::new(pos.x, pos.y + 1),
                // ignore any other key
                _ => return,
            };
            if let Some(&index) = board.position_map.get(&origin) {
                *board.position_map.get_mut(&pos).unwrap() = index;
                *board.position_map.get_mut(&origin).unwrap() = 0;
                board.slider_map.swap(0, index);
            }
        });
    }

    fn update(board: Res<Board>, mut query: Query<(&Slider, &mut Transform), With<Slider>>) {
        if board.is_changed() {
            query.for_each_mut(|(slider, mut transform)| {
                let Position { x, y } = board.slider_map[slider.num];
                transform.translation = Vec3::new(
                    board.slider_size * (x as f32 + 0.5) + board.offset,
                    board.size - board.slider_size * (y as f32 + 0.5) + board.offset,
                    0.0,
                );
            });
        }
    }
}

impl CleanUp<Self> for Game {}

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(Self::enter))
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    // add mouse support
                    .with_system(Self::mouse_system)
                    // add keyboard support
                    .with_system(Self::keyboard_system)
                    .with_system(Self::update),
            )
            .add_system_set(SystemSet::on_exit(GameState::Game).with_system(Self::exit));
    }
}

#[derive(Component)]
struct Slider {
    // slider's number
    num: usize,
}

// TODO: update board after window resized.
#[derive(Component, Default)]
struct Board {
    offset: f32,
    size: f32,
    slider_size: f32,
    slider_map: Vec<Position>,
    position_map: HashMap<Position, usize>,
}

// setup a position structure for hash
// using isize to avoid overflow
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    // create a new position
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl From<(usize, usize)> for Position {
    fn from(pos: (usize, usize)) -> Self {
        Self {
            x: pos.0 as isize,
            y: pos.1 as isize,
        }
    }
}

impl From<(f32, f32)> for Position {
    fn from(pos: (f32, f32)) -> Self {
        Self {
            x: pos.0 as isize,
            y: pos.1 as isize,
        }
    }
}
