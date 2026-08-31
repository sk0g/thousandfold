use crate::internal_prelude::*;

pub fn spawn_player_and_camera(mut commands: Commands) {
    commands.spawn(("Player".as_name(), Player, MainCamera, Camera3d::default()));
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct MainCamera;
