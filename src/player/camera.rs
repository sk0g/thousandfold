use bevy::{
    color::palettes::css::{
        LIGHT_YELLOW,
        ORANGE_RED,
    },
    light::light_consts::lux,
};

use crate::internal_prelude::*;

pub fn setup_lighting(mut commands: Commands) {
    commands.insert_resource(GlobalAmbientLight {
        color: ORANGE_RED.into(),
        brightness: 200.0,
        ..default()
    });

    commands.spawn((
        "DirectionalLight".as_name(),
        Visibility::Visible,
        Transform::from_xyz(0.0, 4.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        DirectionalLight {
            illuminance: lux::OVERCAST_DAY,
            shadow_maps_enabled: true,
            ..default()
        },
    ));

    commands.spawn((
        "PointLight".as_name(),
        Transform::from_xyz(10.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        PointLight {
            intensity: lux::DIRECT_SUNLIGHT,
            color: LIGHT_YELLOW.into(),
            contact_shadows_enabled: true,
            shadow_maps_enabled: true,
            ..default()
        },
    ));
}

#[derive(Component)]
pub struct MainCamera;
