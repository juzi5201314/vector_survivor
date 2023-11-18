use crate::assets::{FontAssets, GameTime};
use bevy::prelude::*;
use std::time::Duration;

use crate::components::{
    Bullet, BulletSpeed, FireRate, GameEntity, GameOverUI, Level, MoveSpeed, Player, PlayerProps,
    SelectUpgradeUI, StatsUI, StatsUIKill, StatsUITime, TargetCount, UIButton,
};
use crate::events::{
    PlayerBulletSpeedUpEvent, PlayerFireRateUpEvent, PlayerMoveSpeedUpEvent,
    PlayerTargetCountUpEvent, PropsUpdateEvent, StartEvent,
};
use crate::states::AppState;

pub fn select_upgrade_ui(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    lvl: Query<&Level, With<Player>>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
            SelectUpgradeUI,
        ))
        .with_children(|builder| {
            builder.spawn((
                TextBundle::from_section(
                    "选择你的升级!",
                    TextStyle {
                        font: font_assets.chs.clone(),
                        font_size: 50.0,
                        ..Default::default()
                    },
                )
                .with_style(Style {
                    align_self: AlignSelf::Center,
                    ..Default::default()
                })
                .with_text_alignment(TextAlignment::Center),
                Label,
            ));
            builder
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|builder| {
                    spawn_select_upgrade_ui_button(
                        builder,
                        font_assets.chs.clone(),
                        "select_upgrade:move_speed",
                        "移速升级",
                        "\n\n玩家移动速度 +3%",
                    );
                    spawn_select_upgrade_ui_button(
                        builder,
                        font_assets.chs.clone(),
                        "select_upgrade:fire_rate",
                        "射速升级",
                        "\n\n开火速率(每分钟) +3%",
                    );
                    spawn_select_upgrade_ui_button(
                        builder,
                        font_assets.chs.clone(),
                        "select_upgrade:bullet_speed",
                        "弹速升级",
                        "\n\n子弹飞行速度 +3%",
                    );
                    if lvl.single().0 % 5 == 0 {
                        spawn_select_upgrade_ui_button(
                            builder,
                            font_assets.chs.clone(),
                            "select_upgrade:target_count",
                            "子弹数量升级",
                            "\n\n可以同时射出的子弹 +1",
                        );
                    }
                });
        });
}

fn spawn_select_upgrade_ui_button(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    key: &'static str,
    text: &'static str,
    desc: &'static str,
) {
    builder
        .spawn((
            ButtonBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(2f32)),
                    margin: UiRect::all(Val::Px(10f32)),
                    padding: UiRect::all(Val::Px(5f32)),
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::NONE),
                border_color: BorderColor(Color::WHITE),
                ..Default::default()
            },
            UIButton(key),
        ))
        .with_children(|builder| {
            builder.spawn(
                TextBundle::from_sections([
                    TextSection::new(
                        text,
                        TextStyle {
                            font: font.clone(),
                            font_size: 30.0,
                            ..Default::default()
                        },
                    ),
                    TextSection::new(
                        desc,
                        TextStyle {
                            font,
                            font_size: 20.0,
                            ..Default::default()
                        },
                    ),
                ])
                .with_text_alignment(TextAlignment::Center),
            );
        });
}

pub fn game_over_ui(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
            GameOverUI,
        ))
        .with_children(|builder| {
            builder.spawn((
                TextBundle::from_section(
                    "Game Over!",
                    TextStyle {
                        font: font_assets.eng.clone(),
                        font_size: 50.0,
                        ..Default::default()
                    },
                )
                .with_style(Style {
                    align_self: AlignSelf::Center,
                    ..Default::default()
                })
                .with_text_alignment(TextAlignment::Center),
                Label,
            ));
            builder
                .spawn((
                    ButtonBundle {
                        style: Style {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(2f32)),
                            margin: UiRect::top(Val::Px(10f32)),
                            padding: UiRect::all(Val::Px(5f32)),
                            ..Default::default()
                        },
                        background_color: BackgroundColor(Color::NONE),
                        border_color: BorderColor(Color::WHITE),
                        ..Default::default()
                    },
                    UIButton("game_over:restart"),
                ))
                .with_children(|builder| {
                    builder.spawn(TextBundle::from_section(
                        "重新开始",
                        TextStyle {
                            font: font_assets.chs.clone(),
                            font_size: 30.0,
                            ..Default::default()
                        },
                    ));
                });
        });
}

