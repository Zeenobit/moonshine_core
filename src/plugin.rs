use bevy_app::{PluginGroup, PluginGroupBuilder};

pub struct MoonshineCorePlugins;

impl PluginGroup for MoonshineCorePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(crate::save::SavePlugin)
            .add(crate::load::LoadPlugin)
            .add(crate::spawn::SpawnPlugin)
    }
}
