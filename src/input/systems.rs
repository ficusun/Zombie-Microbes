use bevy::prelude::*;
use bevy::time::TimerMode::Repeating;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

use crate::character::components::{CombatState, Skill, SkillCd, SkillsCd, Target, ToSpawnMic};
use crate::character::components::{CursorTargets, IsPlayer};
use crate::input::components::Cursor;

pub fn keyboard_input_system(
    mut curr: ResMut<Cursor>,
    skills_cd: Res<SkillsCd>,
    keyboard_input: Res<Input<KeyCode>>,
    mut players_transform: Query<
        (
            &mut Transform,
            &mut CursorTargets,
            //&mut Target,
            &mut ToSpawnMic,
            &mut Skill,
            &mut SkillCd,
        ),
        With<IsPlayer>,
    >,
    //mut players_transform: Query<&mut KinematicCharacterController, With<IsPlayer>>,
) {
    let mut vel = Vec3::default();

    if let Ok((
        mut player_transform,
        mut cursor_targets,
        //mut target,
        mut toSpawn,
        mut skill,
        mut skill_cd,
    )) = players_transform.get_single_mut()
    {
        if keyboard_input.pressed(KeyCode::W) {
            vel.y += 1.;
        }

        if keyboard_input.pressed(KeyCode::A) {
            vel.x -= 1.;
        }

        if keyboard_input.pressed(KeyCode::D) {
            vel.x += 1.;
        }

        if keyboard_input.pressed(KeyCode::S) {
            vel.y -= 1.;
            //println!("DAAAAAA")
        }

        player_transform.translation += vel.normalize_or_zero();

        let point1 = cursor_targets.0.0.is_some();
        let point2 = cursor_targets.0.1.is_some();
        println!("{}, {}", point1, point2);
        if !(!skill_cd.0.finished() && (!point1 || !point2)) {
            return
        }

        // combat.0 = false;
        //
        // if keyboard_input.pressed(KeyCode::E) {
        //     combat.0 = true;
        //     target.0 = curr.0;
        //     //println!("{}", curr.0)
        // }

        if keyboard_input.pressed(KeyCode::Key1) {
            println!("FollowCursor start");
            skill_cd
                .0
                .set_duration(Duration::from_secs_f32(skills_cd.follow_cursor));
            skill_cd.0.reset();
            *skill = Skill::FollowCursor(Some(curr.0)); //  Some(player_transform.translation.xy()) (Timer::from_seconds(skills_cd.follow_cursor, Repeating))
        }

        if keyboard_input.pressed(KeyCode::Key2) {
            println!("TargetAttack start");
            skill_cd
                .0
                .set_duration(Duration::from_secs_f32(skills_cd.target_attack));
            skill_cd.0.reset();
            *skill = Skill::TargetAttack(Some(curr.0), 1.0); // (Timer::from_seconds(skills_cd.follow_cursor, Repeating))
        }

        if keyboard_input.just_pressed(KeyCode::Q) {
            println!("press Q: {}", toSpawn.0);
            toSpawn.0 = true;
            // target.0 = curr.0;
            println!("press Q: {}", toSpawn.0);
        }
    }
}

pub fn mouse_click_system(
    curr: Res<Cursor>,
    mouse: Res<Input<MouseButton>>,
    mut player: Query<(&mut CursorTargets), With<IsPlayer>>,
) {
    if let Ok((mut cursor_targets)) = player.get_single_mut() {
        if mouse.just_pressed(MouseButton::Left) {
            if cursor_targets.0 .0.is_some() {
                cursor_targets.0 .0 = None
            } else {
                cursor_targets.0 .0 = Some(curr.0)
            }
        }

        if mouse.just_pressed(MouseButton::Right) {
            if cursor_targets.0 .1.is_some() {
                cursor_targets.0 .1 = None
            } else {
                cursor_targets.0 .1 = Some(curr.0)
            }
        }
    }
}

// fn cursor_convert_pos_to_world(window: &Window) -> Option<Vec2> {
//     if let Some(pos) = window.physical_cursor_position() {
//         let size = Vec2::new(
//             window.physical_width() as f32,
//             window.physical_height() as f32,
//         );
//
//         // Convert cursor pos to the world
//         let world_pos = Vec2::new(pos.x - size.x / 2.0, pos.y - size.y / 2.0);
//         return Some(world_pos);
//     }
//
//     None
// }

pub fn mouse_input_system(
    mut curr: ResMut<Cursor>,
    windows: Query<&Window>,
    player: Query<(&Transform, &Camera), With<IsPlayer>>,
) {
    if let (Ok(mut window), Ok(mut player)) = (windows.get_single(), player.get_single()) {
        // if let Some(t) = cursor_convert_pos_to_world(window) {
        //     curr.0 = t;
        // }
        let c = window
            .cursor_position()
            .map_or(Vec2::new(0., 0.), |cur| cur);

        let window_size = Vec2::new(
            window.physical_width() as f32,
            window.physical_height() as f32,
        );
        let ndc_to_world = player.0.compute_matrix() * player.1.projection_matrix().inverse();
        let ndc = Vec2::new(c.x / window_size.x, 1.0 - c.y / window_size.y) * 2.0 - Vec2::ONE;
        // let ndc = (window.cursor_position().map_or(Vec2::new(0., 0.), |cur| cur) / window_size) * 2.0 - Vec2::ONE;
        curr.0 = ndc_to_world.project_point3(ndc.extend(0.0)).xy();
    }
}
