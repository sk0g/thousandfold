mod common;
mod misc;
mod player;
mod plugins;
mod settings;

pub mod internal_prelude {
    pub use bevy::prelude::*;
    pub use extfn::extfn;

    pub use crate::common::*;
}

pub mod prelude {
    pub use bevy::prelude::*;

    pub use crate::{
        misc::*,
        player::*,
        plugins::*,
        settings::*,
    };
}
