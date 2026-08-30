use crate::internal_prelude::*;

fn setup_camera(mut commands: Commands) {
    commands.spawn(())
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct MainCamera;
