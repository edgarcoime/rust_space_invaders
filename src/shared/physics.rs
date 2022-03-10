use bevy::prelude::*;
use heron::PhysicsLayer;

#[derive(PhysicsLayer)]
pub enum Layer {
    Enemy,
    Player,
    Obstacle,
    Projectile,
}

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
    }
}