use bevy::{audio::AudioSink, prelude::*};

use crate::{
    assets::AudioAtlas, defeat_zone::DefeatZonePlugin, enemy::EnemyPlugin, penguins::PenguinPlugin,
    stage::StagePlugin, words::WordsPlugin, GameState,
};

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EnemyPlugin)
            .add_plugin(PenguinPlugin)
            .add_plugin(WordsPlugin)
            .add_plugin(StagePlugin)
            .add_plugin(DefeatZonePlugin)
            .add_system_set(
                SystemSet::on_enter(GameState::Gameplay).with_system(play_background_music),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Gameplay).with_system(stop_background_music),
            );
    }
}

fn play_background_music(
    assets: Res<AssetServer>,
    audio: Res<Audio>,
    mut audio_atlas: ResMut<AudioAtlas>,
    audio_sink: Res<Assets<AudioSink>>,
) {
    let music = assets.load("audio/background.ogg");
    let handle = audio_sink.get_handle(audio.play_with_settings(
        music,
        PlaybackSettings {
            repeat: true,
            volume: 1.0,
            speed: 1.0,
        },
    ));
    audio_atlas.music = Some(handle);
}

fn stop_background_music(audio_atlas: Res<AudioAtlas>, audio_sink: Res<Assets<AudioSink>>) {
    if let Some(music) = &audio_atlas.music {
        if let Some(sink) = audio_sink.get(music) {
            sink.stop();
        }
    }
}
