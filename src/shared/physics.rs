use bevy::prelude::*;

pub struct Velocity {
    value: Vec2,
}

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
    }
}
