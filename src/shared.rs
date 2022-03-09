mod weapons;
mod physics;
mod attributes;

use bevy::{app::PluginGroupBuilder, prelude::*};
pub use self::weapons::*;
pub use self::attributes::*;
pub use self::physics::*;

#[derive(Component)]
pub struct RenderedAssetInfo {
    pub size: Vec2,
}
impl RenderedAssetInfo {
    pub fn new(size: Vec2) -> Self {
        Self { size }
    }
}

pub struct SharedPluginGroup;
impl PluginGroup for SharedPluginGroup {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(PhysicsPlugin)
            .add(AttributesPlugin)
            .add(WeaponsPlugin);
    }
}
