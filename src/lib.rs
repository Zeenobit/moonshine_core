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

pub mod check {
    pub use moonshine_check::*;
}

#[deprecated(
    since = "0.2.2",
    note = "see documentation at https://github.com/Zeenobit/moonshine_spawn for details"
)]
pub mod spawn {
    #[allow(deprecated)] // TODO: Remove after 0.2.2
    pub use moonshine_spawn::*;
}

pub mod util {
    pub use moonshine_util::*;
}

pub mod prelude {
    pub use crate::check::prelude::*;
    pub use crate::kind::prelude::*;
    pub use crate::object::prelude::*;
    pub use crate::save::prelude::*;
    pub use crate::util::prelude::*;

    pub use crate::MoonshineCorePlugins;

    #[deprecated(
        since = "0.2.2",
        note = "see documentation at https://github.com/Zeenobit/moonshine_spawn for details"
    )]
    #[allow(deprecated)] // TODO: Remove after 0.2.2
    pub use crate::spawn::prelude::*;
}

mod plugin;

pub use plugin::MoonshineCorePlugins;
