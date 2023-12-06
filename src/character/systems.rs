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

    for i in 0..10000 {
        
        let mut children_entity_commands = commands.spawn(RigidBody::Dynamic);
        children_entity_commands
            .insert(Velocity::zero())
            //.insert(KinematicCharacterController::default())
            .insert(Collider::ball(5.))
            //.insert(Sensor)
            .insert(TransformBundle::from(Transform::from_xyz(i as f32,pos.y,pos.z)))
            .insert(MicrobeBundle {
                health: Health(100.),
                speed: Speed(15.),
                is_microbe: Default::default(),
                orbit: Orbit(20. + i as f32),
                draw_stats: DrawStats{
                    radius: 5.0, color: Color::RED
                },
            })
            .insert(Mover{
                max_speed: 100.0,
                max_force: 90.0,
                pos: Default::default(),
                vel: Vec2::new(0., 0.),
                acc: Default::default(),
                stiffness: 3.0,
                damper: 3.0,
                target: Default::default(),
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
        draw_stats: DrawStats{
            radius: character_size, color: Color::GREEN
        },
        combat: CombatState(false),
        target: Default::default(),
    });

    entity_commands.insert(IsPlayer);

    // for id in microbes.iter() {
    //     entity_commands.add_child(*id);
    // }

    microbes.clear();

    // for i in 0..50 {
    //
    //     let mut children_entity_commands = commands.spawn(RigidBody::KinematicPositionBased);
    //     children_entity_commands
    //         //.insert(KinematicCharacterController::default())
    //         .insert(Collider::ball(5.))
    //         .insert(Sensor)
    //         .insert(TransformBundle::from(Transform::from_xyz(pos.x,pos.y,pos.z)))
    //         .insert(MicrobeBundle {
    //             health: Health(100.),
    //             speed: Speed(7.),
    //             is_microbe: Default::default(),
    //             orbit: Orbit(20. + i as f32),
    //                draw_stats: DrawStats{
    //                radius: 5., color: Color::RED
    //                },
    //         })
    //         .insert(Mover{
    //             max_speed: 3.0,
    //             max_force: 3.0,
    //             pos: Default::default(),
    //             vel: Vec2::new(0., 0.),
    //             acc: Default::default(),
    //             stiffness: 3.0,
    //             damper: 3.0,
    //             target: Default::default(),
    //         });
    //     microbes.push(children_entity_commands.id());
    // }
    //
    // let mut entity_commands = commands.spawn(RigidBody::KinematicPositionBased);
    // entity_commands
    //     .insert(Collider::ball(character_size))
    //     .insert(TransformBundle::from(Transform::from_xyz(pos.x,pos.y,pos.z)));
    //
    // entity_commands.insert(PlayerBundle {
    //     health: Health(100.),
    //     speed: Speed(1.),
    //     microbes: Microbes(microbes.clone()),
    //     draw_it: Default::default(),
    //      draw_stats: DrawStats{
    //             radius: character_size, color: Color::GREEN
    //      },
    // });
    //
    // entity_commands.insert(IsBot);
    //
    // for id in microbes.iter() {
    //     entity_commands.add_child(*id);
    // }
    
    println!("Spawned")
}

pub fn camera_scale(
    mut player: Query<(&Microbes, &mut OrthographicProjection), With<IsPlayer>>,
    microbe_stats: Res<MicrobeStats>,
) {
    if let Ok((microbes, mut orthographic_projection)) = player.get_single_mut() {
        let scale = microbes.0.len() as f32 / microbe_stats.max_count;
        let min:f32 = 0.8;
        let max:f32 = 1.8;
        orthographic_projection.scale = min + scale *(max - min); // microbes as f32 * 0.005;
    }
}

pub fn seek(
    mut mover_query: Query<(&mut Transform, &mut Mover, &mut Velocity)>,
    player_query: Query<(&Target, &CombatState)>,
    time: Res<Time>,
) {

    let (tar, combat) = if let Ok(p) = player_query.get_single() {
        (p.0.0, p.1.0)
    } else {
        (Vec2::default(), false)
    };

    for (mut mover_transform, mut mover_data, mut vel) in mover_query.iter_mut() {
        //let displacement = mover_data.target - mover_transform.translation.xy();
        //let force = mover_data.stiffness * displacement - mover_data.damper * mover_data.vel;

        if combat {
            mover_data.target = tar;
        }

        let mut desired = mover_data.target - mover_transform.translation.xy();
        desired = desired.normalize_or_zero() * mover_data.max_speed;
        let steering = desired - mover_data.vel;
        steering.clamp_length(0., mover_data.max_force);

        mover_data.acc = steering; // * time.delta_seconds();
        let acc = mover_data.acc;
        mover_data.vel += acc;
        vel.linvel = vec3(mover_data.vel.x, mover_data.vel.y, 0.).xy();
        //mover_transform.translation += vec3(mover_data.vel.x, mover_data.vel.y, 0.); // * time.delta_seconds();
        mover_data.acc = Vec2::new(0., 0.);
    }
}

