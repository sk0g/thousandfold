use std::time::Duration;

use bevy::{
    color::palettes::tailwind,
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

pub fn dev_plugin(app: &mut App) {
    info!("Setting up dev mode");

    app.add_plugins(RemotePlugin::default())
        .add_plugins(RemoteHttpPlugin::default().with_port(15799))
        .add_plugins(InfiniteGridPlugin)
        .add_systems(Startup, setup_infinite_grid_plugin)
        .add_systems(PostStartup, spawn_debug_buildings)
        .add_plugins(get_fps_overlay_plugin());
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
    let colours = [
        tailwind::EMERALD_950,
        tailwind::AMBER_950,
        tailwind::GRAY_900,
        tailwind::GRAY_700,
        tailwind::NEUTRAL_600,
    ];
    let material_handles = colours.map(|c| materials.add(Color::from(c)));

    for _ in 1..=20 {
        let scale = vec3(rng_f32(5.0, 10.0), rng_f32(5.0, 10.0), rng_f32(5.0, 10.0));
        commands.spawn((
            "DebugBuilding".as_name(),
            Mesh3d(meshes.add(Cuboid::default())),
            MeshMaterial3d(
                fastrand::choice(&material_handles)
                    .expect("Random choice between mats")
                    .clone(),
            ),
            Transform::from_xyz(
                rng_f32(-100.0, 100.0),
                -0.15 + scale.y,
                rng_f32(-100.0, 100.0),
            )
            .with_scale(scale),
        ));
    }
}