pub fn click_button(
    interaction: Query<(&Interaction, &UIButton), Changed<Interaction>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
) {
    for (interaction, button) in &interaction {
        match *interaction {
            Interaction::Pressed => match button.0 {
                "game_over:restart" => next_state.set(AppState::Start),
                s if s.starts_with("tutorial:start") => commands.add(|world: &mut World| {
                    world.send_event(StartEvent(!s.ends_with("-without-bgm")))
                }),
                s if s.starts_with("select_upgrade:") => {
                    match s {
                        "select_upgrade:move_speed" => commands
                            .add(|world: &mut World| world.send_event(PlayerMoveSpeedUpEvent)),
                        "select_upgrade:fire_rate" => commands
                            .add(|world: &mut World| world.send_event(PlayerFireRateUpEvent)),
                        "select_upgrade:bullet_speed" => commands
                            .add(|world: &mut World| world.send_event(PlayerBulletSpeedUpEvent)),
                        "select_upgrade:target_count" => commands
                            .add(|world: &mut World| world.send_event(PlayerTargetCountUpEvent)),
                        _ => {}
                    }
                    next_state.set(AppState::InGame)
                }
                _ => {}
            },
            _ => {}
        }
    }
}

pub fn exit_game_over_ui(mut commands: Commands, entity: Query<Entity, With<GameOverUI>>) {
    entity.for_each(|e| commands.entity(e).despawn_recursive())
}

pub fn exit_select_upgrade_ui(
    mut commands: Commands,
    entity: Query<Entity, With<SelectUpgradeUI>>,
) {
    entity.for_each(|e| commands.entity(e).despawn_recursive())
}

pub fn show_properties(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    mut ev_props_update: EventWriter<PropsUpdateEvent>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    ..Default::default()
                },
                ..Default::default()
            },
            GameEntity,
        ))
        .with_children(|builder| {
            builder.spawn((
                TextBundle::from_section(
                    "loading",
                    TextStyle {
                        font: font_assets.eng.clone(),
                        font_size: 15.0,
                        ..Default::default()
                    },
                ),
                PlayerProps,
                Label,
            ));
        });
    ev_props_update.send(PropsUpdateEvent);
}

pub fn update_properties(
    mut ev_props_update: EventReader<PropsUpdateEvent>,
    mut props: Query<&mut Text, With<PlayerProps>>,
    move_speed: Query<&mut MoveSpeed, (With<Player>, Without<Bullet>)>,
    rate: Query<&mut FireRate, With<Player>>,
    bullet_speed: Query<&mut BulletSpeed, With<Player>>,
    count: Query<&mut TargetCount, With<Player>>,
) {
    for _ in ev_props_update.read() {
        let s = format!(
            "MoveSpeed: {}\nFireRate: {}\nBulletSpeed: {}\nBulletCount: {}",
            move_speed.single().0,
            rate.single().0,
            bullet_speed.single().0,
            count.single().0,
        );

        props.single_mut().sections[0].value = s;
    }
}

pub fn show_stats(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    old: Query<Entity, With<StatsUI>>,
) {
    for entity in &old {
        commands.entity(entity).despawn_recursive();
    }
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
            StatsUI,
        ))
        .with_children(|builder| {
            builder.spawn((
                TextBundle::from_section(
                    "0s",
                    TextStyle {
                        font: font_assets.eng.clone(),
                        font_size: 25.0,
                        ..Default::default()
                    },
                ),
                StatsUITime,
                Label,
            ));
            builder.spawn((
                TextBundle::from_section(
                    "killed 0",
                    TextStyle {
                        font: font_assets.eng.clone(),
                        font_size: 15.0,
                        ..Default::default()
                    },
                ),
                StatsUIKill,
                Label,
            ));
        });
}

pub fn update_time_stats(
    time: Res<Time>,
    game_time: Res<GameTime>,
    mut stat: Query<&mut Text, With<StatsUITime>>,
) {
    if time.elapsed_seconds() % 1.0 == 0f32 {
        stat.single_mut().sections[0].value = format!(
            "{:?}",
            Duration::from_secs((time.elapsed() - game_time.0).as_secs())
        );
    }
}
