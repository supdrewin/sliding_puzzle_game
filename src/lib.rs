mod floating;
mod game;
mod game_menu;
mod game_start;

use bevy::{app::PluginGroupBuilder, prelude::*};
use floating::Floating;
use game::Game;
use game_menu::GameMenu;
use game_start::GameStart;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(&mut self, builder: &mut PluginGroupBuilder) {
        builder.add(Floating).add(GameStart).add(GameMenu).add(Game);
    }
}

pub trait CleanUp<T: Component> {
    // despawn all entity current state marked when exit
    fn exit(mut commands: Commands, query: Query<Entity, With<T>>) {
        query.for_each(|entity| commands.entity(entity).despawn_recursive());
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

// 3x3, 4x4 or None
pub struct GameMode(usize);
