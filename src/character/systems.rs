use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::components::*;
use bevy::sprite::MaterialMesh2dBundle;

pub fn character_spawner(
    mut commands: Commands,
    //rapier_context: Res<RapierContext>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // let mut entity_commands = commands.spawn(
    //     MaterialMesh2dBundle {
    //     mesh: meshes // Vec2::new(size_of_quad, size_of_quad)
    //         .add(shape::Circle::new(def_char_stat.character_size / 2.).into()) //
    //         .into(),
    //     material: materials.add(ColorMaterial::from(Color::GREEN)),
    //     transform: Transform::from_translation(Vec3::new(5., 5., 1.)),
    //     ..default()
    // });
    let character_size = 15.;
    let mut entity_commands = commands.spawn(RigidBody::KinematicPositionBased);
    entity_commands
        .insert(KinematicCharacterController::default())
        .insert(Collider::ball(character_size))
        .insert(TransformBundle::from(Transform::from_xyz(5., 5., 0.)))
        .insert(MaterialMesh2dBundle {
            mesh: meshes // Vec2::new(size_of_quad, size_of_quad)
                .add(shape::Circle::new(character_size).into()) //
                .into(),
            material: materials.add(ColorMaterial::from(Color::GREEN)),
            ..default()
        });
    //.insert(materials.add(ColorMaterial::from(Color::GREEN)));

    entity_commands.insert(PlayerBundle {
        health: Health(100.),
        speed: Speed(1.),
    });

    entity_commands.insert(IsPlayer);

    // if spawn.is_player {
    //     entity_commands.insert(IsPlayer);
    // } else {
    //     entity_commands.insert(IsBot);
    // }

    // entity_commands.insert(SkillCooldownTime(Timer::from_seconds(
    //     def_char_stat.skill_cooldown_time,
    //     TimerMode::Repeating,
    // )));
    println!("Spawned")
}