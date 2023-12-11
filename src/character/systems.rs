use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::components::*;
use bevy_vector_shapes::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

pub fn game_control(
    // game_status: Res<GameStatus>,
    mut game_status: ResMut<GameStatus>,
    //mut game_reset: ResMut<GameReset>,
    //mut character_init: ResMut<CharacterInit>,
    mut commands: Commands,
    mut ccg: ResMut<CharacterCollisionGroups>,
    query: Query<Entity, With<IsCharacter>>,
    character_stats: Res<CharacterStats>,
    mut player_entity_id: ResMut<PlayerEntityId>,
) {
    match *game_status {
        GameStatus::ResetGame => {
            ccg.0 = u32::MAX;
            for entity in query.iter() {
                commands.entity(entity).despawn_recursive()
            }
            *game_status = GameStatus::SpawnCharacter
        }
        GameStatus::SpawnCharacter => {
            *game_status = GameStatus::ResetMenu;
            let pos = Vec3::from((0., 0., 0.));

            let parent_group = bit_map_group_take(&mut ccg.0);
            let child_group = bit_map_group_take(&mut ccg.0);
            // player
            player_entity_id.0 = commands
                .spawn(RigidBody::KinematicPositionBased)
                .insert(Collider::ball(character_stats.size))
                .insert(Camera2dBundle::default())
                .insert(TransformBundle::from(Transform::from_xyz(
                    pos.x, pos.y, pos.z,
                )))
                .insert(CharacterBundle {
                    health: Health(character_stats.health),
                    speed: Speed(character_stats.speed),
                    to_spawn_mic: Default::default(),
                    draw_stats: DrawStats {
                        radius: character_stats.size,
                        color: Color::GREEN,
                    },
                    combat: CombatState(false),
                    target: Default::default(),
                    energy: Energy(character_stats.energy),
                    count_microbes: Default::default(),
                    skill_cd: Default::default(),
                    skill: Default::default(),
                    is_bot: Default::default(),
                    type_of_entity: TypeOfEntity::Character,
                    character: Default::default(),
                    character_collision_group: CharacterCollisionGroup {
                        parent_id: parent_group,
                        child_id: child_group,
                    },
                    cursor_targets: Default::default(),
                    child_color: ChildColor(Color::GREEN),
                })
                .insert(CollisionGroups::new(
                    Group::from_bits_retain(parent_group),
                    Group::from_bits_retain(u32::MAX & !child_group),
                ))
                .insert(IsPlayer)
                .with_children(|parent| {
                    parent
                        .spawn(())
                        .insert(InvisiblePlaceholder)
                        .insert(Skill::default());
                })
                .id();
        }
        _ => (),
    }
}
pub fn character_spawner(
    mut commands: Commands,
    mut ccg: ResMut<CharacterCollisionGroups>,
    character_stats: Res<CharacterStats>,
    microbe_stats: Res<MicrobeStats>,
    world_size: Res<WorldSize>,
    bots: Query<(&IsBot, &IsCharacter)>,
    game_status: Res<GameStatus>,
    rapier_context: Res<RapierContext>,
) {
    match *game_status {
        GameStatus::Game => (),
        _ => return,
    }

    let mut bot_count = 0;
    for (is_bot, _) in bots.iter() {
        if is_bot.0 {
            bot_count += 1;
        }
    }

    if bot_count < character_stats.max_count_bots {
        let pos = find_free_space(
            &rapier_context,
            microbe_stats.spawn_radius_max,
            world_size.0,
        );

        if pos.is_none() {
            return;
        }

        let parent_group = bit_map_group_take(&mut ccg.0);
        let child_group = bit_map_group_take(&mut ccg.0);

        commands
            .spawn(RigidBody::KinematicPositionBased)
            .insert(Collider::ball(character_stats.size))
            .insert(TransformBundle::from(Transform::from_xyz(
                pos.unwrap().x,
                pos.unwrap().y,
                0.0,
            )))
            .insert(CharacterBundle {
                health: Health(character_stats.health),
                speed: Speed(character_stats.speed),
                to_spawn_mic: ToSpawnMic(true),
                draw_stats: DrawStats {
                    radius: character_stats.size,
                    color: Color::AQUAMARINE,
                },
                combat: CombatState(false),
                target: Default::default(),
                energy: Energy(character_stats.energy),
                count_microbes: Default::default(),
                skill_cd: Default::default(),
                skill: Default::default(),
                is_bot: IsBot(true),
                type_of_entity: TypeOfEntity::Character,
                character: Default::default(),
                character_collision_group: CharacterCollisionGroup {
                    parent_id: parent_group,
                    child_id: child_group,
                },
                cursor_targets: Default::default(),
                child_color: ChildColor(Color::BEIGE),
            })
            .insert(CollisionGroups::new(
                Group::from_bits_retain(parent_group),
                Group::from_bits_retain(u32::MAX & !child_group),
            ))
            .with_children(|parent| {
                parent
                    .spawn(())
                    .insert(InvisiblePlaceholder)
                    .insert(Skill::default());
            });
    }
}