pub fn draw_entities(
    mut painter: ShapePainter,
    mut draw_data: Query<(&Transform, &DrawStats)>,

) {
    for (transform, draw_stats) in draw_data.iter() {
        painter.color = draw_stats.color;
        painter.transform.translation = transform.translation;
        painter.circle(draw_stats.radius);
    }
}

fn custom_cos_0_1_0(x: f32) -> f32 {
    0.5 * (1.0 + (x * 2.0 * PI + PI).cos())
}

pub fn calc_microbes_pos(
    mut player_query: Query<(&Transform, &Microbes, &CombatState), With<DrawIt>>, // , &Wave
    mut microbes_query: Query<(&mut Transform, &Speed, &Orbit, &mut Mover), (With<Microbe>, Without<DrawIt>)>, // , &Wave // (With<Microbe>, Without<IsPlayer>)
    time: Res<Time>,
) {
    for (&transform, microbes, combat) in player_query.iter() { // (&transform, &wave)
        // let waveData = wave;
        if combat.0 {
            continue
        }

        let mut rng = rand::thread_rng();

        let points_count = microbes.len();
        let base_radius = 40.;
        let radius_increment = 15.;
        let rotation_speed = 25.;
        let phase_speed = 45.;
        let frequency = 3.;
        let amplitude = 10.;
        let range = 360.;
        let strength_factor = 1.;
        let base_particles_per_orbit = 20.;

        let mut cur_orbit = 10.;
        // let mut test = points_count as f32 / 3.;
        for (i, entity_id) in microbes.iter().enumerate() {
            //if let Some(entity) = commands.get_entity(*entity_id) {
            match microbes_query.get_mut(*entity_id) {
                Ok((mut mic_transform, speed, orbit, mut mover)) => {
                    let particles_per_orbit = base_particles_per_orbit + (radius_increment  / base_radius + 1.);
                    let angl_increment = 360. / particles_per_orbit;
                    let time_elapsed = time.elapsed().as_secs_f32(); // time.delta().as_secs_f32(); // time.elapsed().as_secs_f32();
                    let particle_in_orbit = i as f32 % particles_per_orbit;
                    let angle_deg = particle_in_orbit as f32 * angl_increment;
                    // let angle_deg = i as f32 * angl_increment;

                    let current_angle = (angle_deg + time_elapsed * rotation_speed).to_radians(); //(speed.0 * rng.gen_range(1.0..3.))).to_radians();
                    let current_phase = (angle_deg + time_elapsed * phase_speed).to_radians();

                    cur_orbit += (i as f32 / particles_per_orbit) + 1.;
                    let mut r = base_radius+(radius_increment);// * cur_orbit as f32);
                    // let mut r = orbit.0;// (((i as f32 + 1.) / test) as i32 * 50) as f32; //rng.gen_range(50.0..150.); orbit.0;
                    if (angle_deg < range) {
                        let percent = angle_deg / range;
                        let strength = custom_cos_0_1_0(strength_factor * percent);
                        r += strength * (current_phase * frequency).sin() * amplitude * cur_orbit as f32; // + rng.gen_range(0.0..100.)
                    }

                    let x = current_angle.cos() * r;
                    let y = current_angle.sin() * r;

                    let mic_pos =  Vec3::from((x, y, 0.)); // transform.translation +
                    // painter.transform.translation = mic_transform.translation + mic_pos; //transform.translation
                    mover.target = (transform.translation + mic_pos).xy();
                    // mic_transform.translation = transform.translation + mic_pos; //
                    // painter.transform.translation = mic_transform.translation + transform.translation;
                    // // microbes_query.get_mut(*entity_id).unwrap().0.translation = mic_pos;
                    // painter.circle(5.);
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
