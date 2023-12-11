use std::time;
use bevy::prelude::*;
use bevy_rapier2d::prelude::Group;

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
pub struct ChildColor(pub Color);

#[derive(Component, Deref, DerefMut, Default)]
pub struct Target(pub Vec2);

#[derive(Component, Deref, DerefMut, Default)]
pub struct CursorTargets(pub (Option<Vec2>, Option<Vec2>));

// #[derive(Component, Deref, DerefMut, Default)]
// pub struct SkillTarget(pub Option<Vec2>);

#[derive(Component, Deref, DerefMut, Default)]
pub struct Targets(pub Vec<Vec2>);

#[derive(Component, Deref, DerefMut, Default)]
pub struct Microbes(pub Vec<Entity>);

#[derive(Component, Default)]
pub struct IsPlayer;

#[derive(Component, Default)]
pub struct IsCharacter;

// #[derive(Component, Default)]
// pub struct IsBot;

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
pub struct CharacterCollisionGroup {
    pub parent_id: Group,
    pub child_id: Group
}
#[derive(Resource, Default)]
pub struct MineCollisionGroups {
    pub parent_id: Group,
    pub child_id: Group
}

#[derive(Component, Default)]
pub struct SkillCd(pub Timer);

#[derive(Resource, Default)]
pub struct SkillsCd {
    pub follow_cursor: f32,
    pub target_attack: f32,
    pub patrolling: f32,
}

#[derive(Component, PartialEq, Copy, Clone)] // Default
pub enum Skill {
    Patrolling(Option<Vec2>, Option<Vec2>),
    Rest(f32),
    TargetAttack(Option<Vec2>, f32),
    FollowCursor(Option<Vec2>),
}

impl Default for Skill {
    fn default() -> Self { Skill::Rest(30.) } // (Timer::from_seconds(1., TimerMode::Repeating))
}

#[derive(Component, Default, PartialEq, Copy, Clone)] // Default
pub enum TypeOfEntity {
    Character,
    #[default]
    Microbe,
}

#[derive(Component, Default)]
pub struct IsBot(pub bool);

#[derive(Resource)]
pub struct CharacterCollisionGroups(pub Group); // pub u32

impl Default for CharacterCollisionGroups {
    fn default() -> Self { Self(Group::ALL) } // u32::MAX (Timer::from_seconds(1., TimerMode::Repeating))
}

#[derive(Component, Default)]
pub struct Mover {
    pub max_speed: f32,
    pub max_force: f32,
    pub vel: Vec2,
    pub acc: Vec2,
    pub stiffness: f32,
    pub damper: f32,
    pub rotation_speed: f32,
    // new
    pub angle: f32,
    pub radius: f32,
    pub speed: f32,
}

#[derive(Resource, Default)]
pub struct MicrobeStats {
    pub min_count: i32,
    pub max_count: i32,
    pub size: f32,
    pub health: f32,
    pub spawn_price: f32,
    pub speed: f32,
    pub spawn_radius_min: f32,
    pub spawn_radius_max: f32,
}

#[derive(Resource, Default)]
pub struct CharacterEnergyStats {
    pub max_count: f32,
    pub regeneration_rate_per_sec: f32,
    pub character_microbes_die_energy_back: f32,
    pub enemy_microbes_kill_energy_reward: f32,
}

#[derive(Resource, Default)]
pub struct CharacterStats {
    pub max_count_bots: i32,
    pub size: f32,
    pub health: f32,
    pub energy: f32,
    pub speed: f32,
}

// just hidden simple marker
#[derive(Component, Default)]
pub struct InvisiblePlaceholder;

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
    pub skill_cd: SkillCd,
    pub skill: Skill,
    pub is_bot: IsBot,
    pub type_of_entity: TypeOfEntity,
    pub character: IsCharacter,
    pub character_collision_group: CharacterCollisionGroup,
    pub cursor_targets: CursorTargets,
    pub child_color: ChildColor,
    // pub is_player: IsPlayer,
}

#[derive(Bundle)]
pub struct MicrobeBundle {
    pub health: Health,
    pub is_microbe: Microbe,
    // pub orbit: Orbit,
    pub draw_stats: DrawStats,
    pub target: Target,
    pub type_of_entity: TypeOfEntity,
    // pub targets: Targets,
    // pub target: Target,
    // pub skill_target: SkillTarget,
    // pub rest_target: RestTarget,
    // pub parent_id: ParentEntityID,
    pub is_bot: IsBot,
    pub skill: Skill,
}


#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct ColorText;