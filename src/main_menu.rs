use bevy::{app::AppExit, prelude::*};

use crate::{assets::FontsAtlas, GameState};

#[derive(Component)]
pub struct MainMenuUIRoot;

#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
pub struct QuitButton;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_main_menu)
            .add_system_set(
                SystemSet::on_update(GameState::MainMenu)
                    .with_system(start_button_clicked)
                    .with_system(quit_button_clicked),
            );
    }
}

fn start_button_clicked(
    mut commands: Commands,
    interactions: Query<&Interaction, (With<StartButton>, Changed<Interaction>)>,
    menu_root: Query<Entity, With<MainMenuUIRoot>>,
    mut game_state: ResMut<State<GameState>>,
) {
    for interaction in &interactions {
        if matches!(interaction, Interaction::Clicked) {
            let root_entity = menu_root.single();
            commands.entity(root_entity).despawn_recursive();

            game_state.set(GameState::Gameplay).unwrap();
        }
    }
}

fn quit_button_clicked(
    interactions: Query<&Interaction, (With<QuitButton>, Changed<Interaction>)>,
    mut exit: EventWriter<AppExit>,
) {
    for interaction in &interactions {
        if matches!(interaction, Interaction::Clicked) {
            exit.send(AppExit);
        }
    }
}

fn spawn_main_menu(mut commands: Commands, font_server: Res<FontsAtlas>) {
    let start_button = spawn_button(&mut commands, &font_server, "Start Game", Color::RED);
    commands.entity(start_button).insert(StartButton);

    let quit_button = spawn_button(
        &mut commands,
        &font_server,
        "Quit",
        Color::rgb(0.4, 0.4, 0.9),
    );
    commands.entity(quit_button).insert(QuitButton);

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
        .insert(MainMenuUIRoot)
        .with_children(|commands| {
            commands.spawn(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect::all(Val::Percent(3.0)),
                    ..default()
                },
                text: Text::from_section(
                    "Penguin Game",
                    TextStyle {
                        font: font_server.common_font.clone(),
                        font_size: 56.0,
                        color: Color::BLACK,
                    },
                ),
                ..default()
            });
        })
        .add_child(start_button)
        .add_child(quit_button);
}

fn spawn_button(
    commands: &mut Commands,
    font_server: &FontsAtlas,
    text: &str,
    color: Color,
) -> Entity {
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(45.0), Val::Percent(8.0)),
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect::all(Val::Percent(2.0)),
                ..default()
            },
            background_color: color.into(),
            ..default()
        })
        .with_children(|commands| {
            commands.spawn(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect::all(Val::Percent(3.0)),
                    ..default()
                },
                text: Text::from_section(
                    text,
                    TextStyle {
                        font: font_server.common_font.clone(),
                        font_size: 34.0,
                        color: Color::BLACK,
                    },
                ),
                ..default()
            });
        })
        .id()
}
