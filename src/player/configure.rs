use crate::internal_prelude::*;

pub fn spawn_player_and_camera(mut commands: Commands) {
    commands.spawn((
        "Player".as_name(),
        Visibility::Visible,
        Transform::from_xyz(0.0, 150.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        super::Player,
        super::MainCamera,
        Camera3d::default(),
    ));
    super::setup_lighting(commands);
}
