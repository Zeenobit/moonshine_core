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

pub mod util {
    pub use moonshine_util::*;
}

pub mod tag {
    pub use moonshine_tag::*;
}

pub mod prelude {
    pub use crate::kind::prelude::*;
    pub use crate::object::prelude::*;
    pub use crate::save::prelude::*;
    pub use crate::tag::prelude::*;
    pub use crate::util::prelude::*;

    pub use crate::MoonshineCorePlugins;
}

mod plugin;

pub use plugin::MoonshineCorePlugins;
