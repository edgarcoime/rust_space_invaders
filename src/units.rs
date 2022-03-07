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
// endregion:   Constants

// region:      Resources
// endregion:   Resources

// region:      Components
#[derive(Component)]
pub struct MovementSpeed(f32);

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
pub struct Velocity(Vec2);
impl Default for Velocity {
    fn default() -> Self {
        Self(Vec2::new(0., 600.))
    }
}
impl Velocity {
    fn from(vel: Vec2) -> Self {
        Self(vel)
    }

    fn normal_projectile() -> Self {
        Self(Vec2::new(0., 600.))
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
