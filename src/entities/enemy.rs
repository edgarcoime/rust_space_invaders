use bevy::prelude::*;
use heron::prelude::*;

use crate::{shared::{Health, WeaponState, MovementSpeed}, utils::RenderedAssetInfo, SpriteInfos, SpriteInfo, GAME_TIME_STEP, WinSize};

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
            .add_system_set(
                SystemSet::new()
                    .with_system(manage_alien_movement_direction)
                    .with_system(manage_alien_horizontal_movement)
                    .with_system(manage_alien_vertical_movement)
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

// region:      Enemy Formation movement
fn manage_alien_horizontal_movement(
    mut q: Query<&mut Transform, With<Enemy>>,
    alien_state: ResMut<AlienFormationState>
) {
    for mut tf in q.iter_mut() {
        tf.translation.x += 
            alien_state.movement_direction * 
            alien_state.movement_speed.value * 
            GAME_TIME_STEP;
    }
}

fn manage_alien_vertical_movement(
    mut commands: Commands,
    mut q: Query<(Entity, &mut Transform), With<Enemy>>,
    mut alien_state: ResMut<AlienFormationState>,
    win_size: Res<WinSize>,
) {
    // let mut entities_despawned: HashSet<Entity> = HashSet::new();
    if alien_state.move_down {
        for (en, mut tf) in q.iter_mut() {
            tf.translation.y += -10.;

            if tf.translation.y.abs() > win_size.h / 2. {
                commands.entity(en).despawn();
            }
        }
        alien_state.move_down = false;
    }
}

fn manage_alien_movement_direction(
    mut q: Query<(&mut Transform, &RenderedAssetInfo), With<Enemy>>,
    mut alien_state: ResMut<AlienFormationState>,
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
// endregion:   Enemy Formation movement