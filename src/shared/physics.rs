use std::collections::HashSet;

use bevy::{prelude::*};
use heron::{PhysicsLayer, CollisionLayers, CollisionEvent};

use crate::{entities::{Obstacle, Enemy, Friendly}, utils::get_components_for_entity};

use super::{Projectile, Health};

#[derive(PhysicsLayer)]
pub enum WorldPhysicsLayer {
    Enemy,
    Player,
    Friendly,
    FriendlyProjectile,
    HostileProjectile,
    Projectile,
    Obstacle,
}

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(manage_projectile_hit_obstacles)
            .add_system_set( // Projectile Logic
                SystemSet::new()
                    .with_system(manage_projectile_hit_obstacles)
                    .with_system(manage_friendly_projectiles_hit_enemy)
                    .with_system(manage_hostile_projectiles_hit_friendly)
            )
            .add_system_set(
                SystemSet::new()
                    .with_system(manage_enemy_hit_obstacles)
                    .with_system(manage_enemy_hit_friendly)
            )
        ;
    }
}

fn manage_projectile_hit_obstacles(
    mut commands: Commands, 
    mut events: EventReader<CollisionEvent>,
    mut obs_q: Query<&mut Health, With<Obstacle>>,
    proj_q: Query<&Projectile, With<Projectile>>,
) {
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
        .for_each(|(obs_en, proj_en)| {
            if let (Ok(mut obs_hp), Ok(projectile)) 
                = (obs_q.get_mut(obs_en), proj_q.get(proj_en)) 
                {
                    // calculate damage and despawn
                    if entities_despawned.get(&proj_en).is_none() 
                    {
                        entities_despawned.insert(proj_en);
                        obs_hp.current_hp -= projectile.damage;
                        commands.entity(proj_en).despawn();
                    }

                    if  entities_despawned.get(&obs_en).is_none() 
                        && obs_hp.dead()
                    {
                        entities_despawned.insert(obs_en);
                        commands.entity(obs_en).despawn();
                    }
                }
        });
}

fn manage_friendly_projectiles_hit_enemy (
    mut commands: Commands, 
    mut events: EventReader<CollisionEvent>,
    mut enemy_q: Query<&mut Health, With<Enemy>>,
    proj_q: Query<&Projectile, With<Projectile>>,
) {
    let mut entities_despawned: HashSet<Entity> = HashSet::new();
    events
        .iter()
        .filter(|e| e.is_started())
        .filter_map(|event| {
            let (entity_1, entity_2) = event.rigid_body_entities();
            let (layers_1, layers_2) = event.collision_layers();

            if is_friendly_projectile(layers_1) && is_enemy(layers_2) {
                Some((entity_1, entity_2))
            } else if is_enemy(layers_1) && is_friendly_projectile(layers_2) {
                Some((entity_2, entity_1))
            } else {
                None
            }
        })
        .for_each(|(proj_en, enemy_en)| {
            if let (Ok(projectile), Ok(mut enemy_hp)) 
                = (proj_q.get(proj_en), enemy_q.get_mut(enemy_en)) 
                {
                    println!("Running FRIENDLY projectile hit ENEMY logic");
                    // calculate damage and despawn
                    if (entities_despawned.get(&proj_en)).is_none() 
                    {
                        enemy_hp.current_hp -= projectile.damage;
                        commands.entity(proj_en).despawn();
                        entities_despawned.insert(proj_en);
                    }

                    if  (entities_despawned.get(&enemy_en)).is_none() 
                        && enemy_hp.dead()
                    {
                        commands.entity(enemy_en).despawn();
                        entities_despawned.insert(enemy_en);
                    }
                }
        });
}

