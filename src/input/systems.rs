use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::character::{components::IsPlayer};
use crate::character::components::{CombatState, Target, ToSpawnMic};
use crate::input::components::Cursor;

pub fn keyboard_input_system(
    mut curr: ResMut<Cursor>,
    keyboard_input: Res<Input<KeyCode>>,
    mut players_transform: Query<(&mut Transform, &mut CombatState, &mut Target, &mut ToSpawnMic), With<IsPlayer>>,
    //mut players_transform: Query<&mut KinematicCharacterController, With<IsPlayer>>,
) {
    let mut vel = Vec3::default();

    if let Ok((mut player, mut combat, mut target, mut toSpawn)) = players_transform.get_single_mut() {
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

        player.translation += vel.normalize_or_zero();

        combat.0 = false;

        if keyboard_input.pressed(KeyCode::E) {
            combat.0 = true;
            target.0 = curr.0;
            //println!("{}", curr.0)
        }

        if keyboard_input.just_pressed(KeyCode::Q) {
            toSpawn.0 = true;
            // target.0 = curr.0;
            //println!("{}", curr.0)
        }
    }

    //vel = vel.normalize();
    // player.translation = vel;
    // player.translation = Some(vel.normalize().xy());
    // if vel.y != 0. && vel.x !=0. {
    //     player.translation += vel.normalize();
    // }

    // match player {
    //     Some(mut p) => {
    //         p += vel
    //     }
    //     None => ()
    // }
}

fn cursor_convert_pos_to_world(window: &Window) -> Option<Vec2> {
    if let Some(pos) = window.physical_cursor_position() {
        let size = Vec2::new(
            window.physical_width() as f32,
            window.physical_height() as f32,
        );

        // Convert cursor pos to the world
        let world_pos = Vec2::new(pos.x - size.x / 2.0, pos.y - size.y / 2.0);
        return Some(world_pos);
    }

    None
}


pub fn mouse_input_system(
    mut curr: ResMut<Cursor>,
    windows: Query<&Window>,
    player: Query<(&Transform, &Camera), With<IsPlayer>>,
) {
    if let (Ok(mut window), Ok(mut player)) = (windows.get_single(), player.get_single()) {
        // if let Some(t) = cursor_convert_pos_to_world(window) {
        //     curr.0 = t;
        // }
        let c = window.cursor_position().map_or(Vec2::new(0., 0.), |cur| cur);

        let window_size = Vec2::new(window.physical_width() as f32, window.physical_height() as f32);
        let ndc_to_world = player.0.compute_matrix() * player.1.projection_matrix().inverse();
        let ndc = Vec2::new(c.x / window_size.x, 1.0 - c.y / window_size.y) * 2.0 - Vec2::ONE;
        // let ndc = (window.cursor_position().map_or(Vec2::new(0., 0.), |cur| cur) / window_size) * 2.0 - Vec2::ONE;
        curr.0 = ndc_to_world.project_point3(ndc.extend(0.0)).xy();
    }
}
