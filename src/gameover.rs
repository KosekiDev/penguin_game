use bevy::prelude::*;

use crate::{GameState, assets::FontsAtlas};

#[derive(Component)]
pub struct GameoverUIRoot;

pub struct GameoverPlugin;

impl Plugin for GameoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(spawn_game_over));
    }
}

fn spawn_game_over(mut commands: Commands, font_server: Res<FontsAtlas>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BackgroundColor(Color::WHITE),
            ..default()
        })
        .insert(GameoverUIRoot)
        .with_children(|commands| {
            commands.spawn(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect::all(Val::Percent(3.0)),
                    ..default()
                },
                text: Text::from_section(
                    "GameOver",
                    TextStyle {
                        font: font_server.common_font.clone(),
                        font_size: 56.0,
                        color: Color::RED,
                    },
                ),
                ..default()
            });
        });
}
