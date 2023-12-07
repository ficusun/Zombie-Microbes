use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_vector_shapes::prelude::*;

mod systems;
pub mod components;

use systems::*;
use components::*;
pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, character_spawner)
            .add_systems(Update, microbes_spawner)
            .add_systems(Update, calc_microbes_pos)
            .add_systems(Update, seek_system)
            .add_systems(Update, draw_entities)
            .add_systems(Update, camera_scale)
            .insert_resource(MicrobeStats{
                min_count: 100.0,
                max_count: 10000.0,
                size: 5.0,
                health: 50.0,
                spawn_price: 20.0,
                spawn_radius: 500.0 });
    }
}