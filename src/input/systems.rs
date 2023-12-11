use bevy::prelude::*;
use std::time::Duration;

use crate::character::components::{CursorTargets, IsPlayer, WorldSize};
use crate::character::components::{Skill, SkillCd, SkillsCd, ToSpawnMic};
use crate::input::components::Cursor;

pub fn keyboard_input_system(
    skills_cd: Res<SkillsCd>,
    keyboard_input: Res<Input<KeyCode>>,
    mut players_transform: Query<
        (
            &mut Transform,
            &mut CursorTargets,
            &mut ToSpawnMic,
            &mut Skill,
            &mut SkillCd,
        ),
        With<IsPlayer>,
    >,
    world_size: Res<WorldSize>,
) {
    let mut vel = Vec3::default();

    if let Ok((
        mut player_transform,
        cursor_targets,
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

        let new_pos = player_transform.translation + vel.normalize_or_zero();

        let mut set_new_pos = true;
        if new_pos.y < -world_size.0 || new_pos.y > world_size.0 {
            set_new_pos = false;
        }
        if new_pos.x < -world_size.0 || new_pos.x > world_size.0 {
            set_new_pos = false;
        }

        if set_new_pos {
            player_transform.translation = new_pos;
        }

        if keyboard_input.just_pressed(KeyCode::Q) {
            println!("Spawn microbes");
            toSpawn.0 = true;
        }

        let points_present = cursor_targets.0 .0.is_some() || cursor_targets.0 .1.is_some();
        let skill_available = skill_cd.0.finished();

        // when skill unavailable or no points on the visible map character can't do magic
        if !skill_available || !points_present {
            return;
        }

        if keyboard_input.pressed(KeyCode::Key1) {
            println!("FollowCursor start");
            skill_cd
                .0
                .set_duration(Duration::from_secs_f32(skills_cd.follow_cursor));
            skill_cd.0.reset();
            *skill = Skill::FollowCursor(None); //  Some(player_transform.translation.xy()) (Timer::from_seconds(skills_cd.follow_cursor, Repeating))
        }

        if keyboard_input.pressed(KeyCode::Key2) {
            println!("TargetAttack start");
            skill_cd
                .0
                .set_duration(Duration::from_secs_f32(skills_cd.target_attack));
            skill_cd.0.reset();
            *skill = Skill::TargetAttack(None, 0.); // (Timer::from_seconds(skills_cd.follow_cursor, Repeating))
        }

        if keyboard_input.pressed(KeyCode::Key3) {
            println!("Patrolling start");
            skill_cd
                .0
                .set_duration(Duration::from_secs_f32(skills_cd.patrolling));
            skill_cd.0.reset();
            *skill = Skill::Patrolling(None, None); // (Timer::from_seconds(skills_cd.follow_cursor, Repeating))
        }
    }
}

pub fn mouse_click_system(
    curr: Res<Cursor>,
    mouse: Res<Input<MouseButton>>,
    mut player: Query<&mut CursorTargets, With<IsPlayer>>,
) {
    if let Ok(mut cursor_targets) = player.get_single_mut() {
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

pub fn mouse_input_system(
    mut curr: ResMut<Cursor>,
    windows: Query<&Window>,
    player: Query<(&Transform, &Camera), With<IsPlayer>>,
) {
    if let (Ok(window), Ok(player)) = (windows.get_single(), player.get_single()) {
        let c = window
            .cursor_position()
            .map_or(Vec2::new(0., 0.), |cur| cur);

        let window_size = Vec2::new(
            window.physical_width() as f32,
            window.physical_height() as f32,
        );
        let ndc_to_world = player.0.compute_matrix() * player.1.projection_matrix().inverse();
        let ndc = Vec2::new(c.x / window_size.x, 1.0 - c.y / window_size.y) * 2.0 - Vec2::ONE;
        curr.0 = ndc_to_world.project_point3(ndc.extend(0.0)).xy();
    }
}
