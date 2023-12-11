mod character;
mod input;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_vector_shapes::Shape2dPlugin;

use crate::character::components::{GameStatus, MenuCamera};
use character::CharacterPlugin;
use input::InputPlugin;

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
        ))
        .insert_resource(MenuCameraIs(false))
        .add_systems(Update, button_system)
        .add_plugins(CharacterPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(Shape2dPlugin::default())
        .run();
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
pub struct StartButtonText;

#[derive(Component)]
pub struct StartButton;

#[derive(Resource)]
pub struct MenuCameraIs(pub bool);

pub fn setup(
    mut windows: Query<&mut Window>,
    // mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    if let Ok(mut window) = windows.get_single_mut() {
        window.cursor.visible = true;
    }
    rapier_config.gravity = Vec2::ZERO; // For 2D

    // commands.spawn((Camera2dBundle::default(), MenuCamera));
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    //mut game_status: ResMut<GameStatus>,
    mut game_status: ResMut<GameStatus>,
    // mut menu_camera_is: ResMut<MenuCameraIs>,
    mut commands: Commands,
    menu_camera: Query<Entity, With<MenuCamera>>,
    start_button: Query<Entity, With<StartButton>>,
    asset_server: Res<AssetServer>,
) {
    match *game_status {
        GameStatus::Game => return,
        GameStatus::SpawnCharacter => return,
        GameStatus::ResetMenu => {
            *game_status = GameStatus::Game;
            if let Ok(start_b) = start_button.get_single() {
                commands.entity(start_b).despawn_recursive();
            }

            if let Ok(camera) = menu_camera.get_single() {
                commands.entity(camera).despawn_recursive();
                // menu_camera_is.0 = false
            }

            return;
        }
        GameStatus::SpawnMenu => {
            commands.spawn((Camera2dBundle::default(), MenuCamera));

            commands
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .insert(StartButton)
                .with_children(|parent| {
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                width: Val::Px(150.0),
                                height: Val::Px(65.0),
                                border: UiRect::all(Val::Px(5.0)),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            border_color: BorderColor(Color::BLACK),
                            background_color: NORMAL_BUTTON.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle::from_section(
                                    "Start Game",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 40.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                    },
                                ),
                                StartButtonText,
                            ));
                        });
                });
            *game_status = GameStatus::Menu;
        }
        _ => (),
    }

    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *game_status = GameStatus::ResetGame;
                //text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
            }
            Interaction::Hovered => {
                //text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                //text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}
