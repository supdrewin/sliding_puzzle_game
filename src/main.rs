use bevy::{prelude::*, render::camera::WindowOrigin};
use sliding_puzzle_game::{GameMode, GamePlugins, GameState};

fn main() {
    App::new()
        // background color
        .insert_resource(ClearColor(Color::ORANGE))
        // 3x3, 4x4 or None
        .insert_resource(None::<GameMode>)
        // welcome to game
        .add_state(GameState::Start)
        .add_plugins(DefaultPlugins)
        // this is the actual game
        .add_plugins(GamePlugins)
        .add_startup_system(setup_camera)
        .run();
}

// setup both ui camera and 2d camera with origin is bottom left
fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.window_origin = WindowOrigin::BottomLeft;
    commands.spawn_bundle(camera);
}
