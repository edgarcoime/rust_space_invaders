mod weapons;

use bevy::{app::PluginGroupBuilder, prelude::*};

use self::weapons::WeaponsPlugin;

pub struct SharedPluginGroup;
impl PluginGroup for SharedPluginGroup {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(WeaponsPlugin);
    }
}
