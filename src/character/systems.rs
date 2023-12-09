use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::components::*;
use crate::input::components::Cursor;
use bevy_vector_shapes::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

pub fn character_spawner(mut commands: Commands) {
    let pos = Vec3::from((5., 5., 0.));
    let character_size = 15.;

    commands
        .spawn(RigidBody::KinematicPositionBased)
        //.insert(KinematicCharacterController::default())
        .insert(Collider::ball(character_size))
        .insert(TransformBundle::from(Transform::from_xyz(
            pos.x, pos.y, pos.z,
        )))
        .insert(Camera2dBundle::default())
        .insert(CharacterBundle {
            health: Health(100.),
            speed: Speed(1.),
            // draw_it: Default::default(),
            to_spawn_mic: Default::default(),
            draw_stats: DrawStats {
                radius: character_size,
                color: Color::GREEN,
            },
            combat: CombatState(false),
            target: Default::default(),
            energy: Energy(250.),
            count_microbes: Default::default(),
            skill_cd: Default::default(),
            skill: Default::default(),
            is_bot: Default::default(),
            character: Default::default(),
        })
        .insert(IsPlayer)
        .with_children(|parent| {
            parent.spawn(()).insert(InvisiblePlaceholder).insert(Skill::default());
        });
    println!("Spawned")
}

pub fn microbes_spawner(
    mut commands: Commands,
    microbe_stats: Res<MicrobeStats>,
    mut characters_query: Query<(Entity, &mut ToSpawnMic, &mut Energy, &Children, &IsBot)>, // , With<IsCharacter>
) {

    let mut rng = rand::thread_rng();
    for (entity, mut to_spawn_min, mut energy, children, is_bot) in characters_query.iter_mut() {
        if to_spawn_min.0 && energy.0 > microbe_stats.spawn_price {
            while energy.0 > microbe_stats.spawn_price {
                if children.len() >= microbe_stats.max_count as usize {
                    to_spawn_min.0 = false;
                    break
                }
                energy.0 -= microbe_stats.spawn_price;

                let x =
                    rng.gen_range(-microbe_stats.spawn_radius_max..microbe_stats.spawn_radius_max); // transform.translation.x +
                let y =
                    rng.gen_range(-microbe_stats.spawn_radius_max..microbe_stats.spawn_radius_max); // transform.translation.y +

                let mut children_entity_commands = commands.spawn(RigidBody::Dynamic);
                children_entity_commands
                    .insert(Velocity::zero())
                    .insert(Collider::ball(5.))
                    .insert(TransformBundle::from(Transform::from_xyz(x, y, 0.)))
                    .insert(MicrobeBundle {
                        health: Health(100.),
                        is_microbe: Default::default(),
                        // orbit: Orbit(20. + i as f32),
                        draw_stats: DrawStats {
                            radius: microbe_stats.size,
                            color: Color::RED,
                        },
                        target: Default::default(),
                        // skill_target: Default::default(),
                        // rest_target: Default::default(),
                        // parent_id: ParentEntityID(entity),
                        // targets: Default::default(),
                        is_bot: IsBot(is_bot.0),
                        skill: Default::default(),
                    })
                    .insert(Mover {
                        max_speed: 200.0,
                        max_force: 100.0,
                        vel: Vec2::new(0., 0.),
                        acc: Default::default(),
                        stiffness: 2.0,
                        damper: 1.0,
                        angle: rng.gen_range(0.0..=2. * PI), // random(0, 2 * PI)
                        radius: rng.gen_range(
                            microbe_stats.spawn_radius_min..=microbe_stats.spawn_radius_max,
                        ),
                        speed: microbe_stats.speed,
                    });

                children_entity_commands.set_parent(entity);
                // if let Some(mut entityCom) = commands.get_entity(entity) {
                //     entityCom.add_child(children_entity_commands.id());
                // };
                //count_microbes += 1;
            }

            to_spawn_min.0 = false;
        }
    }
}

pub fn camera_scale(
    mut player: Query<(&Microbes, &mut OrthographicProjection), With<IsPlayer>>,
    microbe_stats: Res<MicrobeStats>,
) {
    if let Ok((microbes, mut orthographic_projection)) = player.get_single_mut() {
        let scale = (microbes.0.len() as i32 / microbe_stats.max_count) as f32;
        let min: f32 = 0.8;
        let max: f32 = 1.8;
        orthographic_projection.scale = min + scale * (max - min); // microbes as f32 * 0.005;
    }
}

