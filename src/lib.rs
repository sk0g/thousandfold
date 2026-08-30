mod settings;

#[cfg(debug_assertions)]
mod dev;

pub mod internal_prelude {
    pub use bevy::prelude::*;
    pub use extfn::extfn;
}

pub mod prelude {
    pub use bevy::prelude::*;

    pub use crate::settings::*;

    #[cfg(debug_assertions)]
    pub use crate::dev::*;
}
