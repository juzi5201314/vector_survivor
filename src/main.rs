#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Duration;

use bevy::prelude::*;
use bevy::render::settings::WgpuSettings;
use bevy::render::RenderPlugin;
use bevy::window::PrimaryWindow;
use bevy::DefaultPlugins;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use bevy_screen_diagnostics::*;
use bevy_vector_shapes::prelude::{
    DiscBundle, Rectangle, RectangleBundle, ShapeBundle, ShapeConfig,
};
use bevy_vector_shapes::Shape2dPlugin;
use bevy_xpbd_2d::prelude::{Collider, Gravity, PhysicsPlugins, RigidBody};
use rand::Rng;

use crate::assets::{AudioAssets, FontAssets, GameTime, Killed};
use crate::components::{
    BulletSpeed, Enemy, FireRate, GameEntity, Level, MoveSpeed, Player, TargetCount, XPBar, BGM, XP,
};
use crate::events::{
    read_kill_event, read_player_bullet_speed_up_event, read_player_fire_rate_up_event,
    read_player_move_speed_up_event, read_player_target_count_up_event, KillEvent,
    PlayerBulletSpeedUpEvent, PlayerFireRateUpEvent, PlayerMoveSpeedUpEvent,
    PlayerTargetCountUpEvent, PropsUpdateEvent, StartEvent, XpIncEvent,
};
use crate::fire::player_fire;
use crate::movement::{bullet_collision, enemy_approaches_player, move_bullet, move_player};
use crate::states::AppState;
use crate::tutorial::{close_tutorial, setup_tutorial};
use crate::ui::{
    click_button, exit_game_over_ui, exit_select_upgrade_ui, game_over_ui, select_upgrade_ui,
    show_properties, show_stats, update_properties, update_time_stats,
};

mod assets;
pub mod components;
mod events;
mod fire;
mod movement;
mod states;
mod tutorial;
mod ui;

fn main() {
    App::new()
        .add_plugins(EmbeddedAssetPlugin {
            mode: PluginMode::ReplaceDefault,
        })
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Vector Survivor".to_owned(),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest())
                .set(RenderPlugin {
                    render_creation: WgpuSettings {
                        //#[cfg(not(debug_assertions))] backends: Some(Backends::DX12),
                        ..Default::default()
                    }
                    .into(),
                }),
        )
        //.add_plugins(TomlAssetPlugin::<Weapons>::new(&["weapons.toml"]),)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(Shape2dPlugin::default())
        .add_plugins(ScreenDiagnosticsPlugin::default())
        .add_plugins((ScreenFrameDiagnosticsPlugin, ScreenEntityDiagnosticsPlugin))
        .add_event::<XpIncEvent>()
        .add_event::<KillEvent>()
        .add_event::<PlayerMoveSpeedUpEvent>()
        .add_event::<PlayerBulletSpeedUpEvent>()
        .add_event::<PlayerTargetCountUpEvent>()
        .add_event::<PlayerFireRateUpEvent>()
        .add_event::<PropsUpdateEvent>()
        .add_event::<StartEvent>()
        .add_systems(Startup, setup)
        // states
        .add_state::<AppState>()
        .add_loading_state(
            LoadingState::new(AppState::Loading).continue_to_state(AppState::Tutorial),
        )
        .add_systems(OnEnter(AppState::Tutorial), setup_tutorial)
        .add_systems(
            OnEnter(AppState::Start),
            (setup_game, show_properties, show_stats),
        )
        .add_systems(OnEnter(AppState::SelectUpgrade), select_upgrade_ui)
        .add_systems(OnEnter(AppState::GameOver), (game_over_ui, exit_game))
        .add_systems(Update, volume)
        .add_systems(
            Update,
            (
                move_player,
                //move_player_with_touch,
                //move_player_with_mouse,
                camera_follow,
                enemy_approaches_player,
                player_fire,
                move_bullet,
                bullet_collision,
                render_xp_bar,
                read_player_move_speed_up_event,
                read_player_bullet_speed_up_event,
                read_player_fire_rate_up_event,
                read_player_target_count_up_event,
                update_properties,
                read_kill_event,
            )
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            click_button.run_if(
                in_state(AppState::GameOver)
                    .or_else(in_state(AppState::SelectUpgrade))
                    .or_else(in_state(AppState::Tutorial)),
            ),
        )
        .add_systems(Update, close_tutorial.run_if(in_state(AppState::Tutorial)))
        .add_systems(
            FixedUpdate,
            (spawn_enemy, update_time_stats).run_if(in_state(AppState::InGame)),
        )
        .add_systems(OnExit(AppState::GameOver), exit_game_over_ui)
        .add_systems(OnExit(AppState::SelectUpgrade), exit_select_upgrade_ui)
        //.add_systems(OnExit(AppState::InGame), exit_game)
        .insert_resource(Time::<Fixed>::from_seconds(0.2))
        .insert_resource(Gravity(Vec2::ZERO))
        .insert_resource(GameTime(Duration::ZERO))
        .insert_resource(Killed(0))
        .add_collection_to_loading_state::<_, FontAssets>(AppState::Loading)
        .add_collection_to_loading_state::<_, AudioAssets>(AppState::Loading)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            hdr: false,
            ..Default::default()
        },
        ..Default::default()
    });
}

