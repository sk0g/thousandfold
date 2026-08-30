mod settings;

mod plugins;

pub mod internal_prelude {
    pub use bevy::prelude::*;

    pub use extfn::extfn;
}

pub mod prelude {
    pub use bevy::prelude::*;

    pub use crate::plugins::*;
    pub use crate::settings::*;
}
