use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::character::{components::IsPlayer};
pub fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    // mut players_transform: Query<&mut Transform, With<IsPlayer>>,
    mut players_transform: Query<&mut KinematicCharacterController, With<IsPlayer>>,
) {
    let mut vel = Vec3::default();
    Vec3::from

    // let mut player = players_transform // : Option<Vec2>
    //     .get_single_mut().unwrap();//.translation.xy();
    //     //.map_or_else(None, |p| Some(p.translation.xy())); //single_mut()
    let mut player = players_transform // : Option<Vec2>
         .get_single_mut().unwrap();//.translation.xy();
    //     //.map_or_else(None, |p| Some(p.translation.xy())); //single_mut()

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

    //vel = vel.normalize();
    // player.translation = vel;
    player.translation = Some(vel.normalize().xy());
    if keyboard_input.just_pressed(KeyCode::E) {
        println!("{}", player.translation.unwrap())
    }
    // match player {
    //     Some(mut p) => {
    //         p += vel
    //     }
    //     None => ()
    // }
}