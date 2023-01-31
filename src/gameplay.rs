use bevy::prelude::*;

use crate::{
    defeat_zone::DefeatZonePlugin, enemy::EnemyPlugin, penguins::PenguinPlugin, stage::StagePlugin,
    words::WordsPlugin,
};

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EnemyPlugin)
            .add_plugin(PenguinPlugin)
            .add_plugin(WordsPlugin)
            .add_plugin(StagePlugin)
            .add_plugin(DefeatZonePlugin);
    }
}
