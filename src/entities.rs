mod enemy;
mod player;

use bevy::{app::PluginGroupBuilder, prelude::*};
use self::{player::PlayerPlugin, enemy::EnemyPlugin};

pub struct EntitiesPluginGroup;
impl PluginGroup for EntitiesPluginGroup {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(PlayerPlugin)
            .add(EnemyPlugin)
        ;
    }
}
