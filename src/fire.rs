use std::time::Duration;

use bevy::prelude::{Color, Commands, Entity, Local, Query, Res, Transform, With, Without};
use bevy::time::Time;
use bevy_vector_shapes::prelude::{DiscBundle, ShapeBundle, ShapeConfig};
use bevy_xpbd_2d::prelude::Collider;

use crate::components::{
    AttackTarget, Bullet, BulletSpeed, Enemy, FireRate, GameEntity, MoveSpeed, Player, TargetCount,
};

pub fn player_fire(
    mut commands: Commands,
    players: Query<
        (&Transform, &FireRate, &TargetCount, &BulletSpeed),
        (With<Player>, Without<Enemy>),
    >,
    enemies: Query<(&Transform, Entity), (With<Enemy>, Without<Player>)>,
    time: Res<Time>,
    mut last_fire: Local<Duration>,
) {
    if enemies.is_empty() {
        return;
    }
    let (player, fire_rate, target_count, bullet_speed) = players.get_single().unwrap();
    // pre minute
    let rate = Duration::from_secs(60).as_millis() as f32 / fire_rate.0;
    let interval = time.elapsed() - *last_fire;

    // fire
    if interval.as_millis() >= rate as u128 {
        let mut entities: Vec<(f32, Entity)> = enemies
            .iter()
            .map(|enemy| (player.translation.distance(enemy.0.translation), enemy.1))
            .collect();
        glidesort::sort_in_vec_by(&mut entities, |l, r| l.0.total_cmp(&r.0));

        let target = entities.iter().take(target_count.0);

        for (_, entity) in target {
            commands.spawn((
                ShapeBundle::circle(
                    &ShapeConfig {
                        color: Color::WHITE,
                        transform: Transform::from_translation(player.translation),
                        ..ShapeConfig::default_2d()
                    },
                    3.0,
                ),
                GameEntity,
                Bullet,
                MoveSpeed(bullet_speed.0),
                AttackTarget(*entity),
                Collider::ball(3.0),
            ));
        }
        *last_fire = time.elapsed();
    }
}
