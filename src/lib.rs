pub mod prelude {
    pub use bevy::prelude::*;

    mod some_mod {
        const SOME_CONST: i32 = 10;
    }
}
