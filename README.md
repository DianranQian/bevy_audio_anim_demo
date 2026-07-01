# Bevy 音频与动画示例项目

演示 Bevy 0.19 的音频和动画 API，资产来自 Bevy 官方仓库。

## 运行命令

```bash
# 音频基础 — AudioPlayer + LOOP/DESPAWN
cargo run --example audio_basic

# 音频控制 — AudioSink 暂停/音量/变速
cargo run --example audio_control

# 空间音频 — 3D 距离衰减 + 方位声像
cargo run --example audio_spatial

# 动画播放 — AnimationPlayer + Fox.glb 模型动画
cargo run --example anim_basic

# 程序化动画 — AnimationClip + VariableCurve（无外部模型）
cargo run --example anim_clip
```

## 示例说明

### audio_basic
- `AudioPlayer::new()` 播放音频
- `PlaybackSettings::LOOP` 循环播放
- `PlaybackSettings::DESPAWN` 播完自动清理实体
- 按 P 暂停/恢复

### audio_control
- `AudioSink::toggle()` 暂停/恢复
- `AudioSink::set_volume()` 音量控制
- `AudioSink::set_speed()` 播放速度
- 空格暂停，↑↓ 音量，←→ 速度，R 重置

### audio_spatial
- `PlaybackSettings::LOOP.with_spatial(true)` 空间音频
- `SpatialListener` 听者（左耳/右耳）
- 声源围绕听者旋转，自动演示距离衰减和方位声像

### anim_basic
- 加载 `Fox.glb` 带骨骼动画模型
- `AnimationPlayer` 播放/暂停/变速
- 按 1/2/3 切换动画（观察/走/跑），空格暂停，+/- 变速

### anim_clip
- 程序化创建 `AnimationClip`
- `AnimatableCurve` + `UnevenSampleAutoCurve` 自定义曲线
- 驱动 Transform 的 translation 和 rotation
- 不依赖外部模型文件

## 资产来源

- `assets/sounds/windless_slopes.ogg` — Bevy 官方背景音乐
- `assets/sounds/breakout_collision.ogg` — Bevy 官方音效
- `assets/models/animated/Fox.glb` — Bevy 官方狐狸模型（3 个动画）
