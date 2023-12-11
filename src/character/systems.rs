use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::components::*;
use bevy_vector_shapes::prelude::*;
use rand::Rng;
use std::f32::consts::PI;
use std::time::Duration;

pub fn game_control(
    mut game_status: ResMut<GameStatus>,
    mut commands: Commands,
    mut ccg: ResMut<CharacterCollisionGroups>,
    query: Query<Entity, With<IsCharacter>>,
    character_stats: Res<CharacterStats>,
    mut player_entity_id: ResMut<PlayerEntityId>,
    mut destroyed_entities: ResMut<DestroyedEntities>,
) {
    match *game_status {
        GameStatus::ResetGame => {
            destroyed_entities.destroyed_microbes = 0;
            destroyed_entities.destroyed_characters = 0;
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
                    parent_id: ParentEntityID(None),
                    cursor_targets: Default::default(),
                    child_color: ChildColor(Color::GREEN),
                    character_power: Default::default(),
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
                parent_id: ParentEntityID(None),
                cursor_targets: Default::default(),
                child_color: ChildColor(Color::BEIGE),
                character_power: Default::default(),
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
                    break;
                }
                energy.0 -= microbe_stats.spawn_price;

                let not_interact_with = (u32::MAX & !ccg.parent_id) & !ccg.child_id;

                let x =
                    rng.gen_range(-microbe_stats.spawn_radius_max..microbe_stats.spawn_radius_max);
                let y =
                    rng.gen_range(-microbe_stats.spawn_radius_max..microbe_stats.spawn_radius_max);

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
                        draw_stats: DrawStats {
                            radius: microbe_stats.size,
                            color: child_color.0,
                        },
                        target: Default::default(),
                        type_of_entity: Default::default(),
                        character_collision_group: CharacterCollisionGroup {
                            parent_id: ccg.parent_id,
                            child_id: ccg.child_id,
                        },
                        energy: Default::default(),
                        parent_id: ParentEntityID(Some(entity)),
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
                        angle: rng.gen_range(0.0..=2. * PI),
                        radius: rng.gen_range(
                            microbe_stats.spawn_radius_min..=microbe_stats.spawn_radius_max,
                        ),
                        speed: microbe_stats.speed,
                    })
                    .set_parent(entity);

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
        orthographic_projection.scale = min + scale * (max - min);
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

pub fn calc_power (
    mut characters: Query<(&Children, &IsBot, &mut CharacterPower), With<IsCharacter>>,
    microbe_stats: Res<MicrobeStats>,
) {
    for (children, is_bot, mut power) in characters.iter_mut() {
        let mut p = 0f32;
        if is_bot.0 {
            p = children.len() as f32 * microbe_stats.health;
        } else {
            p = children.len() as f32 * (microbe_stats.health / 2.);
        }

        power.0 = p;
    }
}

