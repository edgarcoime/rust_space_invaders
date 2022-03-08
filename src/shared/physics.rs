use bevy::prelude::*;

use crate::{WinSize, GAME_TIME_STEP};

use super::Projectile;

#[derive(Component)]
pub struct Velocity {
    value: Vec2,
}
impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            value: Vec2::new(x, y)
        }
    }
}

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(move_all_projectiles)
        ;
    }
}

fn move_all_projectiles(
    mut commands: Commands,
    mut q: Query<(Entity, &Velocity, &mut Transform), With<Projectile>>,
    win_size: Res<WinSize>,
) {
    for (entity, vel, mut tf) in q.iter_mut() {
        tf.translation.y += vel.value.y * GAME_TIME_STEP;
        tf.translation.x += vel.value.x * GAME_TIME_STEP;

        if  tf.translation.y.abs() > win_size.h || 
            tf.translation.x.abs() > win_size.w {
                commands.entity(entity).despawn();
        }
    }
}