// passive regeneration energy for each character per second
pub fn energy_regeneration(
    mut characters: Query<(&mut Energy), Without<Microbe>>,
    energy_stats: Res<CharacterEnergyStats>,
    time: Res<Time>,
) {
    for mut energy in characters.iter_mut() {
        energy.0 = (energy.0 + energy_stats.regeneration_rate_per_sec * time.delta_seconds())
            .min(energy_stats.max_count);
    }
}

pub fn skill_process_time(
    mut characters: Query<(&mut SkillCd, &mut Skill), With<IsCharacter>>,
    time: Res<Time>,
) {
    for (mut skill_cd, mut skill) in characters.iter_mut() {
        skill_cd.0.tick(time.delta());
        if skill_cd.0.just_finished() {
            *skill = Skill::default();
        }
    }
}

pub fn skill_to_children(
    mut characters: Query<(&mut Skill, &Children), (With<IsCharacter>, Changed<Skill>)>,
    mut microbes_query: Query<(&mut Skill), (With<Microbe>, Without<IsCharacter>)>,
) {
    for (mut skill, children) in characters.iter_mut() {
        for child in children.iter() {
            if let Ok(mut microbe_skill) = microbes_query.get_mut(*child) {
                if *microbe_skill != *skill {
                    *microbe_skill = *skill;
                }
            }
        }
    }
}

pub fn new_seek_system(
    mut microbes_query: Query<
        (
            &Transform,
            &Skill,
            // &mut Targets,
            &mut Target,
            &mut Mover,
            &mut Velocity,
            &IsBot,
        ),
        With<Microbe>,
    >,
    // characters_query: Query<(&Target, &CombatState, &Transform), Without<Microbe>>,
    time: Res<Time>,
    cursor: Res<Cursor>,
    microbe_stats: Res<MicrobeStats>,
) {
    let mut force: Vec2 = Vec2::default();
    let mut rng = rand::thread_rng();
    for (microbe_transform, skill, mut microbe_target, mut mover, mut vel, is_bot) in
        microbes_query.iter_mut()
    {
        match skill {
            Skill::Rest => {
                // if let Some(mut microbe_target) = targets.0.first_mut() {

                    if microbe_target.0.distance(microbe_transform.translation.xy()) <= mover.speed {
                        mover.angle +=  mover.speed * time.delta().as_secs_f32(); // PI/6.;//
                        microbe_target.x = mover.radius * mover.angle.cos();
                        microbe_target.y = mover.radius * mover.angle.sin();
                        let pulsation_speed = 3.0;

                        let pulsation_phase = time.elapsed().as_secs_f32()
                            * pulsation_speed
                            * rng.gen_range(0.1..10.0);
                        mover.radius = microbe_stats.spawn_radius_min
                            + (microbe_stats.spawn_radius_max - microbe_stats.spawn_radius_min)
                                * 0.5
                                * (1.0 + pulsation_phase.sin());
                    } else {
                        let displacement = microbe_target.0 - microbe_transform.translation.xy();
                        force = mover.stiffness * displacement - mover.damper * mover.vel;
                    }
                //}
            }
            Skill::Patrolling(tar1, tar2) => {}
            Skill::FollowCursor(tar1) => {
                //if let Some(mut microbe_target) = tar1 {
                    // **mic_target = transform.translation.xy() + **per_target;

                    if is_bot.0 {
                        if let Some(new_target) = tar1 {
                            microbe_target.0 = *new_target;
                        }
                    } else {
                        microbe_target.0 = cursor.0;
                    }

                    let mut desired = microbe_target.0 - microbe_transform.translation.xy();
                    desired = desired.normalize_or_zero() * mover.max_speed;
                    force = desired - mover.vel;
                    force.clamp_length(0., mover.max_force);
                //}
            }
            Skill::TargetAttack(tar1, damper) => {
                if let Some(microbe_target) = tar1 {
                    let displacement = *microbe_target - microbe_transform.translation.xy();
                    force = mover.stiffness * displacement - *damper * mover.vel;
                }
            }
        }

        mover.acc = force;
        mover.vel = mover.vel + mover.acc * time.delta_seconds();
        vel.linvel = mover.vel;
        mover.acc *= 0.;
    }
}

