mod character;
mod input;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use character::CharacterPlugin;
use input::InputPlugin;

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_plugins((DefaultPlugins,
                     RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
                     RapierDebugRenderPlugin::default()))
        .add_plugins(CharacterPlugin)
        .add_plugins(InputPlugin)
        .run();
}

pub fn setup(mut commands: Commands) {
    let mut test = Camera2dBundle::default();
    test.transform.translation.z = 10.;
    println!("{}",&test.transform.translation);
    commands.spawn(test);

    // test.send(SpawnCharacter { is_player: true });
}