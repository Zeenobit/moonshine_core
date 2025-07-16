use bevy_app::{PluginGroup, PluginGroupBuilder};

/// A [`PluginGroup`] which adds all the core plugins.
pub struct MoonshineCorePlugins;

impl PluginGroup for MoonshineCorePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(crate::tag::TagPlugin)
    }
}
