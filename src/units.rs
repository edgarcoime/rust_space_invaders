mod enemy;
mod player;
mod weapons;

use bevy::{app::PluginGroupBuilder, prelude::*};

use self::{enemy::EnemyPlugin, player::PlayerPlugin, weapons::WeaponsPlugin};

// TODO: Shared resources among "units"
//  - health
//    -- original_health
//    -- current_health
//  - xp
//  - damage

// Must export not just a plugin but a multitude of plugins

// region:      Constants
const DEFAULT_WEAPON_COOLDOWN: f32 = 100.;
// endregion:   Constants

// region:      Resources
// endregion:   Resources

// region:      Components
#[derive(Component)]
struct Health {
    original_hp: u32,
    current_hp: u32,
}
impl Default for Health {
    fn default() -> Self {
        Self { original_hp: 1, current_hp: 1 }
    }
}

#[derive(Component)]
struct ExperiencePoints(u32);

#[derive(Component)]
pub struct Velocity(f32);
impl Default for Velocity {
    fn default() -> Self {
        Self(250.)
    }
}
impl Velocity {
    fn normal_projectile() -> Self {
        Self(600.)
    }
}



// endregion:   Components

// region:      Entities
// endregion:   Entities

pub struct UnitsPluginGroup;
impl PluginGroup for UnitsPluginGroup {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(PlayerPlugin)
            .add(EnemyPlugin)
            .add(WeaponsPlugin)
            ;
    }
}
