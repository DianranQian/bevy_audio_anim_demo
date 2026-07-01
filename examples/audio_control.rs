//! AudioSink 控制演示：暂停、音量、变速
//!
//! 运行：cargo run --example audio_control

use bevy::audio::Volume;
use bevy::prelude::*;
use bevy_audio_anim_demo::*;

fn main() {
    App::new()
        .add_plugins(default_plugins())
        .add_plugins(DemoPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_status, handle_input))
        .run();
}

#[derive(Component)]
struct Music;

#[derive(Component)]
struct StatusText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_default_camera(&mut commands);

    commands.spawn((
        AudioPlayer::new(asset_server.load("sounds/windless_slopes.ogg")),
        PlaybackSettings::LOOP,
        Music,
    ));

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
        "AudioSink 控制演示\n空格: 暂停/恢复 | ↑↓: 音量 | ←→: 速度 | R: 重置",
    );
}

fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut music: Query<&mut AudioSink, With<Music>>,
) {
    let Ok(mut sink) = music.single_mut() else { return };

    if keyboard.just_pressed(KeyCode::Space) {
        sink.toggle_playback();
    }
    let vol = sink.volume().to_linear();
    if keyboard.just_pressed(KeyCode::ArrowUp) {
        sink.set_volume(Volume::Linear((vol + 0.1).min(1.0)));
    }
    if keyboard.just_pressed(KeyCode::ArrowDown) {
        sink.set_volume(Volume::Linear((vol - 0.1).max(0.0)));
    }
    let speed = sink.speed();
    if keyboard.just_pressed(KeyCode::ArrowRight) {
        sink.set_speed((speed + 0.25).min(3.0));
    }
    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        sink.set_speed((speed - 0.25).max(0.25));
    }
    if keyboard.just_pressed(KeyCode::KeyR) {
        sink.set_speed(1.0);
    }
}

fn update_status(
    music: Query<&AudioSink, With<Music>>,
    mut status: Query<&mut Text, With<StatusText>>,
) {
    let Ok(sink) = music.single() else { return };
    let state = if sink.is_paused() { "已暂停" } else { "播放中" };
    let text = format!(
        "状态: {}\n进度: {:.1}s\n音量: {:.1}\n速度: {:.2}x",
        state,
        sink.position().as_secs_f32(),
        sink.volume().to_linear(),
        sink.speed(),
    );
    if let Ok(mut t) = status.single_mut() {
        **t = text;
    }
}
