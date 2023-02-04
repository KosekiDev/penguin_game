use bevy_inspector_egui::Inspectable;
use rand::{self, Rng};
use std::time::Duration;

use bevy::prelude::*;

use crate::{
    assets::{EntitiesAtlas, FontsAtlas},
    penguins::{FishThrowed, PenguinIdleAnimated, PENGUIN_THROW_ORIGIN_Y},
    player::{PlayerScore, PlayerStats},
    stage::StageComponent,
    words::{TextEnemy, WordsResource},
    GameState, CASE_SIZE, WINDOW_HEIGHT, WINDOW_WIDTH,
};

pub const BASE_SPEED: f32 = CASE_SIZE * 0.8 / 60.0; // in px per seconds
pub const ENEMY_GENERATION_TIME_INTERVAL: (f64, f64) = (4.0, 7.0);
pub const SPAWN_TIME_INTERVAL: u64 = 2000;
pub const MIN_SPAWN_TIME_INTERVAL: u64 = 500;
pub const BLOOD_CLEAR_DELAY: f32 = 4000.0; // in ms

#[derive(Component)]
pub struct Fish;

#[derive(Component)]
pub struct Blood {
    pub animation_timer: Timer,
    pub clear_timer: Timer,
}

#[derive(PartialEq, Default, Inspectable, Reflect)]
pub enum EnemyState {
    #[default]
    Walk,
    Idle,
}

#[derive(Component, Default, Inspectable, Reflect)]
#[reflect(Component)]
pub struct Enemy {
    pub life: usize,
    pub state: EnemyState,
    pub velocity: f32,
    pub points: u32,
}

#[derive(Component)]
pub struct EnemyGenitor {
    pub enemy_type: EnemyGeneratedType,
    pub last_generation: f64,
}

pub enum EnemyGeneratedType {
    LittleSpider,
}

#[derive(Component)]
pub struct Target;

#[derive(Component)]
pub struct GeneratedEnemy; // entity generated by other entity

#[derive(Component)]
pub struct EnemyAnimated {
    pub timer: Timer,
}

#[derive(Component)]
pub struct MakeChilds;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, key_pressed.label("key_pressed"))
            .register_type::<Enemy>()
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(animate_enemy)
                    .with_system(enemy_walk)
                    .with_system(generate_enemy)
                    .with_system(spawn_entity)
                    .with_system(despawn_enemies.after("give_point_when_dead"))
                    .with_system(clear_dead_enemy_blood)
                    .with_system(animate_dead_enemy_blood)
                    .with_system(give_point_when_dead.label("give_point_when_dead")),
            );
    }
}

