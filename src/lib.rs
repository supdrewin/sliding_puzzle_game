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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Start,
    Menu,
    Game,
}

// 3x3, 4x4 or None
pub struct GameMode(usize);
