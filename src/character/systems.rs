use bevy::math::vec3;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::components::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_vector_shapes::prelude::*;
use std::f32::consts::PI;

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
        .insert(TransformBundle::from(Transform::from_xyz(5., 5., 0.)));
    // .insert(MaterialMesh2dBundle {
    //     transform: Transform::from_xyz(-50.,-50.,0.),
    //     mesh: meshes // Vec2::new(size_of_quad, size_of_quad)
    //         .add(shape::Circle::new(character_size).into()) //
    //         .into(),
    //     material: materials.add(ColorMaterial::from(Color::GREEN)),
    //     ..default()
    // });
    //.insert(materials.add(ColorMaterial::from(Color::GREEN)));

    entity_commands.insert(PlayerBundle {
        health: Health(100.),
        speed: Speed(1.),
    });

    entity_commands.insert(IsPlayer);

    let mut entity_commands = commands.spawn(RigidBody::KinematicPositionBased);
    entity_commands
        .insert(KinematicCharacterController::default())
        .insert(Collider::ball(character_size))
        .insert(TransformBundle::from(Transform::from_xyz(50., 50., 0.)));
    // .insert(MaterialMesh2dBundle {
    //     transform: Transform::from_xyz(50.,50.,0.),
    //     mesh: meshes // Vec2::new(size_of_quad, size_of_quad)
    //         .add(shape::Circle::new(character_size).into()) //
    //         .into(),
    //     material: materials.add(ColorMaterial::from(Color::GREEN)),
    //     ..default()
    // });
    //.insert(materials.add(ColorMaterial::from(Color::GREEN)));

    // entity_commands.insert(PlayerBundle {
    //     health: Health(100.),
    //     speed: Speed(1.),
    // });
    //
    // entity_commands.insert(IsBot);
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


fn custom_cos_0_1_0(x: f32) -> f32 {
    0.5 * (1.0 + (x * 2.0 * PI + PI).cos())
}

pub fn draw_characters(
    mut painter: ShapePainter,
    mut playerQuery: Query<(&Transform, &Wave), With<IsPlayer>>,
    time: Res<Time>,
) {
    for playerData in playerQuery.iter() {
        painter.transform.translation = playerData.0.translation;
        painter.color = Color::GREEN;
        painter.circle(15.);
        painter.color = Color::RED;

        let waveData = playerData.1;

        let points_count = 90;
        let base_radius = 150.;
        let rotation_speed = 90.;
        let phase_speed = 90.;
        let frequency = 3.;
        let amplitude = 300.;
        let range = 270.;
        let strength_factor = 1.;

        let anglIncrement = 360. / points_count as f32;

        for i in 0..points_count {
            let time_elapsed = time.elapsed().as_secs_f32();
            let angleDeg = i as f32 * anglIncrement;

            let currentAngle = (angleDeg + time_elapsed * rotation_speed).to_radians();
            let currentPhase = (angleDeg + time_elapsed * phase_speed).to_radians();

            let mut r = base_radius;
            if (angleDeg < range) {
                let percent = angleDeg / range;
                let strength = custom_cos_0_1_0(strength_factor * percent);
                r += strength * (currentPhase * frequency).sin() * amplitude;
            }

            let x = currentAngle.cos() * r;
            let y = currentAngle.sin() * r;

            painter.transform.translation = playerData.translation + Vec3::from((x, y, 0.));

            painter.circle(5.);
        }
}
