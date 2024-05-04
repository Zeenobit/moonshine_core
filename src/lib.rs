#![doc = include_str!("../README.md")]

pub mod save {
    pub use moonshine_save::save::*;

    pub mod prelude {
        pub use moonshine_save::prelude::*;
    }
}

pub mod load {
    pub use moonshine_save::load::*;
}

pub extern crate moonshine_kind as kind;
pub extern crate moonshine_object as object;
pub extern crate moonshine_spawn as spawn;
pub extern crate moonshine_util as util;

pub mod prelude {
    pub use moonshine_kind::prelude::*;
    pub use moonshine_object::prelude::*;
    pub use moonshine_save::prelude::*;
    pub use moonshine_spawn::prelude::*;

    pub use crate::MoonshineCorePlugins;
}

mod plugin;

pub use plugin::MoonshineCorePlugins;
