use std::time;
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
        app
            //.insert_resource(MineCollisionGroups::default())
            .add_systems(Startup, character_spawner)
            // .add_systems(Update, first_microbe_spawner)
            .add_systems(Update, microbes_spawner)
            // .add_systems(Update, calc_microbes_pos)
            .add_systems(Update, energy_regeneration)
            .add_systems(Update, skill_process_time)
            .add_systems(Update, skill_to_children)
            .add_systems(Update, new_seek_system)
            //.add_systems(Update, seek_system)
            .add_systems(Update, draw_entities)
            .add_systems(Update, camera_scale)
            .insert_resource(MicrobeStats{
                min_count: 0,
                max_count: 3000,
                size: 5.0,
                health: 50.0,
                spawn_price: 20.0,
                speed: 40.0,
                spawn_radius_min: 30.0,
                spawn_radius_max: 815.0 }) // 815
            .insert_resource(CharacterEnergyStats{
                max_count: 1000.0,
                regeneration_rate_per_sec: 300.0,
                character_microbes_die_energy_back: 10.0,
                enemy_microbes_kill_energy_reward: 20.0,
            })
            .insert_resource(SkillsCd{
                follow_cursor: 5.,
                target_attack: 2.,
                patrolling: 10.,
            });
    }
}