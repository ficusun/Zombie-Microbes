use bevy::prelude::*;

#[derive(Component, Deref, DerefMut, Default)]
pub struct Health(pub f32);

#[derive(Component, Deref, DerefMut, Default)]
pub struct Speed(pub f32);

#[derive(Component, Deref, DerefMut, Default)]
pub struct Orbit(pub f32);

#[derive(Component, Deref, DerefMut, Default)]
pub struct Microbes(pub Vec<Entity>);

#[derive(Component, Default)]
pub struct IsPlayer;

#[derive(Component, Default)]
pub struct IsBot;

#[derive(Component, Default)]
pub struct DrawIt;

#[derive(Component, Default)]
pub struct Microbe;

#[derive(Component, Default)]
pub struct Wave {
    pub points_count: i32,
    pub base_radius: f32,
    pub rotation_speed: f32,
    pub phase_speed: f32,
    pub frequency: f32,
    pub amplitude: f32,
    pub range: f32,
    pub strength_factor: f32,
}

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub health: Health,
    pub speed: Speed,
    pub microbes: Microbes,
    pub draw_it: DrawIt,
    // pub is_player: IsPlayer,
}

#[derive(Bundle, Default)]
pub struct MicrobeBundle {
    pub health: Health,
    pub speed: Speed,
    pub is_microbe: Microbe,
    pub orbit: Orbit,
}
