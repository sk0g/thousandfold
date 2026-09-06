use bevy::{
    math::FloatExt,
    ui::ValNum,
};

pub fn rng_f32(start: f64, end: f64) -> f32 {
    fastrand::f64_inclusive()
        .remap(0.0, 1.0, start, end)
        .val_num_f32()
}

pub fn rng_f64(start: f64, end: f64) -> f64 {
    fastrand::f64_inclusive().remap(0.0, 1.0, start, end)
}
