use std::time::Instant;

use bevy::log::{
    BoxedFmtLayer,
    tracing_subscriber,
    tracing_subscriber::{
        field::MakeExt,
        fmt::time::FormatTime,
    },
};

use crate::internal_prelude::*;

#[expect(
    clippy::unnecessary_wraps,
    reason = "No way around this, fmt layer is reuqired to be Option<Box<...>"
)]
pub fn custom_fmt_layer(_app: &mut App) -> Option<BoxedFmtLayer> {
    let layer = tracing_subscriber::fmt::Layer::default()
        .with_timer(CustomUptime::new())
        .map_fmt_fields(MakeExt::debug_alt)
        .with_writer(std::io::stderr);
    Some(Box::new(layer))
}

pub struct CustomUptime {
    pub epoch: Instant,
}

impl CustomUptime {
    pub fn new() -> Self {
        Self {
            epoch: Instant::now(),
        }
    }
}

impl FormatTime for CustomUptime {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
        let elapsed = self.epoch.elapsed();
        let seconds = elapsed.as_secs_f64();
        write!(w, "{seconds:.6}s")
    }
}
