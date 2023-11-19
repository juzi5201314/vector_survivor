use bevy::prelude::*;

use crate::assets::Killed;
use crate::components::{
    Bullet, BulletSpeed, FireRate, MoveSpeed, Player, StatsUIKill, TargetCount,
};

#[derive(Event)]
pub struct XpIncEvent;

#[derive(Event)]
pub struct KillEvent;

#[derive(Event)]
pub struct PlayerMoveSpeedUpEvent;

#[derive(Event)]
pub struct PlayerFireRateUpEvent;

#[derive(Event)]
pub struct PlayerBulletSpeedUpEvent;

#[derive(Event)]
pub struct PlayerTargetCountUpEvent;

#[derive(Event)]
pub struct PropsUpdateEvent;

#[derive(Event)]
pub struct StartEvent(pub bool);

pub fn read_player_move_speed_up_event(
    mut ev_player_move_speed_up: EventReader<PlayerMoveSpeedUpEvent>,
    mut speed: Query<&mut MoveSpeed, (With<Player>, Without<Bullet>)>,
    mut ev_props_update: EventWriter<PropsUpdateEvent>,
) {
    let mut speed = speed.single_mut();
    for _ in ev_player_move_speed_up.read() {
        speed.0 *= 1.05;
        ev_props_update.send(PropsUpdateEvent);
    }
}

pub fn read_player_fire_rate_up_event(
    mut ev_player_fire_rate_up: EventReader<PlayerFireRateUpEvent>,
    mut rate: Query<&mut FireRate, With<Player>>,
    mut ev_props_update: EventWriter<PropsUpdateEvent>,
) {
    let mut rate = rate.single_mut();
    for _ in ev_player_fire_rate_up.read() {
        rate.0 *= 1.05;
        ev_props_update.send(PropsUpdateEvent);
    }
}

pub fn read_player_bullet_speed_up_event(
    mut ev_player_bullet_speed_up: EventReader<PlayerBulletSpeedUpEvent>,
    mut speed: Query<&mut BulletSpeed, With<Player>>,
    mut ev_props_update: EventWriter<PropsUpdateEvent>,
) {
    let mut speed = speed.single_mut();
    for _ in ev_player_bullet_speed_up.read() {
        speed.0 *= 1.05;
        ev_props_update.send(PropsUpdateEvent);
    }
}

pub fn read_player_target_count_up_event(
    mut ev_player_target_count_up: EventReader<PlayerTargetCountUpEvent>,
    mut count: Query<&mut TargetCount, With<Player>>,
    mut ev_props_update: EventWriter<PropsUpdateEvent>,
) {
    let mut count = count.single_mut();
    for _ in ev_player_target_count_up.read() {
        count.0 += 1;
        ev_props_update.send(PropsUpdateEvent);
    }
}

pub fn read_kill_event(
    mut ev_kill: EventReader<KillEvent>,
    mut ev_xp_up: EventWriter<XpIncEvent>,
    mut killed: ResMut<Killed>,
    mut stat: Query<&mut Text, With<StatsUIKill>>,
) {
    for _ in ev_kill.read() {
        killed.0 += 1;
        stat.single_mut().sections[0].value = format!("killed {}", killed.0);
        ev_xp_up.send(XpIncEvent);
    }
}
