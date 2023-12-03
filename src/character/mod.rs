use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod systems;
pub mod components;

use systems::*;
use components::*;
pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, character_spawner);
    }
}