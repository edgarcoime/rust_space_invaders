mod enemy;
mod player;
mod obstacles;

use bevy::{app::PluginGroupBuilder, prelude::*};
use heron::CollisionLayers;
use heron::CollisionShape;
use heron::RigidBody;
use crate::shared::Health;
use crate::shared::MovementSpeed;
use crate::shared::WeaponState;

pub use self::enemy::*;
pub use self::player::*;
pub use self::obstacles::*;

#[derive(Bundle)]
pub struct BasicShipBundle {
    _hp: Health,
    _ms: MovementSpeed,
    _ws: WeaponState
}
impl Default for BasicShipBundle {
    fn default() -> Self {
        Self {
            _hp: Health::default(),
            _ms: MovementSpeed::default(),
            _ws: WeaponState::fast_normal_weapon(),
        }
    }
}

#[derive(Bundle)]
pub struct EntityPhysicsBundle {
    _rb: RigidBody,
    _cs: CollisionShape,
    _cl: CollisionLayers,
}


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
