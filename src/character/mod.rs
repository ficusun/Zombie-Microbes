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
            .insert_resource(GameStatus::SpawnMenu)
            .insert_resource(PlayerEntityId(Entity::from_bits(1234234567u64)))
            .insert_resource(WorldSize(500.))
            .insert_resource(CharacterCollisionGroups::default())
            .insert_resource(DestroyedEntities::default())
            .add_systems(Update, game_control)
            .add_systems(Update, character_spawner)
            .add_systems(Update, microbes_spawner)
            .add_systems(Update, energy_regeneration)
            .add_systems(Update, health_regeneration)
            .add_systems(Update, skill_process_time)
            .add_systems(Update, skill_to_children)
            .add_systems(Update, seek_system)
            .add_systems(Update, draw_entities)
            .add_systems(Update, camera_scale)
            .add_systems(Update, draw_entities_points)
            .add_systems(Update, collision_events_handler)
            .add_systems(Update, calc_power)
            .add_systems(Update, bots)
            .insert_resource(MicrobeStats{
                min_count: 0,
                max_count: 3000,
                size: 0.5,
                health: 50.0,
                spawn_price: 20.0,
                speed: 40.0,
                spawn_radius_min: 2.5,
                spawn_radius_max: 50.0,
                regeneration_health_rate_per_sec: 0.5,
            }) // 815
            .insert_resource(CharacterEnergyStats{
                max_count: 1000.0,
                regeneration_rate_per_sec: 100.0,
                character_microbes_die_energy_back: 10.0,
                enemy_microbes_kill_energy_reward: 20.0,
            })
            .insert_resource(SkillsCd{
                follow_cursor: 5.,
                target_attack: 2.,
                patrolling: 20.,
            })
            .insert_resource(CharacterStats{
                max_count_bots: 15,
                size: 1.5,
                health: 3000.0,
                energy: 200.0,
                speed: 10.0,
                regeneration_health_rate_per_sec: 5.0,
            })
            .add_systems(Update, fps_display)
            .add_systems(Update, entities_count_display)
            .add_systems(Update, display_player_state)
            //.add_systems(Update, energy_display)
            .add_systems(Startup, spawn_all_text);
    }
}