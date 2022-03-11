use bevy::prelude::*;
use heron::prelude::*;

use crate::{shared::{Health, WeaponState, MovementSpeed}, utils::RenderedAssetInfo, SpriteInfos, SpriteInfo};

use super::{EntityPhysicsBundle, BasicShipBundle};

// region:      Resources
enum AlienType {
    RED,
    GREEN,
    YELLOW,
}

pub struct AlienFormationState {
    movement_direction: f32,
    movement_speed: MovementSpeed,
    move_down: bool,
    available_to_shoot: u32
}
impl Default for AlienFormationState {
    fn default() -> Self {
        Self {
            movement_direction: -1.,
            movement_speed: MovementSpeed { value: 30. },
            move_down: false,
            available_to_shoot: 2,
        }
    }
}
// endregion:   Resources

// region:      Components
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
    #[bundle]
    _ship_bundle: BasicShipBundle,
    #[bundle]
    _phys: EntityPhysicsBundle,
    _e: Enemy,
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
            _ship_bundle: BasicShipBundle::default(),
            _e: Enemy,
            _rai: asset_info,
            _phys: EntityPhysicsBundle {
                _rb: RigidBody::KinematicPositionBased,
                _cs: CollisionShape::Cuboid {
                    half_extends: asset_size.extend(0.) / 2.,
                    border_radius: None,
                },
                _cl: CollisionLayers::none()
            }
        }
    }
}
// endregion:   Components

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(
                StartupStage::PostStartup,
                setup_enemies,
            )
        ;
    }
}

fn setup_enemies(mut commands: Commands, sprite_infos: Res<SpriteInfos>) {
    commands.insert_resource(AlienFormationState::default());

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
        
            commands.spawn_bundle(AlienBundle::new(x, y, alien_type, &sprite_infos));
        }
    }
}