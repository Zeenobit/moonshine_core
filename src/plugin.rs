use bevy_app::{PluginGroup, PluginGroupBuilder};

/// A [`PluginGroup`] which adds all the core plugins.
pub struct MoonshineCorePlugins;

impl PluginGroup for MoonshineCorePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(crate::save::SavePlugin)
            .add(crate::load::LoadPlugin)
            .add(
                #[allow(deprecated)] // TODO: Remove after 0.2.2
                crate::spawn::SpawnPlugin,
            )
    }
}
