mod weapons;
mod physics;
mod attributes;

use bevy::{app::PluginGroupBuilder, prelude::*};
pub use self::weapons::*;
pub use self::attributes::*;
pub use self::physics::*;

#[derive(Component)]
pub struct RealAssetSize {
    pub width: f32,
    pub height: f32,
}
impl RealAssetSize {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub fn to_vec2(&self) -> Vec2 {
        Vec2::new(self.width, self.height)
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
