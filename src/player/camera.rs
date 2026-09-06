use crate::internal_prelude::*;

const DEFAULT_CAMERA_FOV: f32 = 45f32.to_radians();

pub fn camera_and_lighting() -> impl Scene {
    bsn! {
        #Camera3d
        Camera3d

        Projection::custom(PerspectiveProjection {fov: DEFAULT_CAMERA_FOV, ..default()})

        Children [
            #DirectionalLight
            template_value(Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y))
            DirectionalLight {
                shadow_maps_enabled: true,
            },
        ]
    }
}

#[derive(Component)]
pub struct MainCamera;
