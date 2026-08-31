mod common;
mod player;
mod plugins;
mod settings;
mod utils;

pub mod internal_prelude {
    pub use bevy::prelude::*;
    pub use extfn::extfn;

    pub use crate::common::*;
}

pub mod prelude {
    pub use bevy::prelude::*;

    pub use crate::{
        player::*,
        plugins::*,
        settings::*,
        utils::*,
    };
}
