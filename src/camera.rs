use bevy::prelude::*;

#[derive(Component)]
pub struct GameCamera;

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(GameCamera)
        .insert(Name::new("Camera"))
        .insert(VisibilityBundle::default());
}
