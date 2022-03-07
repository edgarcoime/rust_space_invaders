mod enemy;
mod player;

use bevy::{app::PluginGroupBuilder, prelude::*};

use self::{enemy::EnemyPlugin, player::PlayerPlugin};

// TODO: Shared resources among "units"
//  - health
//    -- original_health
//    -- current_health
//  - xp
//  - damage

// Must export not just a plugin but a multitude of plugins

// region:      Resources
// endregion:   Resources

// region:      Components
#[derive(Component)]
struct Health {
    original_hp: u32,
    current_hp: u32,
}

#[derive(Component)]
struct ExperiencePoints(u32);

// endregion:   Components

// region:      Entities
// endregion:   Entities

pub struct UnitsPluginGroup;
impl PluginGroup for UnitsPluginGroup {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(EnemyPlugin)
            .add(PlayerPlugin)
            ;
    }
}
