use std::collections::HashSet;

use bevy::{prelude::*, render::render_phase::EntityPhaseItem};
use heron::{PhysicsLayer, CollisionLayers, CollisionEvent};

#[derive(PhysicsLayer)]
pub enum WorldPhysicsLayer {
    Enemy,
    Player,
    FriendlyProjectile,
    EnemyProjectile,
    Projectile,
    Obstacle,
}

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(manage_projectile_hit_obstacles)
        ;
    }
}

fn manage_projectile_hit_obstacles(mut commands: Commands, mut events: EventReader<CollisionEvent>) {
    let mut entities_despawned: HashSet<Entity> = HashSet::new();
    events
        .iter()
        .filter(|e| e.is_started())
        .filter_map(|event| {
            let (entity_1, entity_2) = event.rigid_body_entities();
            let (layers_1, layers_2) = event.collision_layers();
            if is_obstacle(layers_1) && is_projectile(layers_2) {
                Some((entity_1, entity_2))
            } else if is_projectile(layers_1) && is_obstacle(layers_2) {
                Some((entity_2, entity_1))
            } else {
                None
            }
        })
        .for_each(|(obj_en, proj_en)| {
            if (entities_despawned.get(&obj_en)).is_none() {
                commands.entity(obj_en).despawn();
                entities_despawned.insert(obj_en);
            }

            if (entities_despawned.get(&proj_en)).is_none() {
                commands.entity(proj_en).despawn();
                entities_despawned.insert(proj_en);
            }
        });
}

pub fn is_player(layers: CollisionLayers) -> bool {
    layers.contains_group(WorldPhysicsLayer::Player) && 
    !layers.contains_group(WorldPhysicsLayer::Enemy)
}

pub fn is_enemy(layers: CollisionLayers) -> bool {
    !layers.contains_group(WorldPhysicsLayer::Player) && 
    layers.contains_group(WorldPhysicsLayer::Enemy)
}

pub fn is_obstacle(layers: CollisionLayers) -> bool {
    layers.contains_group(WorldPhysicsLayer::Obstacle)
}

pub fn is_projectile(layers: CollisionLayers) -> bool {
    layers.contains_group(WorldPhysicsLayer::Projectile)
}