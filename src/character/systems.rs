use bevy::math::vec3;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::components::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_vector_shapes::prelude::*;
use std::f32::consts::PI;
use bevy_rapier2d::parry::shape::SharedShape;
use bevy_rapier2d::rapier::prelude::{ColliderBuilder};
use rand::Rng;

pub fn character_spawner(
    mut commands: Commands,
    //rapier_context: Res<RapierContext>,
    // mut collider_set: ResMut<ColliderSet>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let pos = Vec3::from((5.,5.,0.));
    let character_size = 15.;
    let mut microbes = Vec::new();

    for i in 0..90 {
        
        let mut children_entity_commands = commands.spawn(RigidBody::KinematicPositionBased);
        children_entity_commands
            //.insert(KinematicCharacterController::default())
            .insert(Collider::ball(5.))
            .insert(Sensor)
            .insert(TransformBundle::from(Transform::from_xyz(pos.x,pos.y,pos.z)))
            .insert(MicrobeBundle {
                health: Health(100.),
                speed: Speed(15.),
                is_microbe: Default::default(),
                orbit: Orbit(20. + i as f32),
            });
        //let _ = collider_set.insert(children_entity_commands.id(), collider);
        microbes.push(children_entity_commands.id());
    }

    let mut entity_commands = commands.spawn(RigidBody::KinematicPositionBased);
    entity_commands
        //.insert(KinematicCharacterController::default())
        .insert(Collider::ball(character_size))
        .insert(TransformBundle::from(Transform::from_xyz(pos.x,pos.y,pos.z)))
        .insert(Camera2dBundle::default());

    entity_commands.insert(PlayerBundle {
        health: Health(100.),
        speed: Speed(1.),
        microbes: Microbes(microbes.clone()),
        draw_it: Default::default(),
    });

    entity_commands.insert(IsPlayer);

    for id in microbes.iter() {
        entity_commands.add_child(*id);
    }

    microbes.clear();

    for i in 0..50 {

        let mut children_entity_commands = commands.spawn(RigidBody::KinematicPositionBased);
        children_entity_commands
            //.insert(KinematicCharacterController::default())
            .insert(Collider::ball(5.))
            .insert(Sensor)
            .insert(TransformBundle::from(Transform::from_xyz(pos.x,pos.y,pos.z)))
            .insert(MicrobeBundle {
                health: Health(100.),
                speed: Speed(7.),
                is_microbe: Default::default(),
                orbit: Orbit(20. + i as f32),
            });
        microbes.push(children_entity_commands.id());
    }

    let mut entity_commands = commands.spawn(RigidBody::KinematicPositionBased);
    entity_commands
        .insert(Collider::ball(character_size))
        .insert(TransformBundle::from(Transform::from_xyz(pos.x,pos.y,pos.z)));

    entity_commands.insert(PlayerBundle {
        health: Health(100.),
        speed: Speed(1.),
        microbes: Microbes(microbes.clone()),
        draw_it: Default::default(),
    });

    entity_commands.insert(IsBot);

    for id in microbes.iter() {
        entity_commands.add_child(*id);
    }
    
    println!("Spawned")
}


fn custom_cos_0_1_0(x: f32) -> f32 {
    0.5 * (1.0 + (x * 2.0 * PI + PI).cos())
}

pub fn draw_characters(
    mut painter: ShapePainter,
    // mut commands: Commands,
    mut player_query: Query<(&Transform, &Microbes), With<DrawIt>>, // , &Wave
    mut microbes_query: Query<(&mut Transform, &Speed, &Orbit), (With<Microbe>, Without<DrawIt>)>, // , &Wave // (With<Microbe>, Without<IsPlayer>)
    time: Res<Time>,
) {
    for (&transform, microbes) in player_query.iter() { // (&transform, &wave)
        painter.transform.translation = transform.translation;
        painter.color = Color::GREEN;
        painter.circle(15.);
        painter.color = Color::RED;
        // let waveData = wave;

        let mut rng = rand::thread_rng();

        let points_count = microbes.len();
        let base_radius = 150.;
        let rotation_speed = 90.;
        let phase_speed = 90.;
        let frequency = 3.;
        let amplitude = 50.;
        let range = 360.;
        let strength_factor = 15.;

        let angl_increment = 360. / points_count as f32;

        let mut test = points_count as f32 / 3.;
        for (i, entity_id) in microbes.iter().enumerate() {
            //if let Some(entity) = commands.get_entity(*entity_id) {
            match microbes_query.get_mut(*entity_id) {
                Ok((mut mic_transform, speed, orbit)) => {
                    let time_elapsed = time.delta().as_secs_f32(); // time.elapsed().as_secs_f32();
                    let angle_deg = i as f32 * angl_increment;

                    let current_angle = (angle_deg + time_elapsed * (speed.0 * rng.gen_range(1.0..3.))).to_radians();
                    let current_phase = (angle_deg + time_elapsed * phase_speed).to_radians();

                    let mut r = orbit.0;// (((i as f32 + 1.) / test) as i32 * 50) as f32; //rng.gen_range(50.0..150.); orbit.0;
                    if (angle_deg < range) {
                        let percent = angle_deg / range;
                        let strength = custom_cos_0_1_0(strength_factor * percent);
                        r += strength * (current_phase * frequency).sin() * (amplitude); // + rng.gen_range(0.0..100.)
                    }

                    let x = current_angle.cos() * r;
                    let y = current_angle.sin() * r;

                    let mic_pos =  Vec3::from((x, y, 0.)); // transform.translation +
                    // painter.transform.translation = mic_transform.translation + mic_pos; //transform.translation

                    mic_transform.translation = mic_pos;
                    painter.transform.translation = mic_transform.translation + transform.translation;
                    // microbes_query.get_mut(*entity_id).unwrap().0.translation = mic_pos;
                    painter.circle(5.);
                }
                Err(e) => (),
            }
            //}
        }
        //
        // for i in 0..points_count {
        //     let time_elapsed = time.elapsed().as_secs_f32();
        //     let angle_deg = i as f32 * angl_increment;
        //
        //     let current_angle = (angle_deg + time_elapsed * rotation_speed).to_radians();
        //     let current_phase = (angle_deg + time_elapsed * phase_speed).to_radians();
        //
        //     let mut r = base_radius;
        //     if (angle_deg < range) {
        //         let percent = angle_deg / range;
        //         let strength = custom_cos_0_1_0(strength_factor * percent);
        //         r += strength * (current_phase * frequency).sin() * amplitude;
        //     }
        //
        //     let x = current_angle.cos() * r;
        //     let y = current_angle.sin() * r;
        //
        //     painter.transform.translation = transform.translation + Vec3::from((x, y, 0.));
        //
        //     painter.circle(5.);
        // }
    }
}
