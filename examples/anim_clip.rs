//! AnimationClip + VariableCurve 程序化动画演示
//!
//! 运行：cargo run --example anim_clip

use std::f32::consts::PI;

use bevy::animation::{animated_field, AnimatedBy, AnimationTargetId};
use bevy::prelude::*;
use bevy_audio_anim_demo::*;

fn main() {
    App::new()
        .add_plugins(default_plugins())
        .add_plugins(DemoPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, toggle_floor)
        .run();
}

fn toggle_floor(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut floor: Query<&mut Visibility, With<Floor>>,
) {
    if keyboard.just_pressed(KeyCode::KeyF) {
        if let Ok(mut vis) = floor.single_mut() {
            *vis = if *vis == Visibility::Visible {
                Visibility::Hidden
            } else {
                Visibility::Visible
            };
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut clips: ResMut<Assets<AnimationClip>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    spawn_default_camera(&mut commands);

    let planet_name = Name::new("planet");
    let orbit_name = Name::new("orbit");
    let sat_name = Name::new("satellite");

    // ── 创建实体层级（planet 持有 AnimationPlayer）──
    let planet_entity = commands
        .spawn((
            Mesh3d(meshes.add(Sphere::new(0.5).mesh().uv(32, 18))),
            MeshMaterial3d(materials.add(Color::srgb(0.2, 0.6, 1.0))),
            planet_name.clone(),
            AnimationTargetId::from_name(&planet_name),
        ))
        .id();

    let orbit_entity = commands
        .spawn((
            Transform::default(),
            Visibility::default(),
            orbit_name.clone(),
            AnimationTargetId::from_names([planet_name.clone(), orbit_name.clone()].iter()),
            AnimatedBy(planet_entity),
        ))
        .id();

    let satellite_entity = commands
        .spawn((
            Mesh3d(meshes.add(Sphere::new(0.2).mesh().uv(16, 8))),
            MeshMaterial3d(materials.add(Color::srgb(1.0, 0.5, 0.2))),
            Transform::from_xyz(1.5, 0.0, 0.0),
            sat_name.clone(),
            AnimationTargetId::from_names(
                [planet_name.clone(), orbit_name.clone(), sat_name.clone()].iter(),
            ),
            AnimatedBy(planet_entity),
        ))
        .id();

    commands.entity(planet_entity).add_child(orbit_entity);
    commands.entity(orbit_entity).add_child(satellite_entity);

    // planet 自身也要 AnimatedBy 自己（动画系统需要）
    commands
        .entity(planet_entity)
        .insert(AnimatedBy(planet_entity));

    // ── 创建 AnimationClip ──
    let mut clip = AnimationClip::default();

    // Planet: 方形路径位移
    clip.add_curve_to_target(
        AnimationTargetId::from_name(&planet_name),
        AnimatableCurve::new(
            animated_field!(Transform::translation),
            UnevenSampleAutoCurve::new(
                [0.0, 1.0, 2.0, 3.0, 4.0].into_iter().zip([
                    Vec3::new(1.0, 0.0, 1.0),
                    Vec3::new(-1.0, 0.0, 1.0),
                    Vec3::new(-1.0, 0.0, -1.0),
                    Vec3::new(1.0, 0.0, -1.0),
                    Vec3::new(1.0, 0.0, 1.0),
                ]),
            )
            .unwrap(),
        ),
    );

    // Orbit: Y 轴旋转
    clip.add_curve_to_target(
        AnimationTargetId::from_names([planet_name.clone(), orbit_name.clone()].iter()),
        AnimatableCurve::new(
            animated_field!(Transform::rotation),
            UnevenSampleAutoCurve::new(
                [0.0, 2.0, 4.0].into_iter().zip([
                    Quat::IDENTITY,
                    Quat::from_rotation_y(PI),
                    Quat::from_rotation_y(2.0 * PI),
                ]),
            )
            .unwrap(),
        ),
    );

    // Satellite: Y 轴浮动
    clip.add_curve_to_target(
        AnimationTargetId::from_names([planet_name, orbit_name, sat_name].iter()),
        AnimatableCurve::new(
            animated_field!(Transform::translation),
            UnevenSampleAutoCurve::new(
                [0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0]
                    .into_iter()
                    .zip(
                        [0.0, 0.8, 0.0, -0.8, 0.0, 0.8, 0.0, -0.8, 0.0]
                            .map(|y| Vec3::new(1.5, y, 0.0)),
                    ),
            )
            .unwrap(),
        ),
    );

    // ── 构建 AnimationGraph 并播放 ──
    let mut graph = AnimationGraph::new();
    let node = graph.add_clip(clips.add(clip), 1.0, graph.root);
    let graph_handle = graphs.add(graph);

    let mut player = AnimationPlayer::default();
    player.play(node).repeat();
    player.animation_mut(node).unwrap().set_speed(0.5);

    commands
        .entity(planet_entity)
        .insert((AnimationGraphHandle(graph_handle), player));

    spawn_hint(
        &mut commands,
        &asset_server,
        "AnimationClip 程序化动画\n蓝球: 方形路径 | 轨道: 旋转 | 橙球: 浮动\n按 F 隐藏/显示地板",
    );
}
