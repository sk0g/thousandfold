use bevy_enhanced_input::prelude::*;

use crate::internal_prelude::*;

const MOVEMENT_SCALE: f32 = 10.0;
const MOVEMENT_SCALE_MIN_HEIGHT: f32 = 0.8;
const MOVEMENT_SCALE_MAX_HEIGHT: f32 = 4.0;
const ZOOM_SCALE: f32 = 5.0;

const MIN_HEIGHT: f32 = 50.0;
const MAX_HEIGHT: f32 = 300.0;
const HEIGHT_DECREASE_SCALING_STARTS_AT: f32 = 100.0;
const HEIGHT_INCREASE_SCALING_STARTS_AT: f32 = 250.0;

#[extfn]
pub fn configure_input(self: &mut App) -> &mut App {
    self.add_plugins(EnhancedInputPlugin)
        .add_input_context::<GameplayContext>()
        .add_systems(Startup, setup_gameplay_input_context)
        .add_observer(on_camera_move)
        .add_observer(on_camera_zoom)
}

fn setup_gameplay_input_context(mut commands: Commands) {
    commands.spawn((
        "GameplayContext".as_name(),
        GameplayContext,
        actions!(
            GameplayContext[(
                Action::<CameraMove>::new(),
                DeadZone::default(),
                SmoothNudge::default(),
                DeltaScale::default(),
                Scale::splat(MOVEMENT_SCALE),
                Bindings::spawn(Cardinal::wasd_keys()),
            ),
            (
                Action::<CameraZoom>::new(),
                DeadZone::default(),
                SmoothNudge::default(),
                Scale::splat(ZOOM_SCALE),
                Bindings::spawn((
                    Spawn((Binding::mouse_wheel(), SwizzleAxis::YXZ)),
                )),
            ),
        ]),
    ));
}

#[derive(Component)]
struct GameplayContext;

#[derive(InputAction, Debug)]
#[action_output(Vec2)]
struct CameraMove;

fn on_camera_move(
    input: On<Fire<CameraMove>>,
    mut player_transform: Single<&mut Transform, With<super::Player>>,
) {
    let current_height = player_transform.translation.y;

    let scale_multiplier = current_height.remap(
        MIN_HEIGHT,
        MAX_HEIGHT,
        MOVEMENT_SCALE_MIN_HEIGHT,
        MOVEMENT_SCALE_MAX_HEIGHT,
    );

    player_transform.translation -= input.value.extend(0.0).yzx() * scale_multiplier;
}

#[derive(InputAction, Debug)]
#[action_output(f32)]
struct CameraZoom;

fn on_camera_zoom(
    input: On<Fire<CameraZoom>>,
    mut player_transform: Single<&mut Transform, With<super::Player>>,
) {
    // zoom in - positive value - lower height
    let current_height = player_transform.translation.y;

    let val = input.value;

    let scale_multiplier = if val > 0.0 && current_height < HEIGHT_DECREASE_SCALING_STARTS_AT {
        current_height.remap(HEIGHT_DECREASE_SCALING_STARTS_AT, MIN_HEIGHT, 1.0, 0.0)
    } else if val < 0.0 && current_height > HEIGHT_INCREASE_SCALING_STARTS_AT {
        current_height.remap(HEIGHT_INCREASE_SCALING_STARTS_AT, MAX_HEIGHT, 1.0, 0.0)
    } else {
        1.0
    }
    .powi(2);

    player_transform.translation.y -= val * scale_multiplier;
}