pub fn skill_to_children(
    characters: Query<(&Skill, &Children, &Transform, &CursorTargets), With<IsCharacter>>,
    mut microbes_query: Query<(&mut Skill), (With<Microbe>, Without<IsCharacter>)>,
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
                        )
                    }
                    (None, Some(target2)) => {
                        (
                            Some(Vec2::default()),
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
            &mut Target,
            &mut Mover,
            &mut Velocity,
        ),
        With<Microbe>,
    >,
    time: Res<Time>,
    microbe_stats: Res<MicrobeStats>,
) {
    let max_dist_to_points = 5.;
    let mut force: Vec2 = Vec2::default();
    let mut rng = rand::thread_rng();
    for (microbe_transform, skill, mut microbe_target, mut mover, mut vel) in
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
                );

                let speed_factor = scale_value(
                    mover.radius,
                    microbe_stats.spawn_radius_min,
                    microbe_stats.spawn_radius_max,
                    1.,
                    0.1,
                );

                mover.angle =
                    (cur_angle + PI / 12. * (mover.rotation_speed * speed_factor)) % (PI * 2.);
                microbe_target.x = mover.radius * mover.angle.cos();
                microbe_target.y = mover.radius * mover.angle.sin();
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
    mut entities_query: Query<(
        &mut Health,
        &TypeOfEntity,
        &CharacterCollisionGroup,
        &ParentEntityID,
        &mut Energy,
    )>,
    mut cmd: Commands,
    mut ccg: ResMut<CharacterCollisionGroups>,
    mut game_status: ResMut<GameStatus>,
    mut player_entity_id: ResMut<PlayerEntityId>,
    mut character_energy_stats: Res<CharacterEnergyStats>,
    mut destroyed_entities: ResMut<DestroyedEntities>,
) {
    match *game_status {
        GameStatus::Game => (),
        _ => return,
    }

    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(collider1, collider2, _) => {
                let health_entity_one =
                    if let Ok((entity_health, m_t, _, p_i, _)) = entities_query.get(*collider1) {
                        let id = if let Some(id) = p_i.0 {
                            id
                        } else {
                            Entity::from_bits(u64::MAX)
                        };
                        (true, entity_health.0, m_t.eq(&TypeOfEntity::Character), id)
                    } else {
                        (false, 0.0, false, Entity::from_bits(u64::MAX))
                    };

                let health_entity_two =
                    if let Ok((entity_health, m_t, _, p_i, _)) = entities_query.get(*collider2) {
                        let id = if let Some(id) = p_i.0 {
                            id
                        } else {
                            Entity::from_bits(u64::MAX)
                        };
                        (true, entity_health.0, m_t.eq(&TypeOfEntity::Character), id)
                    } else {
                        (false, 0.0, false, Entity::from_bits(u64::MAX))
                    };

                if let Ok((
                    mut my_health,
                    my_type,
                    character_collision_group,
                    parent_entity_id,
                    mut energy,
                )) = entities_query.get_mut(*collider1)
                {
                    my_health.0 = my_health.0 - health_entity_two.1;

                    let no_health = my_health.0 <= 0.0;

                    match (no_health, my_type) {
                        (true, TypeOfEntity::Character) => {
                            bit_map_group_back(&mut ccg.0, character_collision_group.parent_id);
                            bit_map_group_back(&mut ccg.0, character_collision_group.child_id);

                            // when player dead
                            if player_entity_id.0.eq(collider1) {
                                *game_status = GameStatus::SpawnMenu;
                            }

                            // when player kill character
                            if health_entity_two.3.eq(&player_entity_id.0) {
                                destroyed_entities.destroyed_characters += 1;
                            }

                            cmd.entity(*collider1).despawn_recursive()
                        }
                        (true, TypeOfEntity::Microbe) => {

                            // when player microbe die
                            if let Some(per_ent_id) = parent_entity_id.0 {
                                if player_entity_id.0.eq(&per_ent_id) {
                                    energy.0 += (character_energy_stats
                                        .character_microbes_die_energy_back)
                                        .min(character_energy_stats.max_count)
                                }
                            }

                            // when player kill microbe
                            if player_entity_id.0.eq(&collider2) {
                                destroyed_entities.destroyed_microbes += 1;
                                energy.0 += (character_energy_stats
                                    .enemy_microbes_kill_energy_reward)
                                    .min(character_energy_stats.max_count)
                            }

                            // when player kill microbe
                            if player_entity_id.0.eq(&health_entity_two.3) {
                                destroyed_entities.destroyed_microbes += 1;
                                energy.0 += (character_energy_stats
                                    .enemy_microbes_kill_energy_reward)
                                    .min(character_energy_stats.max_count)
                            }

                            cmd.entity(*collider1).despawn_recursive();
                        }
                        _ => (),
                    }
                }

                if let Ok((
                    mut my_health,
                    my_type,
                    character_collision_group,
                    parent_entity_id,
                    mut energy,
                )) = entities_query.get_mut(*collider2)
                {
                    my_health.0 = my_health.0 - health_entity_one.1;

                    let no_health = my_health.0 <= 0.0;
                    match (no_health, my_type) {
                        (true, TypeOfEntity::Character) => {
                            bit_map_group_back(&mut ccg.0, character_collision_group.parent_id);
                            bit_map_group_back(&mut ccg.0, character_collision_group.child_id);

                            // when player die
                            if player_entity_id.0.eq(collider2) {
                                *game_status = GameStatus::SpawnMenu;
                            }

                            // when player kill character
                            if health_entity_one.3.eq(&player_entity_id.0) {
                                destroyed_entities.destroyed_characters += 1;
                            }

                            cmd.entity(*collider2).despawn_recursive()
                        }
                        (true, TypeOfEntity::Microbe) => {

                            // when player microbe die
                            if let Some(per_ent_id) = parent_entity_id.0 {
                                if player_entity_id.0.eq(&per_ent_id) {
                                    energy.0 += (character_energy_stats
                                        .character_microbes_die_energy_back)
                                        .min(character_energy_stats.max_count)
                                }
                            }

                            // when player kill microbe
                            if player_entity_id.0.eq(&collider1) {
                                destroyed_entities.destroyed_microbes += 1;
                                energy.0 += (character_energy_stats
                                    .enemy_microbes_kill_energy_reward)
                                    .min(character_energy_stats.max_count)
                            }

                            // when player kill microbe
                            if player_entity_id.0.eq(&health_entity_one.3) {
                                destroyed_entities.destroyed_microbes += 1;
                                energy.0 += (character_energy_stats
                                    .enemy_microbes_kill_energy_reward)
                                    .min(character_energy_stats.max_count)
                            }

                            cmd.entity(*collider2).despawn_recursive()
                        }
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
                painter.color = point_two_color;
                painter.transform.translation = point2.extend(0.);
                painter.circle(1.);
            }
            (Some(point1), None) => {
                painter.color = point_one_color;
                painter.transform.translation = point1.extend(0.);
                painter.circle(1.);
            }
            _ => (),
        }
    }
}

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

