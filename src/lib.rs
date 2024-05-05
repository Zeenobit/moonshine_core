#![doc = include_str!("../README.md")]

pub mod kind {
    pub use moonshine_kind::*;
}

pub mod object {
    pub use moonshine_object::*;
}

pub mod save {
    pub use moonshine_save::save::*;

    pub mod prelude {
        pub use moonshine_save::prelude::*;
    }
}

pub mod load {
    pub use moonshine_save::load::*;
}

pub mod spawn {
    pub use moonshine_spawn::*;
}

pub mod util {
    pub use moonshine_util::*;
}

pub mod prelude {
    pub use crate::kind::prelude::*;
    pub use crate::object::prelude::*;
    pub use crate::save::prelude::*;
    pub use crate::spawn::prelude::*;

    pub use crate::MoonshineCorePlugins;
}

mod plugin;

pub use plugin::MoonshineCorePlugins;
