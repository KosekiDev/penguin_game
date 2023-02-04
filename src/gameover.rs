use bevy::prelude::*;

use crate::GameState;

pub struct GameoverPlugin;

impl Plugin for GameoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(spawn_game_over));
    }
}

fn spawn_game_over() {
    println!("GameOver !!!");
}