fn manage_hostile_projectiles_hit_friendly (
    mut commands: Commands, 
    mut events: EventReader<CollisionEvent>,
    mut friendly_q: Query<&mut Health, With<Friendly>>,
    proj_q: Query<&Projectile, With<Projectile>>,
) {
    let mut entities_despawned: HashSet<Entity> = HashSet::new();
    events
        .iter()
        .filter(|e| e.is_started())
        .filter_map(|event| {
            let (entity_1, entity_2) = event.rigid_body_entities();
            let (layers_1, layers_2) = event.collision_layers();

            if is_hostile_projectile(layers_1) && is_friendly(layers_2) {
                Some((entity_1, entity_2))
            } else if is_friendly(layers_1) && is_hostile_projectile(layers_2) {
                Some((entity_2, entity_1))
            } else {
                None
            }
        })
        .for_each(|(proj_en, friendly_en)| {
            if let (Ok(projectile), Ok(mut friendly_hp)) 
                = (proj_q.get(proj_en), friendly_q.get_mut(friendly_en)) 
                {
                    println!("Running HOSTILE projectile hit FRIENDLY logic");

                    // calculate damage and despawn
                    if (entities_despawned.get(&proj_en)).is_none() 
                    {
                        friendly_hp.current_hp -= projectile.damage;
                        commands.entity(proj_en).despawn();
                        entities_despawned.insert(proj_en);
                    }

                    if  (entities_despawned.get(&friendly_en)).is_none() 
                        && friendly_hp.dead()
                    {
                        commands.entity(friendly_en).despawn();
                        entities_despawned.insert(friendly_en);
                    }
                }
            }
        );
}

fn manage_enemy_hit_obstacles(
    mut commands: Commands, 
    mut events: EventReader<CollisionEvent>,
) {
    let mut entities_despawned: HashSet<Entity> = HashSet::new();
    events
        .iter()
        .filter(|e| e.is_started())
        .filter_map(|event| {
            let (entity_1, entity_2) = event.rigid_body_entities();
            let (layers_1, layers_2) = event.collision_layers();
            if is_obstacle(layers_1) && is_enemy(layers_2) {
                Some((entity_1, entity_2))
            } else if is_enemy(layers_1) && is_obstacle(layers_2) {
                Some((entity_2, entity_1))
            } else {
                None
            }
        })
        .for_each(|(obs_en, _)| {
            println!("Running ENEMIES hit OBSTACLE logic");

            if entities_despawned.get(&obs_en).is_none() {
                commands.entity(obs_en).despawn();
                entities_despawned.insert(obs_en);
            }
        });
}

fn manage_enemy_hit_friendly(
    mut commands: Commands, 
    mut events: EventReader<CollisionEvent>,
) {
    let mut entities_despawned: HashSet<Entity> = HashSet::new();
    events
        .iter()
        .filter(|e| e.is_started())
        .filter_map(|event| {
            let (entity_1, entity_2) = event.rigid_body_entities();
            let (layers_1, layers_2) = event.collision_layers();
            if is_friendly(layers_1) && is_enemy(layers_2) {
                Some((entity_1, entity_2))
            } else if is_enemy(layers_1) && is_friendly(layers_2) {
                Some((entity_2, entity_1))
            } else {
                None
            }
        })
        .for_each(|(friendly_en, _)| {
            println!("Running ENEMY hit FRIENDLY logic");

            if entities_despawned.get(&friendly_en).is_none() {
                commands.entity(friendly_en).despawn();
                entities_despawned.insert(friendly_en);
            }
        });
}

pub fn is_player(layers: CollisionLayers) -> bool {
    layers.contains_group(WorldPhysicsLayer::Player) && 
    !layers.contains_group(WorldPhysicsLayer::Enemy)
}

pub fn is_friendly(layers: CollisionLayers) -> bool {
    layers.contains_group(WorldPhysicsLayer::Friendly) && 
    !layers.contains_group(WorldPhysicsLayer::Enemy)
}

pub fn is_enemy(layers: CollisionLayers) -> bool {
    !layers.contains_group(WorldPhysicsLayer::Player) && 
    layers.contains_group(WorldPhysicsLayer::Enemy)
}

pub fn is_hostile_projectile(layers: CollisionLayers) -> bool {
    !layers.contains_group(WorldPhysicsLayer::FriendlyProjectile) && 
    layers.contains_group(WorldPhysicsLayer::HostileProjectile)
}

pub fn is_friendly_projectile(layers: CollisionLayers) -> bool {
    !layers.contains_group(WorldPhysicsLayer::HostileProjectile) && 
    layers.contains_group(WorldPhysicsLayer::FriendlyProjectile)
}

pub fn is_projectile(layers: CollisionLayers) -> bool {
    layers.contains_group(WorldPhysicsLayer::Projectile)
}

pub fn is_obstacle(layers: CollisionLayers) -> bool {
    layers.contains_group(WorldPhysicsLayer::Obstacle)
}