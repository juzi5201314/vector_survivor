use bevy::input::mouse::MouseMotion;
use bevy::prelude::{
    Commands, CursorMoved, DespawnRecursiveExt, Entity, EventReader, EventWriter, Input, KeyCode,
    MouseButton, NextState, Quat, Query, Res, ResMut, Time, TouchInput, Transform, Vec2, Vec3,
    Vec3Swizzles, With, Without,
};
use bevy_xpbd_2d::prelude::Collision;

use crate::components::{AttackTarget, Bullet, MoveSpeed};
use crate::events::KillEvent;
use crate::states::AppState;
use crate::{Enemy, Player};

pub fn move_player_with_mouse(
    mouse: Res<Input<MouseButton>>,
    mut ev_mouse_motion: EventReader<MouseMotion>,
    time: Res<Time>,
    mut players: Query<(&mut Transform, &MoveSpeed), With<Player>>,
) {
    for mouse_motion in ev_mouse_motion.read() {
        if mouse.pressed(MouseButton::Left) {
            let (mut player, speed) = players.single_mut();
            let z = player.translation.z;
            let mut direction = Vec2::ZERO;
            if mouse_motion.delta.y < 0f32 {
                direction.y += 1f32;
            }
            if mouse_motion.delta.y > 0f32 {
                direction.y -= 1f32;
            }
            if mouse_motion.delta.x > 0f32 {
                direction.x += 1f32;
            }
            if mouse_motion.delta.x < 0f32 {
                direction.x -= 1f32;
            }
            if direction == Vec2::ZERO {
                return;
            }
            player.translation += (direction * speed.0 * time.delta_seconds()).extend(0f32);
            player.translation.z = z;
        }
    }
}

pub fn move_player_with_touch(
    mut ev_touch_input: EventReader<TouchInput>,
    time: Res<Time>,
    mut players: Query<(&mut Transform, &MoveSpeed), With<Player>>,
) {
    let (mut player, speed) = players.single_mut();
    for touch in ev_touch_input.read() {
        let z = player.translation.z;
        player.rotation = rotate_to(player.translation.xy(), touch.position);
        let local_y = player.local_y();
        player.translation += local_y * speed.0 * time.delta_seconds();
        player.translation.z = z;
    }
}

pub fn move_player(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut players: Query<(&mut Transform, &MoveSpeed), With<Player>>,
) {
    let mut direction = Vec2::ZERO;
    if keys.any_pressed([KeyCode::Up, KeyCode::W]) {
        direction.y += 1f32;
    }
    if keys.any_pressed([KeyCode::Down, KeyCode::S]) {
        direction.y -= 1f32;
    }
    if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
        direction.x += 1f32;
    }
    if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
        direction.x -= 1f32;
    }
    if direction == Vec2::ZERO {
        return;
    }

    for (mut transform, speed) in &mut players {
        let z = transform.translation.z;
        transform.translation += (direction * speed.0 * time.delta_seconds()).extend(0f32);
        transform.translation.z = z;
    }
}

pub fn enemy_approaches_player(
    players: Query<&Transform, With<Player>>,
    time: Res<Time>,
    mut enemies: Query<(&mut Transform, &MoveSpeed), (With<Enemy>, Without<Player>)>,
) {
    let player = players.get_single().unwrap();
    for (mut enemy, speed) in &mut enemies {
        let z = enemy.translation.z;
        enemy.rotation = rotate_to(enemy.translation.xy(), player.translation.xy());
        let local_y = enemy.local_y();
        enemy.translation += local_y * speed.0 * time.delta_seconds();
        enemy.translation.z = z;
        /*let direction = move_to(&enemy.translation, &player.translation);
        let z = enemy.translation.z;
        enemy.translation += (direction * speed.0).extend(0f32);
        enemy.translation.z = z;*/
    }
}

pub fn move_bullet(
    mut bullets: Query<(&mut Transform, &MoveSpeed, &mut AttackTarget), With<Bullet>>,
    enemies: Query<&Transform, (With<Enemy>, Without<Bullet>)>,
    time: Res<Time>,
) {
    for (mut transform, speed, target) in &mut bullets {
        if let Ok(enemy) = enemies.get(target.0) {
            transform.rotation = rotate_to(transform.translation.xy(), enemy.translation.xy());
            //*direction = move_to(&transform.translation, &enemy.translation)
        }

        let z = transform.translation.z;
        //transform.translation += (*direction * speed.0).extend(0f32);
        let local_y = transform.local_y();
        transform.translation += local_y * speed.0 * time.delta_seconds();
        transform.translation.z = z;
    }
}

pub fn bullet_collision(
    mut collision_event_reader: EventReader<Collision>,
    bullets: Query<Entity, With<Bullet>>,
    enemies: Query<Entity, (With<Enemy>, Without<Bullet>)>,
    players: Query<Entity, (With<Player>, (Without<Bullet>, Without<Enemy>))>,
    mut command: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    mut ev_kill: EventWriter<KillEvent>,
) {
    for Collision(contacts) in collision_event_reader.read() {
        match (
            bullets
                .get(contacts.entity1)
                .or_else(|_| bullets.get(contacts.entity2)),
            enemies
                .get(contacts.entity1)
                .or_else(|_| enemies.get(contacts.entity2)),
            players
                .get(contacts.entity1)
                .or_else(|_| players.get(contacts.entity2)),
        ) {
            (Ok(bullet), Ok(enemy), Err(_)) => {
                command.entity(bullet).despawn_recursive();
                command.entity(enemy).despawn_recursive();
                ev_kill.send(KillEvent);
            }
            (Err(_), Ok(_), Ok(_)) => {
                next_state.set(AppState::GameOver);
                //next_state.set(AppState::Restart);
            }
            _ => continue,
        }
    }
}

fn rotate_to(source: Vec2, target: Vec2) -> Quat {
    Quat::from_rotation_arc(Vec3::Y, (target - source).normalize().extend(0f32))
}

fn move_to(source: &Vec3, target: &Vec3) -> Vec2 {
    let mut direction = Vec2::ZERO;
    if source.x > target.x {
        direction.x -= 1f32;
    }
    if source.x < target.x {
        direction.x += 1f32;
    }
    if source.y > target.y {
        direction.y -= 1f32;
    }
    if source.y < target.y {
        direction.y += 1f32;
    }
    direction
}
