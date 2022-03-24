mod animation;
mod floating;
mod game;
mod game_menu;
mod game_start;

use animation::Animation;
use bevy::{app::PluginGroupBuilder, prelude::*};
use floating::Floating;
use game::Game;
use game_menu::GameMenu;
use game_start::GameStart;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(&mut self, builder: &mut PluginGroupBuilder) {
        builder
            .add(Animation)
            .add(Floating)
            .add(GameStart)
            .add(GameMenu)
            .add(Game);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Start,
    Menu,
    Game,
}

impl Default for GameState {
    fn default() -> Self {
        Self::Start
    }
}

#[derive(Component)]
pub struct TextLabel {
    pub scales: Vec<TextScale>,
}

impl TextLabel {
    fn with_section(section: TextScale) -> Self {
        Self {
            scales: vec![section],
        }
    }
}

#[derive(Component)]
pub struct TextScale {
    pub height: f32,
    pub width: f32,
}

impl TextScale {
    fn new(width: f32, height: f32) -> Self {
        Self { height, width }
    }
}

// 3x3, 4x4 or None
struct GameMode(usize);

trait CleanUp<T: Component> {
    // despawn all entity current state marked when exit
    fn exit(mut commands: Commands, query: Query<Entity, With<T>>) {
        query.for_each(|entity| commands.entity(entity).despawn_recursive());
    }
}