pub fn draw_entities(
    mut painter: ShapePainter,
    mut draw_data: Query<(&GlobalTransform, &DrawStats)>,
) {
    for (transform, draw_stats) in draw_data.iter() {
        painter.color = draw_stats.color;
        painter.transform.translation = transform.translation();
        painter.circle(draw_stats.radius);
    }
}

fn custom_cos_0_1_0(x: f32) -> f32 {
    0.5 * (1.0 + (x * 2.0 * PI + PI).cos())
}

pub fn calc_microbes_pos(
    characters_query: Query<(Entity, &Children, &Transform, &CombatState), Without<Microbe>>, // , &Wave &CountMicrobes
    mut microbes_query: Query<(&mut Target), With<Microbe>>, // , &Wave // (With<Microbe>, Without<IsPlayer>)
    time: Res<Time>,
) {
    let base_radius = 30.;
    let radius_increment = 20.;
    let rotation_speed = 10.;
    let phase_speed = 90.;
    let frequency = 0.;
    let amplitude = 0.0;
    let range = 360.;
    let strength_factor = 0.;
    let base_particles_per_orbit = 5.;
    let mut curr_radius = base_radius;
    let mut prev_radius = base_radius;
    let mut curr_particles_per_orbit = base_particles_per_orbit as i32;
    let mut curr_orbit = 1;
    let mut already_done = 0;

    for (entity, children, transform, combat) in characters_query.iter() {
        if combat.0 {
            continue;
        }

        for (i, microbe) in children.iter().enumerate() {
            if let Ok(mut target) = microbes_query.get_mut(*microbe) {
                if i as i32 % curr_particles_per_orbit == 0 && i > 0 {
                    already_done += curr_particles_per_orbit;
                    let df = curr_radius;
                    curr_radius = prev_radius + radius_increment;
                    prev_radius = df;
                    curr_particles_per_orbit = (curr_particles_per_orbit as f32
                        * (curr_radius / prev_radius))
                        .floor() as i32;
                    curr_orbit += 1;
                }

                let angl_increment = 360. / curr_particles_per_orbit as f32;
                let time_elapsed = time.elapsed().as_secs_f32(); // time.delta().as_secs_f32(); // time.elapsed().as_secs_f32();
                let particle_in_orbit = i as i32 % curr_particles_per_orbit;
                let angle_deg = particle_in_orbit as f32 * angl_increment;

                let current_angle = (angle_deg + time_elapsed * rotation_speed).to_radians(); //(speed.0 * rng.gen_range(1.0..3.))).to_radians();
                let current_phase = (angle_deg + time_elapsed * phase_speed).to_radians();

                //let t_orbit = (i as i32 / curr_particles_per_orbit) + 1;
                let mut r = curr_radius; //base_radius+(radius_increment);// * cur_orbit as f32);
                if (angle_deg < range) {
                    let percent = angle_deg / range;
                    let strength = custom_cos_0_1_0(strength_factor * percent);
                    r += strength
                        * (current_phase * frequency).sin()
                        * amplitude
                        * curr_orbit as f32; // + rng.gen_range(0.0..100.)
                }

                let x = current_angle.cos() * r;
                let y = current_angle.sin() * r;

                let mic_pos = Vec2::from((x, y)); // transform.translation + , 0.
                target.0 = mic_pos; // (transform.translation + ).xy()
            }
        }
    }

    // for (mut target, parent_entity) in microbes_query.iter_mut() {
    //     if let Ok((&parent_transform, &parent_combat)) = characters_query.get(parent_entity) {
    //         if *parent_combat {
    //             continue
    //         }
    //
    //         if i as i32  % curr_particles_per_orbit == 0 && i > 0 {
    //             already_done += curr_particles_per_orbit;
    //             let df = curr_radius;
    //             curr_radius = prev_radius + radius_increment;
    //             prev_radius = df;
    //             curr_particles_per_orbit = (curr_particles_per_orbit as f32 * (curr_radius / prev_radius)).floor() as i32;
    //             curr_orbit += 1;
    //         }
    //
    //         let angl_increment = 360. / curr_particles_per_orbit as f32;
    //         let time_elapsed = time.elapsed().as_secs_f32(); // time.delta().as_secs_f32(); // time.elapsed().as_secs_f32();
    //         let particle_in_orbit = i as i32 % curr_particles_per_orbit;
    //         let angle_deg = particle_in_orbit as f32 * angl_increment;
    //
    //         let current_angle = (angle_deg + time_elapsed * rotation_speed).to_radians(); //(speed.0 * rng.gen_range(1.0..3.))).to_radians();
    //         let current_phase = (angle_deg + time_elapsed * phase_speed).to_radians();
    //
    //         //let t_orbit = (i as i32 / curr_particles_per_orbit) + 1;
    //         let mut r = curr_radius; //base_radius+(radius_increment);// * cur_orbit as f32);
    //         if (angle_deg < range) {
    //             let percent = angle_deg / range;
    //             let strength = custom_cos_0_1_0(strength_factor * percent);
    //             r += strength * (current_phase * frequency).sin() * amplitude * curr_orbit as f32; // + rng.gen_range(0.0..100.)
    //         }
    //
    //         let x = current_angle.cos() * r;
    //         let y = current_angle.sin() * r;
    //
    //         let mic_pos =  Vec3::from((x, y, 0.)); // transform.translation +
    //         **mic_target = (transform.translation + mic_pos).xy();
    //     }
    // }

    //     for (&transform, microbes, combat) in character_query.iter() { // (&transform, &wave)
    //     // let waveData = wave;
    //     if combat.0 {
    //         continue
    //     }
    //
    //     let base_radius = 30.;
    //     let radius_increment = 20.;
    //     let rotation_speed = 10.;
    //     let phase_speed = 90.;
    //     let frequency = 0.;
    //     let amplitude = 0.0;
    //     let range = 360.;
    //     let strength_factor = 0.;
    //     let base_particles_per_orbit = 5.;
    //
    //     let mut curr_radius = base_radius;
    //     let mut prev_radius = base_radius;
    //     let mut curr_particles_per_orbit = base_particles_per_orbit as i32;
    //     let mut curr_orbit = 1;
    //     let mut already_done = 0;
    //     for (i, entity_id) in microbes.iter().enumerate() {
    //         match microbes_query.get_mut(*entity_id) {
    //             Ok((mut mic_target)) => {
    //
    //                 if i as i32  % curr_particles_per_orbit == 0 && i > 0 {
    //                     already_done += curr_particles_per_orbit;
    //                     let df = curr_radius;
    //                     curr_radius = prev_radius + radius_increment;
    //                     prev_radius = df;
    //                     curr_particles_per_orbit = (curr_particles_per_orbit as f32 * (curr_radius / prev_radius)).floor() as i32;
    //                     curr_orbit += 1;
    //                 }
    //
    //                 let angl_increment = 360. / curr_particles_per_orbit as f32;
    //                 let time_elapsed = time.elapsed().as_secs_f32(); // time.delta().as_secs_f32(); // time.elapsed().as_secs_f32();
    //                 let particle_in_orbit = i as i32 % curr_particles_per_orbit;
    //                 let angle_deg = particle_in_orbit as f32 * angl_increment;
    //
    //                 let current_angle = (angle_deg + time_elapsed * rotation_speed).to_radians(); //(speed.0 * rng.gen_range(1.0..3.))).to_radians();
    //                 let current_phase = (angle_deg + time_elapsed * phase_speed).to_radians();
    //
    //                 //let t_orbit = (i as i32 / curr_particles_per_orbit) + 1;
    //                 let mut r = curr_radius; //base_radius+(radius_increment);// * cur_orbit as f32);
    //                 if (angle_deg < range) {
    //                     let percent = angle_deg / range;
    //                     let strength = custom_cos_0_1_0(strength_factor * percent);
    //                     r += strength * (current_phase * frequency).sin() * amplitude * curr_orbit as f32; // + rng.gen_range(0.0..100.)
    //                 }
    //
    //                 let x = current_angle.cos() * r;
    //                 let y = current_angle.sin() * r;
    //
    //                 let mic_pos =  Vec3::from((x, y, 0.)); // transform.translation +
    //                 **mic_target = (transform.translation + mic_pos).xy();
    //             }
    //             Err(e) => (),
    //         }
    //     }
    // }
}

