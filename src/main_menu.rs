use bevy::prelude::*;

use crate::GameState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, draw_main_menu)
            .add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(start_gameplay));
    }
}

fn draw_main_menu() {
    println!("Draw the main menu");
}

fn start_gameplay(mut state: ResMut<State<GameState>>) {
    state.set(GameState::Gameplay).unwrap();
}
