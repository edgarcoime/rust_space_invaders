mod enemy;
mod player;

use bevy::{app::PluginGroupBuilder, prelude::*};
pub use self::player::*;
pub use self::enemy::*;

pub struct EntitiesPluginGroup;
impl PluginGroup for EntitiesPluginGroup {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(PlayerPlugin)
            .add(EnemyPlugin)
        ;
    }
}
