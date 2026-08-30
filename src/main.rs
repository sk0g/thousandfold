use bevy::window::WindowResolution;
use thousandfold::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::from(get_resolution()),
            ..default()
        }),
        ..default()
    }));

    #[cfg(debug_assertions)]
    {
        get_dev_plugins(); // TODO
    }

    app.run();
}