fn animate_enemy(
    time: Res<Time>,
    texture: Res<Assets<TextureAtlas>>,
    mut enemy: Query<(
        &mut TextureAtlasSprite,
        &mut EnemyAnimated,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut sprite, mut enemy, atlas) in enemy.iter_mut() {
        enemy.timer.tick(time.delta());

        if enemy.timer.just_finished() {
            let texture_atlas = texture.get(atlas).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

fn enemy_walk(time: Res<Time>, mut enemys: Query<(&Enemy, &mut Transform)>) {
    for (enemy, mut transform) in enemys.iter_mut() {
        if enemy.state == EnemyState::Walk {
            transform.translation.y -=
                enemy.velocity * BASE_SPEED * time.delta().as_millis() as f32 / 60.0;
        }
    }
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
fn key_pressed(
    mut commands: Commands,
    mut inputs: EventReader<ReceivedCharacter>,
    mut enemy: Query<(Entity, &mut Enemy, &Children, &GlobalTransform), With<Target>>,
    mut enemies: Query<(Entity, &mut Enemy, &Children, &GlobalTransform), Without<Target>>,
    mut texts: Query<(&Parent, &GlobalTransform, &mut Text), (Without<Target>, With<TextEnemy>)>,
    atlases: Res<EntitiesAtlas>,
    mut stage: Query<&mut StageComponent>,
    mut penguin: Query<&mut PenguinIdleAnimated, Without<Target>>,
    mut player_stats: ResMut<PlayerStats>,
) {
    if let Some(key) = inputs.iter().next() {
        let mut target: Option<(Entity, Mut<Enemy>, &Children, &GlobalTransform)> = None;
        let mut stage = stage.single_mut();

        let mut miss = || {
            stage.decrease_bonus();
            player_stats.combos_count = 0;
            player_stats.misses += 1;
        };

        if enemy.is_empty() {
            // check target
            let mut sort = texts
                .iter_mut()
                .collect::<Vec<(&Parent, &GlobalTransform, Mut<Text>)>>();

            sort.sort_by(|a, b| {
                a.1.translation()
                    .y
                    .partial_cmp(&b.1.translation().y)
                    .unwrap_or(std::cmp::Ordering::Less)
            });

            let mut target_found = false;

            for (parent, _, text) in sort.iter_mut() {
                if text.sections[0].value.starts_with(key.char.to_owned()) {
                    target = Some(enemies.get_mut(parent.get()).unwrap());

                    text.sections[0].style.font_size *= 1.25;
                    text.sections[0].style.color = Color::RED;

                    commands.entity(parent.get()).insert(Target);

                    target_found = true;
                    break;
                }
            }
            // if no target selected, the player missed. Decrease the bonus
            if !target_found {
                miss();
            }
        }

        if enemy.is_empty() && target.is_none() {
            return;
        }

        let (enemy_entity, mut enemy, children, _) = target.unwrap_or_else(|| enemy.single_mut());

        let (_, transform, mut text) = texts.get_mut(*children.iter().next().unwrap()).unwrap();

        let string = text.sections[0].value.clone();

        if string.chars().next().unwrap_or(' ') == key.char {
            text.sections[0].value.remove(0);

            let sprite = TextureAtlasSprite::new(atlases.fish);

            commands
                .spawn(SpriteSheetBundle {
                    sprite,
                    texture_atlas: atlases.texture_atlas.clone(),
                    transform: Transform {
                        translation: Vec3::new(0.0, PENGUIN_THROW_ORIGIN_Y, 0.5),
                        ..default()
                    },
                    ..default()
                })
                .insert(Name::new("Fish"))
                .insert(FishThrowed {
                    target: enemy_entity,
                    target_position: transform.translation().truncate(),
                });

            let mut penguin = penguin.single_mut();
            penguin.animate = true;
        } else {
            enemy.points = (enemy.points as f32 * 0.9f32) as u32;

            miss();
        }

        if text.sections[0].value.is_empty() {
            commands
                .entity(enemy_entity)
                .remove::<EnemyAnimated>()
                .remove::<Target>();
            enemy.state = EnemyState::Idle;
        }
    }
}

fn give_point_when_dead(
    target: RemovedComponents<Target>,
    enemy_query: Query<(&Enemy, Option<&GeneratedEnemy>)>,
    mut stage: Query<&mut StageComponent>,
    mut score: ResMut<PlayerScore>,
    mut player_stats: ResMut<PlayerStats>,
) {
    let Some( target ) = target.iter().next() else {
        return;
    };

    let mut stage = stage.single_mut();
    let (enemy, generated) = enemy_query.get(target).unwrap();

    score.add((enemy.points as f32 * (1.0 + stage.bonus())) as u32);
    player_stats.combos_count += 1;

    println!(
        "enemy.rs:241 | Combos: {} ; Misses: {}",
        player_stats.combos_count, player_stats.misses
    );

    stage.increase_bonus();
    // Only non generated entity descrease the counter
    if generated.is_none() {
        stage.enemy_defeated();
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_enemy(
    commands: &mut Commands,

    elapsed_time: f64,
    position: Vec2,
    velocity: f32,
    points: u32,
    word: impl Into<String>,

    texture: Handle<TextureAtlas>,
    fonts: &Res<FontsAtlas>,
    long_word: bool,
) {
    let word = word.into();

    let mut enemy = commands.spawn(SpriteSheetBundle {
        texture_atlas: texture,
        transform: Transform::from_translation(Vec3::new(position.x, position.y, 1.0)),
        ..default()
    });
    enemy
        .insert(Enemy {
            life: word.len(),
            state: EnemyState::Walk,
            velocity,
            points,
        })
        .insert(EnemyAnimated {
            timer: Timer::new(Duration::from_millis(100), TimerMode::Repeating),
        })
        .insert(Name::new("Enemy"));
    if long_word {
        enemy.insert(EnemyGenitor {
            enemy_type: EnemyGeneratedType::LittleSpider,
            last_generation: elapsed_time,
        });
    }
    let enemy_id = enemy.id();

    let entity = commands
        .spawn(Text2dBundle {
            text: Text::from_section(
                word,
                TextStyle {
                    font: fonts.common_font.clone(),
                    font_size: 20.0,
                    color: Color::BLACK,
                },
            )
            .with_alignment(TextAlignment::CENTER),
            transform: Transform::from_translation(Vec3::new(0.0, -CASE_SIZE / 1.8, 1.5)),
            ..default()
        })
        .insert(Name::new("Enemy Text"))
        .insert(TextEnemy {
            enemy_entity_id: enemy_id,
        })
        .id();

    commands.entity(enemy_id).add_child(entity);
}

fn spawn_entity(
    mut commands: Commands,
    mut stage: Query<&mut StageComponent>,
    time: Res<Time>,
    atlases: Res<EntitiesAtlas>,
    fonts: Res<FontsAtlas>,
    words: ResMut<WordsResource>,
) {
    let mut stage = stage.single_mut();

    if !stage.can_spawn_enemy() {
        return;
    }

    stage.spawn_timer.tick(time.delta());

    if stage.spawn_timer.just_finished() {
        let mut thread_rng = rand::thread_rng();
        let position_x_percent = thread_rng.gen_range(-0.8..=0.8);
        let spawn_long_word = if stage.can_spawn_long_enemy() {
            thread_rng.gen_bool(0.5)
        } else {
            false
        };

        let position = Vec2::new(
            position_x_percent * WINDOW_WIDTH / 2.0,
            WINDOW_HEIGHT / 2.0 + CASE_SIZE / 1.75,
        );

        let velocity;
        let points;
        let word;
        let texture;

        if spawn_long_word {
            velocity = 0.7 + stage.level as f32 / 10.0;
            points = 10;
            word = words.long_word();
            texture = atlases.big_spider.clone();
        } else if thread_rng.gen_bool(0.1) && stage.level > 3 {
            velocity = 0.7 + stage.level as f32 / 10.0;
            points = 12;
            texture = atlases.carot.clone();

            spawn_enemy(
                &mut commands,
                time.elapsed_seconds_f64(),
                position,
                velocity,
                points,
                words.special_word(),
                texture,
                &fonts,
                false,
            );
            stage.enemy_born(false);

            return;
        } else {
            velocity = 1.0 + stage.level as f32 / 10.0;
            points = 5;
            word = words.short_word();
            texture = atlases.little_spider.clone();
        }

        spawn_enemy(
            &mut commands,
            time.elapsed_seconds_f64(),
            position,
            velocity,
            points,
            word,
            texture,
            &fonts,
            spawn_long_word,
        );

        stage.enemy_born(spawn_long_word);
    }
}

fn generate_enemy(
    mut commands: Commands,
    mut stage: Query<&mut StageComponent>,
    time: Res<Time>,
    mut enemies: Query<(&Enemy, &mut EnemyGenitor, &GlobalTransform), With<Enemy>>,
    words: ResMut<WordsResource>,
    fonts: Res<FontsAtlas>,
    atlases: Res<EntitiesAtlas>,
) {
    let mut stage = stage.single_mut();

    for (enemy, mut genitor, transform) in enemies.iter_mut() {
        let elapsed = time.elapsed_seconds_f64();

        if elapsed - genitor.last_generation >= ENEMY_GENERATION_TIME_INTERVAL.0
            && enemy.state == EnemyState::Walk
        {
            let velocity = 1.2 + stage.level as f32 / 10.0;
            let position = Vec2::new(
                transform.translation().x,
                transform.translation().y - CASE_SIZE,
            );
            let points = 3;
            let word = words.short_word();
            let texture = atlases.little_spider.clone();

            stage.enemy_born(false);

            spawn_enemy(
                &mut commands,
                time.elapsed_seconds_f64(),
                position,
                velocity,
                points,
                word,
                texture,
                &fonts,
                false,
            );
            genitor.last_generation = elapsed;
        }
    }
}

fn despawn_enemies(
    mut commands: Commands,
    enemies: Query<(Entity, &Enemy, &GlobalTransform, Option<&EnemyGenitor>)>,
    atlases: Res<EntitiesAtlas>,
    assets: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for (entity, enemy, transform, big_enemy) in enemies.iter() {
        if enemy.life == 0 {
            commands.entity(entity).despawn_recursive();
            draw_dead_enemy_blood(
                &mut commands,
                transform.translation().truncate(),
                &atlases,
                big_enemy.is_some(),
            );

            let sound_effect = assets.load("audio/enemy_killed.ogg");
            audio.play(sound_effect);
        }
    }
}

fn draw_dead_enemy_blood(
    commands: &mut Commands,
    position: Vec2,
    atlases: &Res<EntitiesAtlas>,
    big_enemy: bool,
) {
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: if big_enemy {
                atlases.big_enemy_blood.clone()
            } else {
                atlases.little_enemy_blood.clone()
            },
            transform: Transform {
                translation: position.extend(0.0),
                ..default()
            },
            ..default()
        })
        .insert(Blood {
            animation_timer: Timer::new(Duration::from_millis(80), TimerMode::Repeating),
            clear_timer: Timer::new(Duration::from_millis(5000), TimerMode::Once),
        })
        .insert(Name::new("Blood"));
}

fn animate_dead_enemy_blood(
    time: Res<Time>,
    texture: Res<Assets<TextureAtlas>>,
    mut bloods: Query<(&mut TextureAtlasSprite, &Handle<TextureAtlas>, &mut Blood)>,
) {
    for (mut sprite, handle_texture, mut animation) in bloods.iter_mut() {
        animation.animation_timer.tick(time.delta());
        let texture_atlas = texture.get(handle_texture).unwrap();

        if animation.animation_timer.just_finished() {
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
            animation.animation_timer.pause();
        }
    }
}

fn clear_dead_enemy_blood(
    mut commands: Commands,
    time: Res<Time>,
    mut bloods: Query<(Entity, &mut Blood, &mut Transform)>,
) {
    for (entity, mut blood, mut transform) in bloods.iter_mut() {
        blood.clear_timer.tick(time.delta());

        let elapsed = blood.clear_timer.elapsed().as_millis() as f32;

        if elapsed >= BLOOD_CLEAR_DELAY {
            let scale = 1.0
                - (elapsed - BLOOD_CLEAR_DELAY)
                    / (blood.clear_timer.duration().as_millis() as f32 - BLOOD_CLEAR_DELAY);

            transform.scale = Vec3::new(scale, scale, 1.0);

            if blood.clear_timer.just_finished() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
