use std::time::Duration;

use bevy::{
    dev_tools::{
        fps_overlay::{
            FpsOverlayConfig,
            FpsOverlayPlugin,
            FrameTimeGraphConfig,
        },
        infinite_grid::{
            InfiniteGrid,
            InfiniteGridPlugin,
            InfiniteGridSettings,
        },
    },
    remote::{
        RemotePlugin,
        http::RemoteHttpPlugin,
    },
};

use crate::internal_prelude::*;

#[extfn]
pub fn configure_dev_plugins(self: &mut App) -> &mut App {
    info!("Setting up dev mode");

    self.add_plugins(RemotePlugin::default())
        .add_plugins(RemoteHttpPlugin::default().with_port(15799))
        .add_plugins(InfiniteGridPlugin)
        .add_systems(Startup, setup_infinite_grid_plugin)
        .add_systems(PostStartup, spawn_debug_buildings)
        .add_plugins(get_fps_overlay_plugin())
}

fn get_fps_overlay_plugin() -> FpsOverlayPlugin {
    FpsOverlayPlugin {
        config: FpsOverlayConfig {
            text_config: TextFont {
                font_size: FontSize::Px(18.0),
                weight: FontWeight::LIGHT,
                font_smoothing: FontSmoothing::AntiAliased,
                ..default()
            },
            refresh_interval: Duration::from_millis(50),
            text_color: Color::srgb(0.3, 0.9, 0.3),
            enabled: true,
            frame_time_graph_config: FrameTimeGraphConfig {
                enabled: false,
                ..default()
            },
        },
    }
}

fn setup_infinite_grid_plugin(mut commands: Commands) {
    commands.spawn((InfiniteGrid, InfiniteGridSettings::default()));
}

fn spawn_debug_buildings(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("Spawning debug buildings");
    for _ in 1..=20 {
        let scale = vec3(rng_f32(5.0, 10.0), rng_f32(5.0, 10.0), rng_f32(5.0, 10.0));
        commands.spawn((
            "DebugBuilding".as_name(),
            Mesh3d(meshes.add(Cuboid::default())),
            MeshMaterial3d(materials.add(Color::srgb(0.3, 0.3, 0.3))),
            Transform::from_xyz(
                rng_f32(-100.0, 100.0),
                -0.15 + scale.y,
                rng_f32(-100.0, 100.0),
            )
            .with_scale(scale),
        ));
    }
}