fn find_free_space(
    rapier_context: &Res<RapierContext>,
    radius: f32,
    world_border: f32,
) -> Option<Vec2> {
    let filter = QueryFilter::only_kinematic();
    let mut point = Vec2::new(0.0, 0.0);
    let mut rng = rand::thread_rng();

    let shape = Collider::ball(radius);
    let mut i = 0;
    let mut ready_return = true;

    while i < 100 {
        point.x = rng.gen_range(-world_border..world_border);
        point.y = rng.gen_range(-world_border..world_border);

        rapier_context.intersections_with_shape(point, 0., &shape, filter, |_| {
            ready_return = false;
            false
        });

        if ready_return {
            return Some(point);
        }

        ready_return = true;
        i += 1;
    }

    None
}

pub fn microbes_spawner(
    mut commands: Commands,
    microbe_stats: Res<MicrobeStats>,
    mut characters_query: Query<(
        Entity,
        &mut ToSpawnMic,
        &mut Energy,
        &Children,
        &IsBot,
        &CharacterCollisionGroup,
        &ChildColor,
    )>,
    game_status: Res<GameStatus>,
) {
    match *game_status {
        GameStatus::Game => (),
        _ => return,
    }

    let mut rng = rand::thread_rng();
    for (entity, mut to_spawn_min, mut energy, children, is_bot, ccg, child_color) in
        characters_query.iter_mut()
    {
        if to_spawn_min.0 && energy.0 > microbe_stats.spawn_price {
            while energy.0 > microbe_stats.spawn_price {
                if children.len() >= microbe_stats.max_count as usize {
                    to_spawn_min.0 = false;
                    println!("Too much children");
                    break;
                }
                energy.0 -= microbe_stats.spawn_price;
                //Group::ALL & 1<<1

                // test groups
                let not_interact_with = (u32::MAX & !ccg.parent_id) & !ccg.child_id;
                //not_interact = not_interact & !ccg.child_id;

                let x =
                    rng.gen_range(-microbe_stats.spawn_radius_max..microbe_stats.spawn_radius_max); // transform.translation.x +
                let y =
                    rng.gen_range(-microbe_stats.spawn_radius_max..microbe_stats.spawn_radius_max); // transform.translation.y +
                                                                                                    // u32 && self and parent = all other;
                let mut children_entity_commands = commands.spawn(RigidBody::Dynamic);
                children_entity_commands
                    .insert(Velocity::zero())
                    .insert(Collider::ball(microbe_stats.size))
                    .insert(CollisionGroups::new(
                        Group::from_bits_retain(ccg.child_id),
                        Group::from_bits_retain(not_interact_with),
                    ))
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(TransformBundle::from(Transform::from_xyz(x, y, 0.)))
                    .insert(MicrobeBundle {
                        health: if is_bot.0 {
                            Health(microbe_stats.health)
                        } else {
                            Health(microbe_stats.health / 2.0)
                        },
                        is_microbe: Default::default(),
                        // orbit: Orbit(20. + i as f32),
                        draw_stats: DrawStats {
                            radius: microbe_stats.size,
                            color: child_color.0,
                        },
                        target: Default::default(),
                        // skill_target: Default::default(),
                        // rest_target: Default::default(),
                        // parent_id: ParentEntityID(entity),
                        // targets: Default::default(),
                        type_of_entity: Default::default(),
                        character_collision_group: CharacterCollisionGroup {
                            parent_id: ccg.parent_id,
                            child_id: ccg.child_id,
                        },
                        //is_bot: IsBot(is_bot.0),
                        skill: Default::default(),
                    })
                    .insert(Mover {
                        max_speed: 200.0,
                        max_force: 100.0,
                        vel: Vec2::new(0., 0.),
                        acc: Default::default(),
                        stiffness: 2.0,
                        damper: 1.0,
                        rotation_speed: rng.gen_range(0.5..=2.),
                        angle: rng.gen_range(0.0..=2. * PI), // random(0, 2 * PI)
                        radius: rng.gen_range(
                            microbe_stats.spawn_radius_min..=microbe_stats.spawn_radius_max,
                        ),
                        speed: microbe_stats.speed,
                    })
                    .set_parent(entity);

                //children_entity_commands.set_parent(entity);
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
    mut player: Query<(&Children, &mut OrthographicProjection), With<IsPlayer>>,
    microbe_stats: Res<MicrobeStats>,
) {
    if let Ok((microbes, mut orthographic_projection)) = player.get_single_mut() {
        let scale = (microbes.len() as i32 / microbe_stats.max_count) as f32;
        let min: f32 = 0.1;
        let max: f32 = 1.1;
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

pub fn health_regeneration(
    mut characters: Query<(&mut Health, &TypeOfEntity)>,
    microbe_stats: Res<MicrobeStats>,
    character_stats: Res<CharacterStats>,
    time: Res<Time>,
) {
    for (mut health, type_of_entity) in characters.iter_mut() {
        match type_of_entity {
            TypeOfEntity::Character => {
                health.0 = (health.0
                    + character_stats.regeneration_health_rate_per_sec * time.delta_seconds())
                .min(character_stats.health)
            }
            TypeOfEntity::Microbe => {
                health.0 = (health.0
                    + microbe_stats.regeneration_health_rate_per_sec * time.delta_seconds())
                .min(microbe_stats.health)
            }
        }
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
    characters: Query<(&Skill, &Children, &Transform, &CursorTargets), With<IsCharacter>>, //   &IsBot, is_bot, , Changed<Skill>
    mut microbes_query: Query<(&mut Skill), (With<Microbe>, Without<IsCharacter>)>,
    //cursor: Res<Cursor>,
    microbe_stats: Res<MicrobeStats>,
) {
    for (skill, children, parent_transform, cursor_targets) in characters.iter() {
        let mut skill_to_child: Skill;
        match *skill {
            Skill::Rest(_) => {
                let cur_radius = scale_value(
                    children.len() as f32,
                    microbe_stats.min_count as f32 + 1.,
                    microbe_stats.max_count as f32,
                    microbe_stats.spawn_radius_min + 150.,
                    microbe_stats.spawn_radius_max,
                );
                // println!("curr rad {}", cur_radius);
                skill_to_child = Skill::Rest(cur_radius);
            }
            Skill::TargetAttack(_, _) => {
                let new_tar = match cursor_targets.0 {
                    (Some(target1), None) => Some(target1 - parent_transform.translation.xy()),
                    (None, Some(target2)) => Some(target2 - parent_transform.translation.xy()),
                    (Some(target1), Some(_)) => Some(target1 - parent_transform.translation.xy()),
                    (None, None) => None,
                };

                let damper = 1.0;
                skill_to_child = Skill::TargetAttack(new_tar, damper);
            }
            Skill::FollowCursor(_) => {
                // if !is_bot.0 {
                //     cursor_targets.0.0 = Some(cursor.0 - parent_transform.translation.xy());
                // }

                let new_tar = match cursor_targets.0 {
                    (Some(target1), None) => Some(target1 - parent_transform.translation.xy()),
                    (Some(target1), Some(_)) => Some(target1 - parent_transform.translation.xy()),
                    _ => None,
                };

                skill_to_child = Skill::FollowCursor(new_tar);
            }
            Skill::Patrolling(_, _) => {
                let (new_tar1, new_tar2) = match cursor_targets.0 {
                    (Some(target1), None) => {
                        (
                            Some(target1 - parent_transform.translation.xy()),
                            Some(Vec2::default()),
                        ) // parent pos = Vec2::default() (0.,0.,)
                    }
                    (None, Some(target2)) => {
                        (
                            Some(Vec2::default()), // parent pos = Vec2::default() (0.,0.,)
                            Some(target2 - parent_transform.translation.xy()),
                        )
                    }
                    (Some(target1), Some(target2)) => (
                        Some(target1 - parent_transform.translation.xy()),
                        Some(target2 - parent_transform.translation.xy()),
                    ),
                    (None, None) => (None, None),
                };

                skill_to_child = Skill::Patrolling(new_tar1, new_tar2);
            }
        }
        for child in children.iter() {
            if let Ok(mut microbe_skill) = microbes_query.get_mut(*child) {
                match *skill {
                    Skill::TargetAttack(_, _) => {
                        if *microbe_skill != *skill {
                            *microbe_skill = skill_to_child;
                        }
                    }
                    _ => *microbe_skill = skill_to_child,
                }
            }
        }
    }
}

// value that scale factor
// start1 and stop1 - its first part min max range
// start2 and stop2 - its second part min max the range scale
fn scale_value(value: f32, start1: f32, stop1: f32, start2: f32, stop2: f32) -> f32 {
    start2 + (stop2 - start2) * ((value - start1) / (stop1 - start1))
}

pub fn seek_system(
    mut microbes_query: Query<
        (
            &Transform,
            &Skill,
            // &mut Targets,
            &mut Target,
            &mut Mover,
            &mut Velocity,
            // &IsBot,
        ),
        With<Microbe>,
    >,
    // characters_query: Query<(&Target, &CombatState, &Transform), Without<Microbe>>,
    time: Res<Time>,
    // cursor: Res<Cursor>,
    microbe_stats: Res<MicrobeStats>,
    // mut painter: ShapePainter,
) {
    let max_dist_to_points = 5.;
    let mut force: Vec2 = Vec2::default();
    let mut rng = rand::thread_rng();
    for (microbe_transform, skill, mut microbe_target, mut mover, mut vel) in //  is_bot
        microbes_query.iter_mut()
    {
        match skill {
            Skill::Rest(_) => {
                let cur_angle = microbe_transform
                    .translation
                    .y
                    .atan2(microbe_transform.translation.x);
                mover.radius = (mover.radius + rng.gen_range(-3..=3) as f32).clamp(
                    microbe_stats.spawn_radius_min,
                    microbe_stats.spawn_radius_max,
                ); // 150.;// *max_radius

                let speed_factor = scale_value(
                    mover.radius,
                    microbe_stats.spawn_radius_min,
                    microbe_stats.spawn_radius_max,
                    //*max_radius,
                    1.,
                    0.1,
                );

                mover.angle =
                    (cur_angle + PI / 12. * (mover.rotation_speed * speed_factor)) % (PI * 2.); // mover.rotation_speed rng.gen_range(0.0..PI/12.) * time.delta_seconds()
                microbe_target.x = mover.radius * mover.angle.cos(); // mover.radius mover.angle.cos()
                microbe_target.y = mover.radius * mover.angle.sin(); // (mover.speed * time.delta().as_secs_f32())
                let displacement = microbe_target.0 - microbe_transform.translation.xy();
                force = mover.stiffness * displacement - mover.damper * mover.vel;
            }
            Skill::Patrolling(tar1, tar2) => {
                if let (Some(target1), Some(target2)) = (tar1, tar2) {
                    if microbe_target.0 != *target1 && microbe_target.0 != *target2 {
                        microbe_target.0 = *target1
                    }

                    if (microbe_target.0 == *target1)
                        && microbe_transform.translation.xy().distance(*target1)
                            < max_dist_to_points
                    {
                        microbe_target.0 = *target2
                    }

                    if (microbe_target.0 == *target2)
                        && microbe_transform.translation.xy().distance(*target2)
                            < max_dist_to_points
                    {
                        microbe_target.0 = *target1
                    }

                    let mut desired = microbe_target.0 - microbe_transform.translation.xy();
                    desired = desired.normalize_or_zero() * mover.max_speed;
                    force = desired - mover.vel;
                    force.clamp_length(0., mover.max_force);
                } else {
                    force = force * 0.;
                }
            }
            Skill::FollowCursor(tar1) => {
                if let Some(new_target) = tar1 {
                    microbe_target.0 = *new_target;
                }

                let mut desired = microbe_target.0 - microbe_transform.translation.xy();
                desired = desired.normalize_or_zero() * mover.max_speed;
                force = desired - mover.vel;
                force.clamp_length(0., mover.max_force);
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

pub fn draw_entities(mut painter: ShapePainter, draw_data: Query<(&GlobalTransform, &DrawStats)>) {
    for (transform, draw_stats) in draw_data.iter() {
        painter.color = draw_stats.color;
        painter.transform.translation = transform.translation();
        painter.circle(draw_stats.radius);
    }
}

pub fn collision_events_handler(
    mut collision_events: EventReader<CollisionEvent>,
    mut entities_query: Query<(&mut Health, &TypeOfEntity, &CharacterCollisionGroup)>, // Entity Some(entity, mut health, collider_handel)
    mut cmd: Commands,
    mut ccg: ResMut<CharacterCollisionGroups>,
    mut game_status: ResMut<GameStatus>,
    mut player_entity_id: ResMut<PlayerEntityId>,
) {
    match *game_status {
        GameStatus::Game => (),
        _ => return,
    }

    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(collider1, collider2, _) => {
                let health_entity_one =
                    if let Ok((entity_health, _, _)) = entities_query.get(*collider1) {
                        entity_health.0
                    } else {
                        0.
                    };

                let health_entity_two =
                    if let Ok((entity_health, _, _)) = entities_query.get(*collider2) {
                        entity_health.0
                    } else {
                        0.
                    };

                if let Ok((mut my_health, my_type, character_collision_group)) =
                    entities_query.get_mut(*collider1)
                {
                    my_health.0 = my_health.0 - health_entity_two;

                    let no_health = my_health.0 <= 0.0;

                    match (no_health, my_type) {
                        (true, TypeOfEntity::Character) => {
                            bit_map_group_back(&mut ccg.0, character_collision_group.parent_id);
                            bit_map_group_back(&mut ccg.0, character_collision_group.child_id);
                            if player_entity_id.0.eq(collider1) {
                                *game_status = GameStatus::SpawnMenu;
                            }
                            cmd.entity(*collider1).despawn_recursive()
                        }
                        (true, TypeOfEntity::Microbe) => cmd.entity(*collider1).despawn_recursive(),
                        _ => (),
                    }
                }

                if let Ok((mut my_health, my_type, character_collision_group)) =
                    entities_query.get_mut(*collider2)
                {
                    my_health.0 = my_health.0 - health_entity_one;

                    let no_health = my_health.0 <= 0.0;
                    match (no_health, my_type) {
                        (true, TypeOfEntity::Character) => {
                            bit_map_group_back(&mut ccg.0, character_collision_group.parent_id);
                            bit_map_group_back(&mut ccg.0, character_collision_group.child_id);

                            if player_entity_id.0.eq(collider2) {
                                *game_status = GameStatus::SpawnMenu;
                            }
                            cmd.entity(*collider2).despawn_recursive()
                        }
                        (true, TypeOfEntity::Microbe) => cmd.entity(*collider2).despawn_recursive(),
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
}

pub fn draw_entities_points(
    mut painter: ShapePainter,
    player: Query<(&CursorTargets), With<IsPlayer>>,
) {
    let point_one_color = Color::rgb(219f32, 49f32, 15f32);
    let point_two_color = Color::rgb(13f32, 10f32, 201f32);
    if let Ok(cursor_targets) = player.get_single() {
        match cursor_targets.0 {
            (Some(point1), Some(point2)) => {
                painter.color = point_one_color;
                painter.transform.translation = point1.extend(0.);
                painter.circle(1.);

                painter.color = point_two_color;
                painter.transform.translation = point2.extend(0.);
                painter.circle(1.);
            }
            (None, Some(point2)) => {
                //painter.transform.translation = point1.extend(0.);
                painter.color = point_two_color;
                painter.transform.translation = point2.extend(0.);
                painter.circle(1.);
            }
            (Some(point1), None) => {
                painter.color = point_one_color;
                painter.transform.translation = point1.extend(0.);
                //painter.transform.translation = point2.extend(0.);
                painter.circle(1.);
            }
            _ => (),
        }
    }
}

// fn custom_cos_0_1_0(x: f32) -> f32 {
//     0.5 * (1.0 + (x * 2.0 * PI + PI).cos())
// }

fn bit_map_group_take(store: &mut u32) -> u32 {
    // &mut u32
    for i in 0..32u32 {
        if *store & (1 << i) != 0 {
            *store = *store & !1 << i;
            return 1u32 << i;
        }
    }
    0u32
}

fn bit_map_group_back(store: &mut u32, group: u32) {
    *store = *store | group
}

// fn text_update_system(
//     diagnostics: Res<DiagnosticsStore>,
//     mut query: Query<&mut Text, With<FpsText>>,
// ) {
//     for mut text in &mut query {
//         if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
//             if let Some(value) = fps.smoothed() {
//                 // Update the value of the second section
//                 text.sections[1].value = format!("{value:.2}");
//             }
//         }
//     }
// }

pub fn test_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    // FPS
    // #[cfg(feature = "default_font")]

    let font_size = 24.0;

    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_section(
            "FPS: ",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size,
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            // bottom:
            // right: ,
            ..default()
        }),
        FpsText,
    ));

    // EntitiesInWorld
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_section(
            "Entities: ",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size,
                color: Color::GOLD,
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(30.0),
            left: Val::Px(5.0),
            // bottom:
            // right: ,
            ..default()
        }),
        EntitiesInWorld,
    ));

    // DestroyedMicrobes
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_section(
            "Destroyed Microbes: ",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size,
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(55.0),
            left: Val::Px(5.0),
            // bottom:
            // right: ,
            ..default()
        }),
        TextStates::DestroyedMicrobes,
        // DestroyedMicrobes,
    ));

    // DestroyedCharacters
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_section(
            "Destroyed Characters: ",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size,
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(80.0),
            left: Val::Px(5.0),
            // bottom:
            // right: ,
            ..default()
        }),
        TextStates::DestroyedCharacters,
        //DestroyedCharacters,
    ));

    // CharacterEnergy
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_section(
            "Character Energy: ",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size,
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(105.0),
            left: Val::Px(5.0),
            // bottom:
            // right: ,
            ..default()
        }),
        TextStates::CharacterEnergy,
        //CharacterEnergy,
    ));

    // CharacterHealth
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_section(
            "Character Health: ",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size,
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(130.0),
            left: Val::Px(5.0),
            // bottom:
            // right: ,
            ..default()
        }),
        TextStates::CharacterHealth,
        //CharacterHealth,
    ));

    // Population
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_section(
            "Character Health: ",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size,
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(155.0),
            left: Val::Px(5.0),
            // bottom:
            // right: ,
            ..default()
        }),
        TextStates::YourPopulation,
        //YourPopulation,
    ));

    // SkillCd
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_section(
            "Skills Cooldown: ",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size,
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(180.0),
            left: Val::Px(5.0),
            // bottom:
            // right: ,
            ..default()
        }),
        TextStates::SkillCd,
        //YourPopulation,
    ));
}

