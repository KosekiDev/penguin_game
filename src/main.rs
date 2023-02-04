use assets::AssetsPlugin;
use bevy::{app::AppExit, prelude::*};
use bevy_inspector_egui::WorldInspectorPlugin;
use camera::GameCameraPlugin;
use gameover::GameoverPlugin;
use gameplay::GameplayPlugin;
use main_menu::MainMenuPlugin;
use player::{PlayerScore, PlayerStats};

mod assets;
mod camera;
mod defeat_zone;
mod enemy;
mod gameover;
mod gameplay;
mod main_menu;
mod penguins;
mod player;
mod stage;
mod words;

pub const WINDOW_WIDTH: f32 = 512.0;
pub const WINDOW_HEIGHT: f32 = 800.0;
pub const CASE_SIZE: f32 = 64.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    Gameplay,
    GameOver,
}

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::rgb(0.9, 0.92, 0.94)))
        .insert_resource(PlayerScore(0))
        .insert_resource(PlayerStats::default())
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: WINDOW_WIDTH,
                        height: WINDOW_HEIGHT,
                        title: "Penguin typing game".to_owned(),
                        resizable: false,
                        ..default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        );

    if !cfg!(release_assertion) {
        app.add_plugin(WorldInspectorPlugin::new());
    }

    app.add_state(GameState::MainMenu)
        .add_plugin(GameCameraPlugin)
        .add_plugin(AssetsPlugin)
        // Screen plugins
        .add_plugin(MainMenuPlugin)
        .add_plugin(GameplayPlugin)
        .add_plugin(GameoverPlugin)
        .add_system(quit_game); // should spawn another menu for pause the game and quit the game

    app.run();
}

fn quit_game(inputs: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if inputs.just_released(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
