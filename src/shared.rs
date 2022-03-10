mod weapons;
mod attributes;
mod physics;

use bevy::{app::PluginGroupBuilder, prelude::*};
use self::{weapons::WeaponsPlugin, attributes::AttributesPlugin, physics::PhysicsPlugin};

pub struct SharedPluginGroup;
impl PluginGroup for SharedPluginGroup {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(PhysicsPlugin)
            .add(WeaponsPlugin)
            .add(AttributesPlugin);
    }
}
