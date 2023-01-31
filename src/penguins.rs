use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;

use crate::{assets::EntitiesAtlas, enemy::Enemy, GameState, CASE_SIZE, WINDOW_HEIGHT};

pub const PENGUIN_THROW_ORIGIN_Y: f32 = -WINDOW_HEIGHT / 2.0 + CASE_SIZE * 1.75;

#[derive(Component)]
pub struct PenguinIdle;

#[derive(Component)]
pub struct PenguinIdleAnimated {
    pub timer: Timer,
    pub animate: bool,
} // must be unique

#[derive(Component)]
pub struct FishThrowed {
    pub target: Entity,
    pub target_position: Vec2,
}

pub struct PenguinPlugin;
impl Plugin for PenguinPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::Startup, spawn_penguin)
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay).with_system(fish_throwed_animate),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(animate_penguin_when_throwing_fish),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay).with_system(fish_and_enemy_collision),
            );
    }
}

fn spawn_penguin(mut commands: Commands, atlases: Res<EntitiesAtlas>) {
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: atlases.penguin_fired_throwed.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, -(WINDOW_HEIGHT / 2.0 - CASE_SIZE * 0.75), 1.5),
                ..default()
            },
            ..default()
        })
        .insert(PenguinIdleAnimated {
            timer: Timer::new(Duration::from_millis(30), TimerMode::Repeating),
            animate: false,
        })
        .insert(Name::new("Penguin"));
}

fn fish_throwed_animate(
    time: Res<Time>,
    textures: Res<EntitiesAtlas>,
    mut fish_throwed: Query<(&mut Transform, &FishThrowed, &mut TextureAtlasSprite)>,
) {
    for (mut penguin, target, mut sprite) in fish_throwed.iter_mut() {
        let tpos = target.target_position;
        let m = (tpos.y - PENGUIN_THROW_ORIGIN_Y) / tpos.x;

        let p = PENGUIN_THROW_ORIGIN_Y;
        penguin.translation.y += CASE_SIZE * time.delta().as_millis() as f32 / 60.0;

        penguin.translation.x = (penguin.translation.y - p) / m;
        penguin.rotation =
            Quat::from_rotation_z(m.atan() + 1.5 * PI + if m < 0.0 { PI } else { 0.0 });

        sprite.index = textures.fish;
    }
}

fn fish_and_enemy_collision(
    mut commands: Commands,
    fishes: Query<(Entity, &Transform, &FishThrowed)>,
    mut enemies: Query<&mut Enemy, Without<FishThrowed>>,
) {
    for (entity, transform, target) in fishes.iter() {
        if transform.translation.y >= target.target_position.y {
            let Ok(mut enemy) = enemies.get_mut(target.target) else {
                continue;
            };

            enemy.life -= 1;
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn animate_penguin_when_throwing_fish(
    time: Res<Time>,
    texture: Res<Assets<TextureAtlas>>,
    mut penguin: Query<(
        &mut TextureAtlasSprite,
        &mut PenguinIdleAnimated,
        &Handle<TextureAtlas>,
    )>,
) {
    if penguin.is_empty() {
        return;
    }

    let (mut sprite, mut animation, handle_texture) = penguin.single_mut();

    if !animation.animate {
        return;
    }

    animation.timer.tick(time.delta());
    let texture_atlas = texture.get(handle_texture).unwrap();

    if animation.timer.just_finished() {
        sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
    }

    if sprite.index == texture_atlas.textures.len() - 1 {
        sprite.index = 0;
        animation.timer.reset();
        animation.animate = false;
    }
}
