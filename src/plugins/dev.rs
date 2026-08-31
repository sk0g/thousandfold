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

    self.add_plugins(InfiniteGridPlugin)
        .add_systems(Startup, setup_infinite_grid_plugin)
        .add_plugins(get_fps_overlay_plugin())
        .add_plugins(RemotePlugin::default())
        .add_plugins(RemoteHttpPlugin::default().with_port(15799))
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
