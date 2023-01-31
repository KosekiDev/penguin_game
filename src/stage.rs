use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;

use crate::{
    assets::FontsAtlas,
    enemy::{MIN_SPAWN_TIME_INTERVAL, SPAWN_TIME_INTERVAL},
    player::PlayerScore,
    GameState, CASE_SIZE, WINDOW_HEIGHT,
};

pub const TEXT_FONT_SIZE: f32 = 30.0;

#[derive(Component, Default, Debug)]
pub struct StageComponent {
    pub level: u16,
    pub enemies_to_defeat: usize,
    pub enemies_defeated: usize,

    pub long_enemies_alive: usize,
    pub long_enemies_defeated: usize,

    pub enemies_alive: usize,
    pub spawn_timer: Timer,

    bonus_ratio: f32, // bonus to apply when an enemy is defeated. Grow with combos
}

impl StageComponent {
    pub fn enemy_defeated(&mut self) {
        self.enemies_defeated += 1;
        self.enemies_alive -= 1;
    }
    pub fn enemy_born(&mut self, long_ennemy: bool) {
        self.enemies_alive += 1;

        if long_ennemy {
            self.long_enemies_alive += 1;
        }
    }

    pub fn increase_bonus(&mut self) {
        self.bonus_ratio += 0.2;
    }
    pub fn decrease_bonus(&mut self) {
        if self.bonus_ratio <= 0.0 {
            return;
        }

        self.bonus_ratio -= 0.2;
    }

    pub fn bonus(&self) -> f32 {
        self.bonus_ratio
    }

    pub fn can_spawn_enemy(&self) -> bool {
        self.enemies_alive + self.enemies_defeated < self.enemies_to_defeat
    }
    pub fn can_spawn_long_enemy(&self) -> bool {
        self.can_spawn_enemy()
            && self.long_enemies_alive + self.long_enemies_defeated
                // < ((self.level as f32 - 1.0) * 0.8) as usize
                < ( self.level / 3 ) as usize
    }

    pub fn reset(&mut self) {
        self.enemies_defeated = 0;

        self.long_enemies_alive = 0;
        self.long_enemies_defeated = 0;

        self.enemies_alive = 0;
        self.bonus_ratio = 0.0;
        self.spawn_timer.reset();
    }

    pub fn next_level(&mut self) {
        self.level += 1;

        let duration: u64 = (self.spawn_timer.duration().as_millis() as f32 * 0.85) as u64;
        self.spawn_timer.set_duration(Duration::from_millis(
            if duration < MIN_SPAWN_TIME_INTERVAL {
                MIN_SPAWN_TIME_INTERVAL
            } else {
                duration
            },
        ));
        self.spawn_timer.reset();
        self.enemies_to_defeat = (self.enemies_to_defeat as f32 * 2.25) as usize;
    }
}

#[derive(Component)]
pub struct StageLevelText {
    timer: Timer,
    animate: bool,
}

#[derive(Component)]
pub struct StageScoreText;

pub struct StagePlugin;
impl Plugin for StagePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_set_to_stage(
            StartupStage::Startup,
            SystemSet::new().with_system(draw_stage_data),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Gameplay).with_system(animate_score_changing),
        )
        .add_system_set(SystemSet::on_update(GameState::Gameplay).with_system(stage_update));
    }
}

fn draw_stage_data(mut commands: Commands, fonts: Res<FontsAtlas>) {
    let stage_name = "Stage ".to_string();
    let stage_level = format!("{}", 1);

    let x = CASE_SIZE * 1.5;
    let y = -WINDOW_HEIGHT / 2.0 + CASE_SIZE * 2.2;
    let z = 5.0;

    commands
        .spawn(Text2dBundle {
            text: Text::from_sections([
                TextSection::new(
                    stage_name,
                    TextStyle {
                        font: fonts.common_font.clone(),
                        font_size: TEXT_FONT_SIZE,
                        color: Color::BLACK,
                    },
                ),
                TextSection::new(
                    stage_level,
                    TextStyle {
                        font: fonts.common_font.clone(),
                        font_size: TEXT_FONT_SIZE,
                        color: Color::BLACK,
                    },
                ),
            ])
            .with_alignment(TextAlignment::BOTTOM_LEFT),
            transform: Transform::from_translation(Vec3::new(x, y, z)),
            ..default()
        })
        .insert(StageComponent {
            level: 1,
            enemies_to_defeat: 2,
            spawn_timer: Timer::new(
                Duration::from_millis(SPAWN_TIME_INTERVAL),
                TimerMode::Repeating,
            ),
            ..default()
        })
        .insert(StageLevelText {
            timer: Timer::new(Duration::from_millis(700), TimerMode::Once),
            animate: false,
        });

    let stage_name = "Score ".to_string();
    let stage_level = format!("{}", 0);

    commands
        .spawn(Text2dBundle {
            text: Text::from_sections([
                TextSection::new(
                    stage_name,
                    TextStyle {
                        font: fonts.common_font.clone(),
                        font_size: 20.0,
                        color: Color::BLACK,
                    },
                ),
                TextSection::new(
                    stage_level,
                    TextStyle {
                        font: fonts.common_font.clone(),
                        font_size: 20.0,
                        color: Color::BLACK,
                    },
                ),
            ])
            .with_alignment(TextAlignment::BOTTOM_LEFT),
            transform: Transform::from_translation(Vec3::new(x, y - 30.0, z)),
            ..default()
        })
        .insert(StageScoreText);
}

fn stage_update(
    mut stage: Query<&mut StageComponent, Changed<StageComponent>>,
    mut level_text: Query<(&mut Text, &mut StageLevelText)>,
    mut score_text: Query<&mut Text, (With<StageScoreText>, Without<StageLevelText>)>,
    player_score: Res<PlayerScore>,
) {
    if stage.is_empty() {
        return;
    }

    let mut stage = stage.single_mut();
    let (mut level_text, mut animation) = level_text.single_mut();
    let mut score_text = score_text.single_mut();

    if stage.enemies_defeated >= stage.enemies_to_defeat && stage.enemies_alive == 0 {
        stage.reset();
        stage.next_level();
        stage.enemies_to_defeat = stage.level as usize;
        animation.animate = true;
    }

    level_text.sections[1].value = format!("{}", stage.level);
    score_text.sections[1].value = format!("{}", player_score.0);
}

fn animate_score_changing(
    time: Res<Time>,
    mut text: Query<(&mut StageLevelText, &mut Transform), Without<StageScoreText>>,
) {
    let (mut animation, mut transform) = text.single_mut();

    if animation.animate {
        animation.timer.tick(time.delta());

        let eq = |x: f32| (1.0 / (2.0 * PI).sqrt()) * ((-x.powf(2.0)) / 2.0).exp();
        let eq0 = eq(0.0);

        let x = 8.0 * animation.timer.elapsed().as_millis() as f32
            / animation.timer.duration().as_millis() as f32;

        transform.scale = Vec3::new(
            1.0 + eq(x - 4.0) * (1.0 / eq0),
            1.0 + eq(x - 4.0) * (1.0 / eq0),
            1.0,
        );

        if animation.timer.just_finished() {
            animation.animate = false;
            animation.timer.reset();
        }
    }
}
