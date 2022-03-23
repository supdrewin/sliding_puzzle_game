use bevy::{
    input::system::exit_on_esc_system as exit_on_esc, prelude::*, render::camera::WindowOrigin,
    window::exit_on_window_close_system as exit_on_close,
};
use sliding_puzzle_game::{GamePlugins, GameState};

fn main() {
    App::new()
        // background color
        .insert_resource(ClearColor(Color::ORANGE))
        .add_plugins(DefaultPlugins)
        // this is the actual game
        .add_plugins(GamePlugins)
        // welcome to game
        .add_state(GameState::default())
        // show our ui and so on
        .add_startup_system(setup_camera)
        // window close don't app exit default
        .add_system(exit_on_close)
        // exit on esc pressed
        .add_system(exit_on_esc)
        .run();
}

// setup both ui camera and 2d camera with origin is bottom left
fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.window_origin = WindowOrigin::BottomLeft;
    commands.spawn_bundle(camera);
}
