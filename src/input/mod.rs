use bevy::prelude::*;
//use crate::input::systems::keyboard_input_system;

mod systems;
pub mod components;

use components::*;
use systems::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Cursor::default())
            .add_systems(Update, keyboard_input_system)
            .add_systems(Update, mouse_input_system);
    }
}