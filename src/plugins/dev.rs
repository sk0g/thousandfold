use bevy::dev_tools::{
    fps_overlay::FpsOverlayPlugin,
    infinite_grid::InfiniteGridPlugin,
};

use crate::internal_prelude::*;

#[extfn]
pub fn configure_dev_plugins(self: &mut App) -> &mut App {
    info!("Setting up dev mode");

    self.add_plugins(InfiniteGridPlugin)
        .add_plugins(FpsOverlayPlugin::default())
}
