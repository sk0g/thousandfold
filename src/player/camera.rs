use bevy::{
    color::palettes::css::LIGHT_YELLOW,
    light::light_consts::lux,
};

use crate::internal_prelude::*;

const DEFAULT_CAMERA_FOV: f32 = 45f32.to_radians();

pub fn camera_and_lighting() -> impl Scene {
    bsn! {
        #Camera3d
        Camera3d

        Projection::custom(PerspectiveProjection {fov: DEFAULT_CAMERA_FOV, ..default()})

        Children [
            #DirectionalLight
            Transform::from_xyz(0.0, 4.0, 0.0)
            DirectionalLight {
                illuminance: lux::DIRECT_SUNLIGHT,
                shadow_maps_enabled: true,
            },

            #PointLight
            Transform::from_xyz(10.0, 10.0, 0.0)
            PointLight {
                intensity: lux::DIRECT_SUNLIGHT,
                color: LIGHT_YELLOW,
                contact_shadows_enabled: true,
                shadow_maps_enabled: true,
            }
        ]
    }
}

#[derive(Component)]
pub struct MainCamera;
