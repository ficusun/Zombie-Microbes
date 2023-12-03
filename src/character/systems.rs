use bevy::math::vec3;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::components::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_vector_shapes::prelude::*;

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

pub fn draw_characters (mut painter: ShapePainter, mut players_transform: Query<&Transform, With<IsPlayer>>) {
    for transform in players_transform.iter() {
        painter.transform.translation = transform.translation;
        painter.cap = Cap::Round;
        painter.color = Color::GREEN;
        painter.thickness_type = ThicknessType::Pixels;
        painter.thickness = 1.;
        let fdsf:i32 = 4;
        I32
        let re = fdsf.sin();
        println!("{}", re);
        // painter.line(Vec3::default(), transform.translation);
        painter.arc(15., 0., 3.14 * 2.);
        //painter.circle(15.);
        // painter.reset = true;
    }
}