//
// pub fn calc_microbes_pos(
//     mut player_query: Query<(&Transform, &Microbes, &CombatState), With<DrawIt>>, // , &Wave
//     mut microbes_query: Query<(&mut Transform, &Speed, &Orbit, &mut Mover), (With<Microbe>, Without<DrawIt>)>, // , &Wave // (With<Microbe>, Without<IsPlayer>)
//     time: Res<Time>,
// ) {
//     for (&transform, microbes, combat) in player_query.iter() { // (&transform, &wave)
//         // let waveData = wave;
//         if combat.0 {
//             continue
//         }
//
//         let mut rng = rand::thread_rng();
//
//         let points_count = microbes.len();
//         let base_radius = 40.;
//         let radius_increment = 15.;
//         let rotation_speed = 25.;
//         let phase_speed = 45.;
//         let frequency = 3.;
//         let amplitude = 10.;
//         let range = 360.;
//         let strength_factor = 1.;
//         let base_particles_per_orbit = 20.;
//
//         let mut cur_orbit = 10.;
//         // let mut test = points_count as f32 / 3.;
//         for (i, entity_id) in microbes.iter().enumerate() {
//             //if let Some(entity) = commands.get_entity(*entity_id) {
//             match microbes_query.get_mut(*entity_id) {
//                 Ok((mut mic_transform, speed, orbit, mut mover)) => {
//                     let particles_per_orbit = base_particles_per_orbit + (radius_increment  / base_radius + 1.);
//                     let angl_increment = 360. / particles_per_orbit;
//                     let time_elapsed = time.elapsed().as_secs_f32(); // time.delta().as_secs_f32(); // time.elapsed().as_secs_f32();
//                     let particle_in_orbit = i as f32 % particles_per_orbit;
//                     let angle_deg = particle_in_orbit as f32 * angl_increment;
//                     // let angle_deg = i as f32 * angl_increment;
//
//                     let current_angle = (angle_deg + time_elapsed * rotation_speed).to_radians(); //(speed.0 * rng.gen_range(1.0..3.))).to_radians();
//                     let current_phase = (angle_deg + time_elapsed * phase_speed).to_radians();
//
//                     cur_orbit += (i as f32 / particles_per_orbit) + 1.;
//                     let mut r = base_radius+(radius_increment);// * cur_orbit as f32);
//                     // let mut r = orbit.0;// (((i as f32 + 1.) / test) as i32 * 50) as f32; //rng.gen_range(50.0..150.); orbit.0;
//                     if (angle_deg < range) {
//                         let percent = angle_deg / range;
//                         let strength = custom_cos_0_1_0(strength_factor * percent);
//                         r += strength * (current_phase * frequency).sin() * amplitude * cur_orbit as f32; // + rng.gen_range(0.0..100.)
//                     }
//
//                     let x = current_angle.cos() * r;
//                     let y = current_angle.sin() * r;
//
//                     let mic_pos =  Vec3::from((x, y, 0.)); // transform.translation +
//                     // painter.transform.translation = mic_transform.translation + mic_pos; //transform.translation
//                     mover.target = (transform.translation + mic_pos).xy();
//                     // mic_transform.translation = transform.translation + mic_pos; //
//                     // painter.transform.translation = mic_transform.translation + transform.translation;
//                     // // microbes_query.get_mut(*entity_id).unwrap().0.translation = mic_pos;
//                     // painter.circle(5.);
//                 }
//                 Err(e) => (),
//             }
//             //}
//         }
//
//         //
//         // for i in 0..points_count {
//         //     let time_elapsed = time.elapsed().as_secs_f32();
//         //     let angle_deg = i as f32 * angl_increment;
//         //
//         //     let current_angle = (angle_deg + time_elapsed * rotation_speed).to_radians();
//         //     let current_phase = (angle_deg + time_elapsed * phase_speed).to_radians();
//         //
//         //     let mut r = base_radius;
//         //     if (angle_deg < range) {
//         //         let percent = angle_deg / range;
//         //         let strength = custom_cos_0_1_0(strength_factor * percent);
//         //         r += strength * (current_phase * frequency).sin() * amplitude;
//         //     }
//         //
//         //     let x = current_angle.cos() * r;
//         //     let y = current_angle.sin() * r;
//         //
//         //     painter.transform.translation = transform.translation + Vec3::from((x, y, 0.));
//         //
//         //     painter.circle(5.);
//         // }
//     }
// }
