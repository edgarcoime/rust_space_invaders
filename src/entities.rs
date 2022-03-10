mod enemy;
mod player;
mod obstacles;

use bevy::{app::PluginGroupBuilder, prelude::*};
use self::{player::PlayerPlugin, enemy::EnemyPlugin, obstacles::ObstaclesPlugin};

pub struct EntitiesPluginGroup;
impl PluginGroup for EntitiesPluginGroup {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(PlayerPlugin)
            .add(EnemyPlugin)
            .add(ObstaclesPlugin)
        ;
    }
}
