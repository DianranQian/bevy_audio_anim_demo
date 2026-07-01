//! AnimationPlayer 基础演示 — 加载 Fox.glb
//!
//! 运行：cargo run --example anim_basic

use std::time::Duration;

use bevy::prelude::*;
use bevy::world_serialization::WorldInstanceReady;
use bevy_audio_anim_demo::*;

fn main() {
    App::new()
        .add_plugins(default_plugins())
        .insert_resource(ClearColor(BG_COLOR))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            spawn_fox_when_ready.run_if(not(resource_exists::<Animations>)),
        )
        .add_systems(
            Update,
            (keyboard_control, update_status).run_if(resource_exists::<Animations>),
        )
        .run();
}

const FOX_PATH: &str = "models/animated/Fox.glb";

#[derive(Resource)]
struct Fox(Handle<Gltf>);

#[derive(Resource)]
struct Animations {
    node_indices: Vec<AnimationNodeIndex>,
    graph_handle: Handle<AnimationGraph>,
}

#[derive(Component)]
struct StatusText;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 相机 — Fox 模型场景尺寸较大
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(100.0, 100.0, 150.0).looking_at(Vec3::new(0.0, 20.0, 0.0), Vec3::Y),
    ));

    // 灯光
    commands.spawn((
        DirectionalLight {
            illuminance: 8000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -0.5)),
    ));

    // 大地平面
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(500.0, 500.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));

    // 触发 glTF 加载
    commands.insert_resource(Fox(asset_server.load(FOX_PATH)));

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
        "AnimationPlayer 演示 (Fox.glb)\n1: Survey | 2: Walk | 3: Run\n空格: 暂停/恢复 | +/-: 变速",
    );
}

fn spawn_fox_when_ready(
    mut commands: Commands,
    fox: Res<Fox>,
    asset_server: Res<AssetServer>,
    gltfs: Res<Assets<Gltf>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    if !asset_server.is_loaded_with_dependencies(&fox.0) {
        return;
    }
    let gltf = gltfs.get(&fox.0).unwrap();

    let (graph, node_indices) = AnimationGraph::from_clips([
        gltf.animations[0].clone(),
        gltf.animations[1].clone(),
        gltf.animations[2].clone(),
    ]);
    let graph_handle = graphs.add(graph);
    commands.insert_resource(Animations {
        node_indices,
        graph_handle,
    });

    let scene = gltf.default_scene.clone().unwrap();
    commands
        .spawn(WorldAssetRoot(scene))
        .observe(setup_scene);
}

fn setup_scene(
    _ready: On<WorldInstanceReady>,
    mut commands: Commands,
    animations: Res<Animations>,
    player: Single<(Entity, &mut AnimationPlayer)>,
) {
    let (entity, mut player) = player.into_inner();
    let mut transitions = AnimationTransitions::new();
    transitions
        .play(&mut player, animations.node_indices[0], Duration::ZERO)
        .repeat();
    commands
        .entity(entity)
        .insert(AnimationGraphHandle(animations.graph_handle.clone()))
        .insert(transitions);
}

fn keyboard_control(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    animations: Res<Animations>,
) {
    let Ok((mut player, mut transitions)) = players.single_mut() else { return };
    let Some((&idx, _)) = player.playing_animations().next() else { return };

    if keyboard.just_pressed(KeyCode::Space) {
        let anim = player.animation_mut(idx).unwrap();
        if anim.is_paused() {
            anim.resume();
        } else {
            anim.pause();
        }
    }
    if keyboard.just_pressed(KeyCode::Equal) || keyboard.just_pressed(KeyCode::NumpadAdd) {
        let anim = player.animation_mut(idx).unwrap();
        anim.set_speed((anim.speed() + 0.25).min(3.0));
    }
    if keyboard.just_pressed(KeyCode::Minus) || keyboard.just_pressed(KeyCode::NumpadSubtract) {
        let anim = player.animation_mut(idx).unwrap();
        anim.set_speed((anim.speed() - 0.25).max(0.25));
    }
    if keyboard.just_pressed(KeyCode::Digit1) {
        transitions.play(&mut player, animations.node_indices[0], Duration::from_millis(300)).repeat();
    }
    if keyboard.just_pressed(KeyCode::Digit2) {
        transitions.play(&mut player, animations.node_indices[1], Duration::from_millis(300)).repeat();
    }
    if keyboard.just_pressed(KeyCode::Digit3) {
        transitions.play(&mut player, animations.node_indices[2], Duration::from_millis(300)).repeat();
    }
}

fn update_status(
    players: Query<&AnimationPlayer>,
    mut status: Query<&mut Text, With<StatusText>>,
) {
    let Ok(player) = players.single() else { return };
    let (state, speed) = if let Some((&idx, _)) = player.playing_animations().next() {
        let anim = player.animation(idx).unwrap();
        (if anim.is_paused() { "已暂停" } else { "播放中" }, anim.speed())
    } else {
        ("无动画", 0.0)
    };
    if let Ok(mut t) = status.single_mut() {
        **t = format!("状态: {}\n速度: {:.2}x", state, speed);
    }
}
