use bevy::prelude::*;
use crate::input::systems::keyboard_input_system;

mod systems;
mod components;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, keyboard_input_system);
    }
}