pub fn fps_display(mut query: Query<&mut Text, With<FpsText>>, time: Res<Time>) {
    for mut text in &mut query {
        //println!("test: {:02}", );
        let fps = 1.0 / time.delta_seconds();
        if let Some(mut t) = text.sections.first_mut() {
            t.value = format!("FPS: {:#0.0}", fps);
        }
    }
}

pub fn entities_count_display(
    entities: Query<Entity>,
    mut query: Query<&mut Text, With<EntitiesInWorld>>,
    // time: Res<Time>,
) {
    for mut text in &mut query {
        if let Some(mut t) = text.sections.first_mut() {
            t.value = format!("Entities: {:#0.0}", entities.iter().count());
        }
    }
}

pub fn display_player_state(
    mut query: Query<(&mut Text, &TextStates), With<TextStates>>,
    player: Query<(&Health, &Energy, &Children, &SkillCd), With<IsPlayer>>,
) {
    let (health, energy, population, destroyed_microbes, destroyed_characters, skill_cd) =
        if let Ok((health, energy, population, skill_cd)) = player.get_single() {
            (
                health.0,
                energy.0,
                (population.len() - 1) as f32,
                0.0,
                0.0,
                skill_cd.0.percent_left() * 100.,
            )
        } else {
            (0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
        };
    for (mut text_module, text_state) in query.iter_mut() {
        match (text_module.sections.first_mut(), text_state) {
            (Some(text), TextStates::CharacterHealth) => {
                text.value = format!("Health: {:#0.0}", health)
            }
            (Some(text), TextStates::CharacterEnergy) => {
                text.value = format!("Energy: {:#0.0}", energy)
            }
            (Some(text), TextStates::YourPopulation) => {
                text.value = format!("Your Population: {:#0.0}", population)
            }
            (Some(text), TextStates::DestroyedMicrobes) => {
                text.value = format!("Destroyed Microbes: {:#0.0}", destroyed_microbes)
            }
            (Some(text), TextStates::DestroyedCharacters) => {
                text.value = format!("Destroyed Characters: {:#0.0}", destroyed_characters)
            }
            (Some(text), TextStates::SkillCd) => {
                text.value = format!("Skills Cooldown: {:#0.0}%", skill_cd)
            }
            _ => (),
        }
    }
}
