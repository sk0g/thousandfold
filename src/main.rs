use bevy::{
    log::LogPlugin,
    window::WindowResolution,
};
use thousandfold::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::from(get_resolution()),
                    ..default()
                }),
                ..default()
            })
            .set(LogPlugin {
                fmt_layer: custom_fmt_layer,
                ..default()
            }),
    );

    #[cfg(debug_assertions)]
    {
        app.configure_dev_plugins();
    }

    app.add_systems(Startup, spawn_player_and_camera).run();
}
