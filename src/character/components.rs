use bevy::prelude::*;

#[derive(Component, Deref, DerefMut, Default)]
pub struct Health(pub f32);

#[derive(Component, Deref, DerefMut, Default)]
pub struct Speed(pub f32);

#[derive(Component, Deref, DerefMut, Default)]
pub struct CountMicrobes(pub i32);

#[derive(Component, Deref, DerefMut)]
pub struct ParentEntityID(pub Entity);

// #[derive(Component, Deref, DerefMut, Default)]
// pub struct Orbit(pub f32);

#[derive(Component, Deref, DerefMut, Default)]
pub struct CombatState(pub bool);

#[derive(Component, Deref, DerefMut, Default)]
pub struct ToSpawnMic(pub bool);

#[derive(Component, Deref, DerefMut, Default)]
pub struct Energy(pub f32);

#[derive(Component, Deref, DerefMut, Default)]
pub struct Target(pub Vec2);

#[derive(Component, Deref, DerefMut, Default)]
pub struct Microbes(pub Vec<Entity>);

#[derive(Component, Default)]
pub struct IsPlayer;

#[derive(Component, Default)]
pub struct IsBot;

// #[derive(Component, Default)]
// pub struct DrawIt;

#[derive(Component, Default)]
pub struct Microbe;


#[derive(Component, Default)]
pub struct DrawStats {
    pub radius: f32,
    pub color: Color
}

#[derive(Component, Default)]
pub struct Mover {
    pub max_speed: f32,
    pub max_force: f32,
    pub vel: Vec2,
    pub acc: Vec2,
    pub stiffness: f32,
    pub damper: f32,
}

#[derive(Resource, Default)]
pub struct MicrobeStats {
    pub min_count: f32,
    pub max_count: f32,
    pub size: f32,
    pub health: f32,
    pub spawn_price: f32,
    pub spawn_radius: f32,
}

#[derive(Bundle, Default)]
pub struct CharacterBundle {
    pub health: Health,
    pub speed: Speed,
    pub to_spawn_mic: ToSpawnMic,
    // pub microbes: Microbes,
    // pub draw_it: DrawIt,
    pub draw_stats: DrawStats,
    pub combat: CombatState,
    pub target: Target,
    pub energy: Energy,
    pub count_microbes: CountMicrobes,
    // pub is_player: IsPlayer,
}

#[derive(Bundle)]
pub struct MicrobeBundle {
    pub health: Health,
    pub is_microbe: Microbe,
    // pub orbit: Orbit,
    pub draw_stats: DrawStats,
    pub target: Target,
    pub parent_id: ParentEntityID,
}