pub fn spawn_all_text(mut commands: Commands, asset_server: Res<AssetServer>) {

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
            ..default()
        }),
        TextStates::SkillCd,
    ));
    // Power
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_section(
            "Power: ",
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
            top: Val::Px(205.0),
            left: Val::Px(5.0),
            ..default()
        }),
        TextStates::Power,
    ));
}

pub fn fps_display(mut query: Query<&mut Text, With<FpsText>>, time: Res<Time>) {
    for mut text in &mut query {
        let fps = 1.0 / time.delta_seconds();
        if let Some(mut t) = text.sections.first_mut() {
            t.value = format!("FPS: {:#0.0}", fps);
        }
    }
}

pub fn entities_count_display(
    entities: Query<Entity>,
    mut query: Query<&mut Text, With<EntitiesInWorld>>,
) {
    for mut text in &mut query {
        if let Some(mut t) = text.sections.first_mut() {
            t.value = format!("Entities: {:#0.0}", entities.iter().count());
        }
    }
}

pub fn display_player_state(
    mut query: Query<(&mut Text, &TextStates), With<TextStates>>,
    player: Query<(&Health, &Energy, &Children, &SkillCd, &CharacterPower), With<IsPlayer>>,
    mut game_status: ResMut<GameStatus>,
    destroyed_entities: Res<DestroyedEntities>,
) {
    match *game_status {
        GameStatus::Game => (),
        _ => return,
    }

    let (health, energy, population, destroyed_microbes, destroyed_characters, skill_cd, power) =
        if let Ok((health, energy, population, skill_cd, char_power)) = player.get_single() {
            (
                health.0,
                energy.0,
                (population.len() - 1) as f32,
                destroyed_entities.destroyed_microbes as f32,
                destroyed_entities.destroyed_characters as f32,
                skill_cd.0.percent_left() * 100.,
                char_power.0,
            )
        } else {
            (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
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
            },
            (Some(text), TextStates::Power) => {
                text.value = format!("Power: {:#0.0}", power)
            }
            _ => (),
        }
    }
}


pub fn bots (
    mut characters: Query<(&mut Transform, &mut Energy, &mut ToSpawnMic, &mut SkillCd, &mut Skill, &mut CursorTargets, &CharacterPower, &Speed), (With<IsCharacter>, Without<IsPlayer>)>,
    mut player: Query<(&Transform, &CharacterPower), With<IsPlayer>>,
    time: Res<Time>,
    skills_cd: Res<SkillsCd>,
    microbe_stats: Res<MicrobeStats>,
) {
    if let Ok((p_trans, p_power)) = player.get_single() {
        for (mut transform,
            mut energy,
            mut toSpawnMic,
            mut skillCd,
            mut skill,
            mut cursorTargets,
            characterPower,
            speed
        ) in characters.iter_mut() {
            let new_pos = (p_trans.translation - transform.translation).normalize_or_zero() * speed.0 * time.delta_seconds();
            transform.translation += new_pos;

            let dis = p_trans.translation.distance(transform.translation);

            if time.elapsed_seconds() as i32 % 6 == 0 {
                toSpawnMic.0 = true;
            }

            if dis <= microbe_stats.spawn_radius_max {
                cursorTargets.0.0 = Some(p_trans.translation.xy());

                if !skillCd.0.finished() {
                    return
                }

                if (time.elapsed_seconds() as i32 % skills_cd.target_attack as i32 == 0) && (energy.0 > 150.0) {
                    energy.0 -= 100.0;
                    *skill = Skill::TargetAttack(None, 2.);
                    skillCd
                        .0
                        .set_duration(Duration::from_secs_f32(skills_cd.target_attack));
                    skillCd.0.reset();
                }

                if (time.elapsed_seconds() as i32 % skills_cd.follow_cursor as i32 == 0) && (energy.0 > 150.0) {
                    energy.0 -= 100.0;
                    *skill = Skill::FollowCursor(None);
                    skillCd
                        .0
                        .set_duration(Duration::from_secs_f32(skills_cd.follow_cursor));
                    skillCd.0.reset();
                }

                if (time.elapsed_seconds() as i32 % skills_cd.patrolling as i32 == 0) && (energy.0 > 150.0) {
                    energy.0 -= 100.0;
                    *skill = Skill::Patrolling(None, None);
                    skillCd
                        .0
                        .set_duration(Duration::from_secs_f32(skills_cd.patrolling));
                    skillCd.0.reset();
                }
            }
        }
    }
}