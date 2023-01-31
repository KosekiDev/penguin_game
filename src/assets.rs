use bevy::prelude::*;

use crate::{defeat_zone::DEFEAT_ZONE_HEIGHT, CASE_SIZE};

#[derive(Resource)]
pub struct EntitiesAtlas {
    pub texture_atlas: Handle<TextureAtlas>,
    // penguin fired
    pub penguin_fired_standby: usize,
    pub penguin_fired_throwed: Handle<TextureAtlas>,

    pub fish: usize,
    // little spider animation
    pub little_spider: Handle<TextureAtlas>,
    pub big_spider: Handle<TextureAtlas>,

    pub carot: Handle<TextureAtlas>,

    pub defeat_zone: usize,

    pub little_enemy_blood: Handle<TextureAtlas>,
    pub big_enemy_blood: Handle<TextureAtlas>,
}

#[derive(Resource)]
pub struct FontsAtlas {
    pub common_font: Handle<Font>,
}

pub struct AssetsPlugin;
impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_assets);
    }
}

fn load_assets(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    audio: Res<Audio>,
) {
    let music = assets.load("audio/background.ogg");
    audio.play_with_settings(
        music,
        PlaybackSettings {
            repeat: true,
            volume: 1.0,
            speed: 1.0,
        },
    );

    let image_handle = assets.load("penguin.png");
    let mut atlas = TextureAtlas::new_empty(image_handle.clone(), Vec2::splat(256.0));

    let penguin_fired_standby = atlas.add_texture(Rect {
        min: Vec2::new(0.0, 0.0),
        max: Vec2::new(CASE_SIZE, CASE_SIZE),
    });

    let penguin_fired_throwed = texture_atlases.add(TextureAtlas::from_grid(
        image_handle.clone(),
        Vec2::splat(CASE_SIZE),
        4,
        1,
        None,
        Some(Vec2::new(0.0, 0.0)),
    ));

    let fish = atlas.add_texture(Rect {
        min: Vec2::new(0.0, CASE_SIZE),
        max: Vec2::new(CASE_SIZE, CASE_SIZE * 2.0),
    });

    let little_spider = texture_atlases.add(TextureAtlas::from_grid(
        image_handle.clone(),
        Vec2::splat(CASE_SIZE),
        3,
        1,
        None,
        Some(Vec2::new(CASE_SIZE * 4.0, 0.0)),
    ));

    let big_spider = texture_atlases.add(TextureAtlas::from_grid(
        image_handle.clone(),
        Vec2::new(CASE_SIZE, CASE_SIZE - 1.0),
        3,
        1,
        None,
        Some(Vec2::new(CASE_SIZE * 4.0, CASE_SIZE)),
    ));

    let carot = texture_atlases.add(TextureAtlas::from_grid(
        image_handle.clone(),
        Vec2::new(CASE_SIZE, CASE_SIZE - 1.0),
        3,
        1,
        None,
        Some(Vec2::new(CASE_SIZE, CASE_SIZE)),
    ));

    let defeat_zone = atlas.add_texture(Rect {
        min: Vec2::new(0.0, CASE_SIZE * 2.0),
        max: Vec2::new(CASE_SIZE * 8.0, CASE_SIZE * 2.0 + DEFEAT_ZONE_HEIGHT),
    });

    let little_enemy_blood = texture_atlases.add(TextureAtlas::from_grid(
        image_handle.clone(),
        Vec2::new(CASE_SIZE, CASE_SIZE),
        3,
        1,
        None,
        Some(Vec2::new(0.0, CASE_SIZE * 4.0)),
    ));

    let big_enemy_blood = texture_atlases.add(TextureAtlas::from_grid(
        image_handle,
        Vec2::new(CASE_SIZE, CASE_SIZE),
        3,
        1,
        None,
        Some(Vec2::new(0.0, CASE_SIZE * 5.0)),
    ));

    let atlas_handle = texture_atlases.add(atlas);

    commands.insert_resource(EntitiesAtlas {
        texture_atlas: atlas_handle,
        penguin_fired_standby,
        penguin_fired_throwed,
        fish,
        little_spider,
        big_spider,
        carot,
        defeat_zone,
        little_enemy_blood,
        big_enemy_blood,
    });
    commands.insert_resource(FontsAtlas {
        common_font: assets.load("fonts/QuattrocentoSans-Regular.ttf"),
    });
}
