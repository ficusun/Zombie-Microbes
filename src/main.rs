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

pub fn setup(mut commands: Commands) {
    // let mut test = Camera2dBundle::default();
    // test.transform.translation.z = 10.;
    // println!("{}",&test.transform.translation);
    commands.spawn(Camera2dBundle::default());

    // test.send(SpawnCharacter { is_player: true });
}
