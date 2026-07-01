//! 3D 空间音频演示
//!
//! 运行：cargo run --example audio_spatial

use std::f32::consts::PI;

use bevy::{color::palettes::basic::*, prelude::*};
use bevy_audio_anim_demo::*;

fn main() {
    App::new()
        .add_plugins(default_plugins())
        .add_plugins(DemoPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (orbit_listener, update_status))
        .run();
}

#[derive(Component)]
struct Listener;

#[derive(Component)]
struct StatusText;

const ORBIT_RADIUS_X: f32 = 6.0;
const ORBIT_RADIUS_Z: f32 = 2.0;
const ORBIT_SPEED: f32 = 0.8;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 声源（蓝色球）— 固定在原点
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.3).mesh().uv(32, 18))),
        MeshMaterial3d(materials.add(Color::from(BLUE))),
        Transform::from_xyz(0.0, 0.5, 0.0),
        AudioPlayer::new(asset_server.load("sounds/windless_slopes_mono.ogg")),
        PlaybackSettings::LOOP.with_spatial(true),
    ));

    // 听者（白球）— 围绕声源旋转，红绿耳朵跟着走
    let listener = SpatialListener::new(0.4);
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.2).mesh().uv(16, 8))),
        MeshMaterial3d(materials.add(Color::WHITE.with_alpha(0.8))),
        Transform::from_xyz(ORBIT_RADIUS_X, 0.5, 0.0),
        listener.clone(),
        Listener,
        children![
            (
                Mesh3d(meshes.add(Cuboid::new(0.15, 0.15, 0.15))),
                MeshMaterial3d(materials.add(Color::from(RED))),
                Transform::from_translation(listener.left_ear_offset),
            ),
            (
                Mesh3d(meshes.add(Cuboid::new(0.15, 0.15, 0.15))),
                MeshMaterial3d(materials.add(Color::from(LIME))),
                Transform::from_translation(listener.right_ear_offset),
            ),
        ],
    ));

    // 俯视角相机
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 12.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Text::new(""),
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
        "3D 空间音频演示\n蓝球=声源（固定）| 白球=听者（围绕声源旋转）\n红/绿=左/右耳\n靠近→声大 | 远离→声小",
    );
}

fn orbit_listener(time: Res<Time>, mut listener: Query<&mut Transform, With<Listener>>) {
    let Ok(mut tf) = listener.single_mut() else { return };
    let angle = time.elapsed_secs() * ORBIT_SPEED;
    tf.translation.x = angle.cos() * ORBIT_RADIUS_X;
    tf.translation.z = angle.sin() * ORBIT_RADIUS_Z;
    tf.translation.y = 0.5 + (time.elapsed_secs() * 1.5).sin() * 0.3;
}

fn update_status(
    time: Res<Time>,
    listener: Query<&Transform, With<Listener>>,
    mut status: Query<&mut Text, With<StatusText>>,
) {
    let Ok(tf) = listener.single() else { return };
    let dist = tf.translation.length();
    let angle = tf.translation.x.atan2(tf.translation.z) * 180.0 / PI;
    let side = if angle > -45.0 && angle < 45.0 {
        "声源在正前方"
    } else if angle >= 45.0 && angle < 135.0 {
        "声源在右侧"
    } else if angle <= -45.0 && angle > -135.0 {
        "声源在左侧"
    } else {
        "声源在正后方"
    };
    if let Ok(mut t) = status.single_mut() {
        **t = format!("距离: {:.1}\n方位: {:.0}° ({})\n时间: {:.1}s", dist, angle, side, time.elapsed_secs());
    }
}
