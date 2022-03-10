use std::{collections::HashSet, f32::consts::PI};

use bevy::{prelude::*, sprite::{self, collide_aabb::collide}, core::FixedTimestep};
use rand::prelude::SliceRandom;
use crate::{Game, WinSize, SpriteInfos, shared::{Health, RenderedAssetInfo, WeaponState, Velocity, MovementSpeed, Projectile}, AssetScaling, GAME_TIME_STEP};

use super::Obstacle;

enum AlienType {
    RED,
    GREEN,
    YELLOW,
}

#[derive(Component)]
pub struct FromEnemy;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct EnemyState;

#[derive(Bundle)]
struct AlienBundle {
    #[bundle]
    _sb: SpriteBundle,
    _e: Enemy,
    _h: Health,
    _ws: WeaponState,
    _rai: RenderedAssetInfo,
}
impl AlienBundle {
    fn new(x: f32, y: f32, alien_type: AlienType, sprite_infos: &Res<SpriteInfos>) -> Self {
        let asset = match alien_type {
            AlienType::RED => sprite_infos.red_enemy.clone(),
            AlienType::GREEN => sprite_infos.green_enemy.clone(),
            AlienType::YELLOW => sprite_infos.yellow_enemy.clone(),
        };

        let asset_size = Vec2::new (
            1. * asset.1.x,
            1. * asset.1.y,
        );
        let asset_info = RenderedAssetInfo::new(asset_size);

        Self {
            _sb: SpriteBundle {
                texture: asset.0,
                transform: Transform {
                    translation: Vec3::new(x, y, 5.),
                    ..Default::default()
                },
                ..Default::default()
            },
            _e: Enemy,
            _h: Health::default(),
            _ws: WeaponState::fast_normal_weapon(),
            _rai: asset_info,
        }
    }
}

pub struct AlienState {
    movement_direction: f32,
    movement_speed: MovementSpeed,
    move_down: bool,
    available_to_shoot: u32,
}

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(
                StartupStage::PostStartup,
                setup_enemies
            )
            .add_system_set(
                SystemSet::new()
                    .with_system(manage_alien_movement_direction)
                    .with_system(manage_alien_horizontal_movement)
                    .with_system(manage_alien_vertical_movement)
                    .with_system(alien_hit_obstacle)
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.800))
                    .with_system(alien_random_shoot)
            )
            ;
    }
}

fn setup_enemies(
    mut commands: Commands,
    sprite_infos: Res<SpriteInfos>
) {
    // setup resources
    commands.insert_resource(AlienState {
        movement_direction: -1.,
        movement_speed: MovementSpeed { value: 30. },
        move_down: false,
        available_to_shoot: 2,
    });

    let alien_rows = 6;
    let alien_cols = 8;
    let x_distance: f32 = 60.;
    let y_distance: f32 = 48.;
    let x_offset: f32 = -210.;
    let y_offset: f32 = -50.;

    for (row_idx, row) in (0..alien_rows).rev().enumerate() {
        for (col_idx, _) in (0..alien_cols).enumerate() {
            let x = col_idx as f32 * x_distance + x_offset;
            let y = row_idx as f32 * y_distance + y_offset;

            let alien_type = match row {
                0 => AlienType::YELLOW,
                a if a >= 1 && a <= 2 => AlienType::GREEN,
                _ => AlienType::RED,
            };
        
            commands.spawn_bundle(AlienBundle::new(
                x, y, alien_type, &sprite_infos)
            );
        }
    }
}

pub fn alien_hit_obstacle (
    mut commands: Commands,
    obstacle_q: Query<(Entity, &RenderedAssetInfo, &Transform), With<Obstacle>>,
    enemy_q: Query<(&RenderedAssetInfo, &Transform), With<Enemy>>,
) {
    let mut entities_despawned: HashSet<Entity> = HashSet::new();
    for (ob_en, ob_rai, ob_tf) in obstacle_q.iter() {
        for (en_rai, en_tf) in enemy_q.iter() {
            let collision = collide (
                ob_tf.translation,
                ob_rai.size,
                en_tf.translation,
                en_rai.size
            );

            if let Some(_) = collision {
                if (entities_despawned.get(&ob_en)).is_none() {
                    commands.entity(ob_en).despawn();
                    entities_despawned.insert(ob_en);
                }
            }
        }
    }
}

pub fn alien_random_shoot(
    mut commands: Commands,
    mut q: Query<(&mut WeaponState, &Transform), With<Enemy>>,
    mut alien_state: ResMut<AlienState>,
    time: Res<Time>,
    sprite_infos: Res<SpriteInfos>,
    asset_scaling: Res<AssetScaling>,
) {
    // TODO: how to choose randomly more efficiently?
    let vec_q = q
        .iter()
        .collect::<Vec<_>>();
    let q_rand = vec_q
        .choose_multiple(&mut rand::thread_rng(), alien_state.available_to_shoot.try_into().unwrap());

    for (weapon_state, tf) in q_rand.into_iter() {
        if weapon_state.ready {
            let pos = tf.translation.truncate();
            let asset_size = 
                asset_scaling.enemy_projectile.truncate() * sprite_infos.alien_laser.1;
            let asset_info = RenderedAssetInfo::new(asset_size);
            
            commands
                .spawn()
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(asset_size),
                        ..Default::default()
                    },
                    texture: sprite_infos.alien_laser.0.clone(),
                    transform: Transform {
                        // quaternion rotation using radians
                        rotation: Quat::from_rotation_z(PI),
                        translation: Vec3::new(pos.x, pos.y, 0.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(asset_info)
                .insert(Projectile::default())
                .insert(Velocity::new(0., -weapon_state.projectile_speed))
                .insert(FromEnemy)
                ;
        }
    }
}

fn manage_alien_horizontal_movement(
    mut q: Query<&mut Transform, With<Enemy>>,
    mut alien_state: ResMut<AlienState>
) {
    for mut tf in q.iter_mut() {
        tf.translation.x += 
            alien_state.movement_direction * 
            alien_state.movement_speed.value * 
            GAME_TIME_STEP;
    }
}

fn manage_alien_vertical_movement(
    mut q: Query<&mut Transform, With<Enemy>>,
    mut alien_state: ResMut<AlienState>
) {
    if alien_state.move_down {
        for mut tf in q.iter_mut() {
            tf.translation.y += -10.;
        }
        alien_state.move_down = false;
    }
}

fn manage_alien_movement_direction(
    mut q: Query<(&mut Transform, &RenderedAssetInfo), With<Enemy>>,
    mut alien_state: ResMut<AlienState>,
    win_size: Res<WinSize>,
) {
    for (tf, info) in q.iter_mut() {
        let curr_x = tf.translation.x;
        if curr_x.abs() >= (win_size.w / 2.) - (info.size.x / 2.) {
            alien_state.movement_direction *= -1.;
            alien_state.move_down = true;
            break;
        }
    }
}