use std::collections::HashSet;

use bevy::{prelude::*, sprite::collide_aabb::{Collision, collide}, reflect::List};
use crate::{Game, entities::{FromPlayer, Enemy, Obstacle}, SpriteInfos, AssetScaling};
use super::{Health, RenderedAssetInfo};

#[derive(Component)]
pub struct Projectile {
    damage: u32
}
impl Default for Projectile {
    fn default() -> Self {
        Self { damage: 1 }
    }
}

#[derive(Component)]
pub struct WeaponState {
    pub ready: bool,
    pub cooldown: f64,
    pub last_fired: f64,
    pub projectile_speed: f32,
}
impl WeaponState {
    pub fn fast_normal_weapon() -> Self {
        Self {
            ready: true,
            cooldown: 0.2,
            last_fired: 0.,
            projectile_speed: 700.,
        }
    }

    pub fn fired(&mut self, time: f64) {
        self.ready = false;
        self.last_fired = time;
    }

    pub fn reset(&mut self) {
        self.ready = true;
        self.last_fired = 0.;
    }
}

pub struct WeaponsPlugin;
impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(manage_all_weapons_state)
            .add_system(manage_player_projectiles_hit)
            .add_system(manage_projectiles_hit_obstacles)
            // .add_system(display_events)
        ;
    }
}

fn manage_all_weapons_state (
    mut q: Query<&mut WeaponState>,
    time: Res<Time>,
) {
    // might have to check if it is online?
    for mut w_state in q.iter_mut() {
        let now = time.seconds_since_startup();
        let last_shot = w_state.last_fired;

        if w_state.last_fired == 0. || now > last_shot + w_state.cooldown {
            w_state.reset();
        }
    }
}

fn manage_player_projectiles_hit (
    mut commands: Commands,
    mut game: ResMut<Game>,
    projectile_q: Query<
        (Entity, &Projectile, &RenderedAssetInfo, &Transform), 
        (With<FromPlayer>, With<Projectile>)
    >, // projectiles
    mut enemy_q: Query<(Entity, &mut Health, &RenderedAssetInfo, &Transform), With<Enemy>>,
) {
    for (proj_en, proj, proj_asset_info, proj_tf) in projectile_q.iter() {
        for (ene_en, mut ene_health, enemy_asset_info, ene_tf) in enemy_q.iter_mut() {

            let collision = collide(
                proj_tf.translation,
                proj_asset_info.size,
                ene_tf.translation,
                enemy_asset_info.size,
            );

            if let Some(_) = collision {
                ene_health.current_hp -= proj.damage;
                println!("{}", ene_health.current_hp);
                commands.entity(proj_en).despawn();

                if ene_health.current_hp <= 0 {
                    commands.entity(ene_en).despawn();
                    game.active_enemies -= 1;
                }
            }
        }
    }
}

fn manage_projectiles_hit_obstacles (
    mut commands: Commands,
    mut obstacles_q: Query<
        (Entity, &mut Health, &Sprite, &Transform),
        With<Obstacle>,
    >,
    projectiles_q: Query<
        (Entity, &Projectile, &RenderedAssetInfo, &Transform),
        With<Projectile>,
    >,
) {
    // TODO: Prevent multiple despawn calls
    // When a laser hits two obstacles at the same time
    let mut entities_despawned: HashSet<Entity> = HashSet::new();
    for (ob_en, mut ob_health, ob_sprite, ob_tf) in obstacles_q.iter_mut() {
        for (proj_en, proj, proj_info, proj_tf) in projectiles_q.iter() {
            if let Some(ob_size) = ob_sprite.custom_size {
                let collision = collide(
                    proj_tf.translation,
                    proj_info.size,
                    ob_tf.translation,
                    ob_size
                );

                if let Some(_) = collision {
                    ob_health.current_hp -= proj.damage;

                    // Despawn and ensure entity is not despawned twice
                    if (entities_despawned.get(&proj_en)).is_none() {
                        commands.entity(proj_en).despawn();
                        entities_despawned.insert(proj_en);
                    }

                    if  ob_health.current_hp <= 0 && 
                        entities_despawned.get(&ob_en).is_none() {
                            commands.entity(ob_en).despawn();
                            entities_despawned.insert(ob_en);
                    }
                }
            }
        }
    }
}