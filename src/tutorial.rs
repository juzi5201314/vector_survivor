use crate::assets::{AudioAssets, FontAssets};
use crate::components::{TutorialUI, UIButton, BGM};
use crate::events::StartEvent;
use crate::states::AppState;
use bevy::audio::{Volume, VolumeLevel};
use bevy::prelude::*;

pub fn setup_tutorial(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    border: UiRect::all(Val::Px(2f32)),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                border_color: BorderColor(Color::WHITE),
                ..Default::default()
            },
            TutorialUI,
        ))
        .with_children(|builder| {
            builder.spawn((
                TextBundle::from_section(
                    "使用W/A/S/D或者方向键来控制角色(中心圆点)的移动\n\",\"和\".\"号用于控制音量",
                    TextStyle {
                        font: font_assets.chs.clone(),
                        font_size: 40.0,
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
            builder.spawn((
                TextBundle::from_section(
                    "小心红色方块，要是被他们碰到就会当场去世！",
                    TextStyle {
                        font: font_assets.chs.clone(),
                        font_size: 40.0,
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
            builder.spawn((
                TextBundle::from_section(
                    "当底部绿色经验条满了之后，可以选择一项技能升级，每5级有额外升级项目。\n\n",
                    TextStyle {
                        font: font_assets.chs.clone(),
                        font_size: 40.0,
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
                    UIButton("tutorial:start"),
                ))
                .with_children(|builder| {
                    builder.spawn(TextBundle::from_section(
                        "开始",
                        TextStyle {
                            font: font_assets.chs.clone(),
                            font_size: 30.0,
                            ..Default::default()
                        },
                    ));
                });
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
                    UIButton("tutorial:start-without-bgm"),
                ))
                .with_children(|builder| {
                    builder.spawn(TextBundle::from_section(
                        "开始（无背景音乐）",
                        TextStyle {
                            font: font_assets.chs.clone(),
                            font_size: 30.0,
                            ..Default::default()
                        },
                    ));
                });
            /*builder.spawn((
                TextBundle::from_section(
                    "\n\n[按任意键继续]\n（如果你的键盘上没有“任意”这个键，那你一定是买了假货）",
                    TextStyle {
                        font: font_assets.chs.clone(),
                        font_size: 30.0,
                        ..Default::default()
                    },
                )
                .with_style(Style {
                    align_self: AlignSelf::Center,
                    ..Default::default()
                })
                .with_text_alignment(TextAlignment::Center),
                Label,
            ));*/
        });
}

pub fn close_tutorial(
    mut ev_start: EventReader<StartEvent>,
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    ui: Query<Entity, With<TutorialUI>>,
    audio_assets: Res<AudioAssets>,
) {
    for bgm in ev_start.read() {
        commands.entity(ui.single()).despawn_recursive();
        next_state.set(AppState::Start);
        if bgm.0 {
            commands.spawn((
                AudioBundle {
                    source: audio_assets.bgm.clone(),
                    settings: PlaybackSettings::LOOP
                        .with_volume(Volume::Relative(VolumeLevel::new(0.1))),
                },
                BGM,
            ));
        }
    }
}
