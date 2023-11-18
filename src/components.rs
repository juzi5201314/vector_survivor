use bevy::prelude::{Component, Entity};

#[derive(Component)]
pub struct GameEntity;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct XP(pub usize);

#[derive(Component)]
pub struct Level(pub usize);

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct FireRate(pub f32);

#[derive(Component)]
pub struct BulletSpeed(pub f32);

#[derive(Component)]
pub struct TargetCount(pub usize);

#[derive(Component)]
pub struct MoveSpeed(pub f32);

#[derive(Component)]
pub struct AttackTarget(pub Entity);

#[derive(Component)]
pub struct UIButton(pub &'static str);

#[derive(Component)]
pub struct GameOverUI;

#[derive(Component)]
pub struct SelectUpgradeUI;

#[derive(Component)]
pub struct TutorialUI;

#[derive(Component)]
pub struct XPBar;

#[derive(Component)]
pub struct PlayerProps;

#[derive(Component)]
pub struct StatsUI;

#[derive(Component)]
pub struct StatsUIKill;

#[derive(Component)]
pub struct StatsUITime;

#[derive(Component)]
pub struct BGM;
