use bevy::prelude::*;

use crate::{assets::EntitiesAtlas, enemy::Enemy, GameState, CASE_SIZE, WINDOW_HEIGHT};

pub const DEFEAT_ZONE_HEIGHT: f32 = 85.0;

#[derive(Component)]
pub struct DefeatZone;

pub struct DefeatZonePlugin;

impl Plugin for DefeatZonePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_defeat_zone)
            .add_system_set(SystemSet::on_update(GameState::Gameplay).with_system(gameover));
    }
}

fn spawn_defeat_zone(mut commands: Commands, atlases: Res<EntitiesAtlas>) {
    let mut sprite = TextureAtlasSprite::new(atlases.defeat_zone);
    sprite.custom_size = Some(Vec2::new(CASE_SIZE * 8.0, 85.0));

    // Penguin Store
    commands
        .spawn(SpriteSheetBundle {
            sprite,
            texture_atlas: atlases.texture_atlas.clone(),
            transform: Transform::from_translation(Vec3::new(
                0.0,
                -(WINDOW_HEIGHT / 2.0 - DEFEAT_ZONE_HEIGHT / 2.0),
                1.0,
            )),
            ..default()
        })
        .insert(DefeatZone);
}

fn gameover(
    enemies: Query<&Transform, With<Enemy>>,
    defeat_zone: Query<&Transform, (With<DefeatZone>, Without<Enemy>)>,
    mut state: ResMut<State<GameState>>,
) {
    let defeat_zone = defeat_zone.single();

    for enemy in enemies.iter() {
        let offset = enemy.translation.y - defeat_zone.translation.y;
        if offset.abs() <= DEFEAT_ZONE_HEIGHT / 2.0 {
            state.set(GameState::GameOver).unwrap();
            return;
        }
    }
}
