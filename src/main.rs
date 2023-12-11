mod character;
mod input;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_vector_shapes::Shape2dPlugin;

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
        .add_plugins(CharacterPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(Shape2dPlugin::default())
        .run();
}

pub fn setup(
    mut windows: Query<&mut Window>,
    // mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    if let Ok(mut window) = windows.get_single_mut() {
        window.cursor.visible = true;
    }

    rapier_config.gravity = Vec2::ZERO; // For 2D
}
