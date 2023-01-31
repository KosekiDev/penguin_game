use bevy::prelude::*;

pub struct GameoverPlugin;

impl Plugin for GameoverPlugin {
    fn build(&self, app: &mut App) {
        println!("game over plugin")
    }
}
