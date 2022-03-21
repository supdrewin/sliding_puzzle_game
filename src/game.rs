use super::{GameMode, GameState};
use bevy::{prelude::*, utils::HashMap};

#[derive(Component)]
pub struct Game;

impl Game {
    fn enter(
        mut commands: Commands,
        mut board: ResMut<Board>,
        windows: Res<Windows>,
        game_mode: Res<Option<GameMode>>,
        asset_server: Res<AssetServer>,
    ) {
        let window = windows.get_primary().unwrap();
        board.2 = window.width().min(window.height());
        board.4 = game_mode.as_ref().as_ref().unwrap().0;
        board.3 = board.2 / board.4 as f32;
        board.0 = vec![Position::default(); board.4.pow(2)];
        let scale = board.3 / 128.0;
        let mut num = 1;
        for y in 0..board.4 {
            for x in 0..board.4 {
                board.1.insert(Position { x, y }, num);
                board.0[num] = Position { x, y };
                commands
                    .spawn_bundle(SpriteBundle {
                        texture: asset_server.load(&format!("sliders/{num}.png")),
                        transform: Transform {
                            scale: Vec3::new(scale, scale, 1.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Slider { num })
                    .insert(Self);
                num += 1;
                num %= board.0.len();
            }
        }
    }

    fn mouse_system(
        mouse_button: Res<Input<MouseButton>>,
        windows: Res<Windows>,
        mut board: ResMut<Board>,
    ) {
        // move following mouse
        if mouse_button.pressed(MouseButton::Left) {
            if let Some(window) = windows.get_primary() {
                if let Some(pos) = window.cursor_position() {
                    let origin = Position::new(
                        (pos.x / board.3) as usize,
                        ((board.2 - pos.y) / board.3) as usize,
                    );
                    if let Some(&index) = board.1.get(&origin) {
                        let mut direct = vec![
                            Position::new(origin.x + 1, origin.y),
                            Position::new(origin.x, origin.y + 1),
                        ];
                        if origin.x > 0 {
                            direct.push(Position::new(origin.x - 1, origin.y));
                        }
                        if origin.y > 0 {
                            direct.push(Position::new(origin.x, origin.y - 1));
                        }
                        for pos in direct {
                            if let Some(0) = board.1.get(&pos) {
                                *board.1.get_mut(&pos).unwrap() = index;
                                *board.1.get_mut(&origin).unwrap() = 0;
                                board.0.swap(0, index);
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
            let pos = board.0[0];
            let bound = board.4 - 1;
            let origin = match code {
                KeyCode::Up | KeyCode::W if pos.y < bound => Position::new(pos.x, pos.y + 1),
                KeyCode::Left | KeyCode::A if pos.x < bound => Position::new(pos.x + 1, pos.y),
                KeyCode::Right | KeyCode::D if pos.x > 0 => Position::new(pos.x - 1, pos.y),
                KeyCode::Down | KeyCode::S if pos.y > 0 => Position::new(pos.x, pos.y - 1),
                // ignore any other key
                _ => return,
            };
            let index = board.1[&origin];
            *board.1.get_mut(&pos).unwrap() = index;
            *board.1.get_mut(&origin).unwrap() = 0;
            board.0.swap(0, index);
        });
    }

    fn update(board: Res<Board>, mut query: Query<(&Slider, &mut Transform), With<Slider>>) {
        if board.is_changed() {
            query.for_each_mut(|(slider, mut transform)| {
                let Position { x, y } = board.0[slider.num];
                transform.translation.x = board.3 * (x as f32 + 0.5);
                transform.translation.y = board.2 - board.3 * (y as f32 + 0.5);
            });
        }
    }

    // despawn all entity current state when exit
    fn exit(mut commands: Commands, query: Query<Entity, With<Self>>) {
        query.for_each(|entity| commands.entity(entity).despawn());
    }
}

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.insert_resource(Board::default())
            .add_system_set(SystemSet::on_enter(GameState::Game).with_system(Self::enter))
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
#[derive(Component, Default)] // 2: board size 3: slider size 4: level
struct Board(Vec<Position>, HashMap<Position, usize>, f32, f32, usize);

// setup a position structure for hash
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    // create a new position
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
