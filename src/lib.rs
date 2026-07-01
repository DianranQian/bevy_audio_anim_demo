//! 共享工具：背景色、相机、灯光、地平面、UI 提示。

use bevy::prelude::*;

/// 动画背景色 #1A1A2E
pub const BG_COLOR: Color = Color::srgb(0.102, 0.102, 0.180);

/// 中文字体路径（文泉驿微米黑，无日文语言表）
pub const FONT_PATH: &str = "fonts/wqy-microhei.ttc";

/// 创建 DefaultPlugins（ICU4X 警告是 Bevy 内部问题，无法过滤）
pub fn default_plugins() -> impl PluginGroup {
    DefaultPlugins
}

/// 地板标记组件
#[derive(Component)]
pub struct Floor;

/// 插件：设置背景色、灯光、地平面（不含相机）
pub struct DemoPlugin;

impl Plugin for DemoPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BG_COLOR))
            .add_systems(Startup, setup_base);
    }
}

fn setup_base(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        DirectionalLight {
            illuminance: 8000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -0.5)),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.15, 0.15, 0.22))),
        Floor,
    ));
}

/// 默认相机
pub fn spawn_default_camera(commands: &mut Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(4.0, 4.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

/// 创建使用中文字体的 TextFont
pub fn cn_font(asset_server: &Res<AssetServer>, size: f32) -> TextFont {
    TextFont {
        font: asset_server.load(FONT_PATH).into(),
        font_size: FontSize::Px(size),
        ..default()
    }
}

/// 屏幕底部操作提示
pub fn spawn_hint(commands: &mut Commands, asset_server: &Res<AssetServer>, text: &str) {
    commands.spawn((
        Text::new(text),
        cn_font(asset_server, 16.0),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

/// 屏幕顶部标题
pub fn spawn_title(commands: &mut Commands, asset_server: &Res<AssetServer>, text: &str) {
    commands.spawn((
        Text::new(text),
        cn_font(asset_server, 20.0),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}
