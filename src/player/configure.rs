use crate::{
    internal_prelude::*,
    player::camera_and_lighting,
};

pub fn spawn_player_and_camera(mut commands: Commands) {
    let player = commands
        .spawn((
            "Player".as_name(),
            Visibility::Visible,
            Transform::from_xyz(0.0, 150.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            super::Player,
            super::MainCamera,
        ))
        .id();

    commands
        .spawn_scene(camera_and_lighting())
        .set_parent_in_place(player);
}
