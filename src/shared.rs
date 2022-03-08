mod weapons;
mod physics;
mod attributes;

use bevy::{app::PluginGroupBuilder, prelude::*};
pub use self::weapons::*;
pub use self::attributes::*;
pub use self::physics::*;

pub struct SharedPluginGroup;
impl PluginGroup for SharedPluginGroup {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(PhysicsPlugin)
            .add(AttributesPlugin)
            .add(WeaponsPlugin);
    }
}