fn camera_follow(
    players: Query<&Transform, With<Player>>,
    mut cameras: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut bar: Query<&mut Transform, (With<XPBar>, Without<Player>, Without<Camera>)>,
) {
    let transform = players.single();
    for mut tf in &mut cameras {
        tf.translation.x = transform.translation.x;
        tf.translation.y = transform.translation.y;
    }
    let window = window.single();
    let bar = &mut bar.single_mut().translation;
    bar.x = transform.translation.x - window.width() / 2f32;
    bar.y = transform.translation.y - window.height() / 2f32;
}

fn setup_game(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
    mut game_time: ResMut<GameTime>,
    mut killed: ResMut<Killed>,
) {
    // spawn player
    commands.spawn((
        ShapeBundle::circle(
            &ShapeConfig {
                color: Color::PINK,
                ..ShapeConfig::default_2d()
            },
            8.0,
        ),
        GameEntity,
        Player,
        XP(0),
        Level(1),
        MoveSpeed(100.0),
        BulletSpeed(250.0),
        FireRate(60.0),
        TargetCount(1),
        RigidBody::Dynamic,
        Collider::ball(8.0),
    ));
    commands.spawn((
        ShapeBundle::rect(
            &ShapeConfig {
                color: Color::GREEN,
                ..ShapeConfig::default_2d()
            },
            Vec2::new(50f32, 5f32),
        ),
        GameEntity,
        XPBar,
    ));
    game_time.0 = time.elapsed();
    killed.0 = 0;
    next_state.set(AppState::InGame);
}

fn exit_game(mut commands: Commands, entities: Query<Entity, With<GameEntity>>) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_enemy(
    mut commands: Commands,
    time: Res<Time>,
    window: Query<&Window, With<PrimaryWindow>>,
    player: Query<&Transform, With<Player>>,
    game_time: Res<GameTime>,
) {
    let window = window.single();
    let player = player.single();

    if time.elapsed_seconds() % 1.0 == 0f32 {
        let game_time = (time.elapsed() - game_time.0).as_secs();
        let mut rng = rand::thread_rng();
        let count = (game_time / 5).max(2);
        for _ in 0..count {
            let x1 = player.translation.x + window.width() / 2f32;
            let x2 = player.translation.x - window.width() / 2f32;
            let y1 = player.translation.y + window.height() / 2f32;
            let y2 = player.translation.y - window.height() / 2f32;

            let random_point = if rng.gen_bool(0.5) {
                // 在屏幕上下出现
                let x = rng.gen_range(x2..=x1);
                let y = if rng.gen_bool(0.5) { y1 } else { y2 };
                Vec2::new(x, y)
            } else {
                // 在屏幕左右出现
                let y = rng.gen_range(y2..=y1);
                let x = if rng.gen_bool(0.5) { x1 } else { x2 };
                Vec2::new(x, y)
            };

            commands.spawn((
                ShapeBundle::rect(
                    &ShapeConfig {
                        color: Color::RED,
                        transform: Transform::from_translation(random_point.extend(0f32)),
                        ..ShapeConfig::default_2d()
                    },
                    Vec2::new(9.0, 9.0),
                ),
                GameEntity,
                Enemy,
                MoveSpeed(
                    80.0 + rng
                        .gen_range((game_time / 20 * 5).min(100)..=(game_time / 10 * 5).min(200))
                        as f32,
                ),
                RigidBody::Dynamic,
                Collider::ball(4.5),
            ));
        }
    }
}

fn render_xp_bar(
    mut ev_xp_inc: EventReader<XpIncEvent>,
    mut bar: Query<&mut Rectangle, With<XPBar>>,
    mut xp: Query<&mut XP>,
    mut level: Query<&mut Level>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    const BASE_REQUIRED_FOR_UPGRADE: usize = 12;

    for _ in ev_xp_inc.read() {
        let mut lvl = level.single_mut();
        let mut xp = xp.single_mut();
        let required_for_upgrade = BASE_REQUIRED_FOR_UPGRADE + lvl.0 * 3;
        xp.0 += 1;
        if xp.0 >= required_for_upgrade {
            xp.0 = 0;
            lvl.0 += 1;
            next_state.set(AppState::SelectUpgrade);
        }

        let width = window.single().width();
        bar.single_mut().size.x = xp.0 as f32 * (width / required_for_upgrade as f32) * 2f32;
    }
}

fn volume(keyboard_input: Res<Input<KeyCode>>, music_controller: Query<&AudioSink, With<BGM>>) {
    if let Ok(sink) = music_controller.get_single() {
        if keyboard_input.just_pressed(KeyCode::Period) {
            if sink.volume() < 3f32 {
                sink.set_volume(sink.volume() + 0.1);
            }
        } else if keyboard_input.just_pressed(KeyCode::Comma) {
            if sink.volume() > 0f32 {
                sink.set_volume((sink.volume() - 0.1).max(0f32));
            }
        }
    }
}
