
use bevy::prelude::*;

#[derive(Component, Deref, DerefMut, Default)]
pub struct Health(pub f32);

#[derive(Component, Deref, DerefMut, Default)]
pub struct Speed(pub f32);

#[derive(Component, Default)]
pub struct IsPlayer;

#[derive(Component, Default)]
pub struct IsBot;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub health: Health,
    pub speed: Speed,
    // pub is_player: IsPlayer,
}

#[derive(Bundle, Default)]
pub struct MicrobeBundle {
    pub health: Health,
    pub speed: Speed,
    // pub is_player: IsPlayer,
}