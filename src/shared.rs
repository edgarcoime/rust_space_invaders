mod weapons;
mod attributes;
mod physics;

use bevy::{app::PluginGroupBuilder, prelude::*};
pub use self::weapons::*;
pub use self::physics::*;
pub use self::attributes::*;

pub struct SharedPluginGroup;
impl PluginGroup for SharedPluginGroup {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(PhysicsPlugin)
            .add(WeaponsPlugin)
            .add(AttributesPlugin);
    }
}
