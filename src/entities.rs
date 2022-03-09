mod enemy;
mod player;
mod obstacles;

use bevy::{app::PluginGroupBuilder, prelude::*};
pub use self::player::*;
pub use self::enemy::*;
pub use self::obstacles::*;

pub struct EntitiesPluginGroup;
impl PluginGroup for EntitiesPluginGroup {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(EntitiesPlugin)
            .add(ObstaclesPlugin)
            .add(PlayerPlugin)
            .add(EnemyPlugin)
        ;
    }
}

pub struct EntitiesPlugin;
impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
    }
}