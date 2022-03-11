use bevy::prelude::*;
use heron::prelude::*;

use crate::{WinSize, SpriteInfos, shared::{MovementSpeed, WeaponState, Projectile, WorldPhysicsLayer}, utils::RenderedAssetInfo, GAME_TIME_STEP, AssetScaling};
use super::{BasicShipBundle, EntityPhysicsBundle};

// region:      Components
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct FromPlayer;

#[derive(Component)]
pub struct PlayerState {
    name: String
}

#[derive(Bundle)]
struct PlayerBundle {
    #[bundle]
    _sb: SpriteBundle,
    #[bundle]
    _ship_bundle: BasicShipBundle,
    #[bundle]
    _phys: EntityPhysicsBundle,
    _p: Player,
    _ps: PlayerState,
    _rai: RenderedAssetInfo,
}
impl PlayerBundle {
    fn new(x: f32, y: f32, sprite_infos: &Res<SpriteInfos>) -> Self {
        let asset = sprite_infos.player.clone();
        let asset_size = Vec2::new (
            1. * asset.1.x,
            1. * asset.1.y,
        );
        let asset_info = RenderedAssetInfo::new(asset_size);

        Self {
            _sb: SpriteBundle {
                texture: asset.0,
                transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                ..Default::default()
            },
            _p: Player,
            _ps: PlayerState { name: "Player 1".to_string() },
            _ship_bundle: BasicShipBundle::default(),
            _rai: asset_info,
            _phys: EntityPhysicsBundle {
                _rb: RigidBody::KinematicPositionBased,
                _cs: CollisionShape::Cuboid {
                    half_extends: asset_size.extend(0.) / 2.,
                    border_radius: None,
                },
                _cl: CollisionLayers::none()
                    .with_group(WorldPhysicsLayer::Player)
                    .with_group(WorldPhysicsLayer::Friendly)
                    .with_mask(WorldPhysicsLayer::Enemy)
                    .with_mask(WorldPhysicsLayer::HostileProjectile)
            }
        }
    }
}
// endregion:   Components

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(
                StartupStage::PostStartup, 
                setup_player
            )
            .add_system(player_movement)
            .add_system(player_shooting)
        ;
    }
}

fn setup_player(
    mut commands: Commands,
    win_size: Res<WinSize>,
    sprite_infos: Res<SpriteInfos>,
) {
    let bottom = -win_size.h / 2.;
    commands
        .spawn()
        .insert_bundle(PlayerBundle::new(0., bottom + 75. / 3., &sprite_infos))
    ;
}

fn player_movement(
    mut q: Query<(&MovementSpeed, &mut Transform), With<Player>>,
    kb_in: Res<Input<KeyCode>>,
    win_size: Res<WinSize>,
    sprite_infos: Res<SpriteInfos>,
) {
    if let Ok((mov_spd, mut tf)) = q.get_single_mut() {
        // TODO: QUERY WILL TRY TO MATCH ALL OF DESIRED
        // SO WILL NOT WORK IF YOUR DESIRED DOES NOT IMPLEMENT BOTH COMPONENTS
        let player_dimensions = sprite_infos.player.1;
        let player_sprite_x = player_dimensions.x;
        let target_bounds_x = win_size.w/2. - player_sprite_x/2.;
        if kb_in.pressed(KeyCode::Left) || kb_in.pressed(KeyCode::A) {
            let desired_x = tf.translation.x + (-1. * mov_spd.value * GAME_TIME_STEP);
            if desired_x > -target_bounds_x {
                tf.translation.x = desired_x
            }
        } else if kb_in.pressed(KeyCode::Right) || kb_in.pressed(KeyCode::D) {
            let desired_x = tf.translation.x + (1. * mov_spd.value * GAME_TIME_STEP);
            if desired_x < target_bounds_x {
                tf.translation.x = desired_x
            }
        };
    }
}

fn player_shooting(
    mut commands: Commands,
    mut q: Query<(&Transform, &mut WeaponState), With<Player>>,
    time: Res<Time>,
    kb: Res<Input<KeyCode>>,
    sprite_infos: Res<SpriteInfos>,
    asset_scaling: Res<AssetScaling>,
) {
    if let Ok((player_tf, mut weapon_state)) = q.get_single_mut() {
        if weapon_state.ready && (kb.pressed(KeyCode::Space) || kb.pressed(KeyCode::Z)) {
            let asset = sprite_infos.player_laser.clone();
            let pos = player_tf.translation;
            let asset_size = 
                asset_scaling.enemy_projectile.truncate() * sprite_infos.player_laser.1;
            let asset_info = RenderedAssetInfo::new(asset_size);

            commands
                .spawn()
                .insert_bundle(SpriteBundle {
                    texture: asset.0,
                    sprite: Sprite { custom_size: Some(asset_size), ..Default::default() },
                    transform: Transform::from_translation(pos),
                    ..Default::default()
                })
                .insert(asset_info)
                .insert(Projectile::default())
                .insert(FromPlayer)
                // physics
                .insert(RigidBody::KinematicVelocityBased)
                .insert(CollisionShape::Capsule {
                    half_segment: asset_size.y / 2.,
                    radius: asset_size.x / 2.,
                })
                .insert(Velocity::from_linear(Vec3::new(0., weapon_state.projectile_speed, 0.)))
                // .insert(CollisionShape::Cuboid {
                //     half_extends: asset_size.extend(0.) / 2.,
                //     border_radius: None,
                // })
                .insert(
                    CollisionLayers::none()
                        .with_group(WorldPhysicsLayer::Projectile)
                        .with_group(WorldPhysicsLayer::FriendlyProjectile)
                        .with_mask(WorldPhysicsLayer::Obstacle)
                        .with_mask(WorldPhysicsLayer::Enemy)
                )
            ;

            // Set weapon state
            weapon_state.fired(time.seconds_since_startup());
        }
    }
}