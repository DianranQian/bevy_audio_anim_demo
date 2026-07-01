//! AudioPlayer + PlaybackSettings 基础演示
//!
//! 运行：cargo run --example audio_basic

use bevy::prelude::*;
use bevy_audio_anim_demo::*;

fn main() {
    App::new()
        .add_plugins(default_plugins())
        .add_plugins(DemoPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, update_status)
        .run();
}

#[derive(Component)]
struct BgmMusic;

#[derive(Component)]
struct SfxSound;

#[derive(Component)]
struct StatusText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_default_camera(&mut commands);

    // 背景音乐 — 循环播放
    commands.spawn((
        AudioPlayer::new(asset_server.load("sounds/windless_slopes.ogg")),
        PlaybackSettings::LOOP,
        BgmMusic,
    ));

    // 音效 — 播完自动 despawn
    commands.spawn((
        AudioPlayer::new(asset_server.load("sounds/breakout_collision.ogg")),
        PlaybackSettings::DESPAWN,
        SfxSound,
    ));

    // 状态文字
    commands.spawn((
        Text::new("加载中..."),
        cn_font(&asset_server, 18.0),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
        StatusText,
    ));

    spawn_hint(
        &mut commands,
        &asset_server,
        "AudioPlayer 基础演示\nBGM: 循环播放 (LOOP) | 音效: 播完自动清理 (DESPAWN)\n按 P 暂停/恢复 BGM",
    );
}

fn update_status(
    keyboard: Res<ButtonInput<KeyCode>>,
    bgm: Query<&AudioSink, With<BgmMusic>>,
    sfx: Query<Entity, With<SfxSound>>,
    mut status: Query<&mut Text, With<StatusText>>,
) {
    let mut lines = vec![];
    match bgm.single() {
        Ok(sink) => {
            let state = if sink.is_paused() { "已暂停" } else { "播放中" };
            lines.push(format!("BGM: {} ({:.1}s)", state, sink.position().as_secs_f32()));
            if keyboard.just_pressed(KeyCode::KeyP) {
                sink.toggle_playback();
            }
        }
        Err(_) => lines.push("BGM: 未加载".to_string()),
    }
    if sfx.single().is_ok() {
        lines.push("音效: 播放中 (将自动 despawn)".to_string());
    } else {
        lines.push("音效: 已 despawn (播完自动清理)".to_string());
    }
    if let Ok(mut text) = status.single_mut() {
        **text = lines.join("\n");
    }
}
