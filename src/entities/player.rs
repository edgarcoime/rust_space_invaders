use bevy::prelude::*;
use heron::prelude::*;

use crate::{WinSize, SpriteInfos, shared::{WeaponState, MovementSpeed}, utils::RenderedAssetInfo, GAME_TIME_STEP};

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
    _p: Player,
    _ps: PlayerState,
    _ws: WeaponState,
    _ms: MovementSpeed,
    _rai: RenderedAssetInfo,
    _rb: RigidBody,
    _cs: CollisionShape,
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
            _ws: WeaponState::fast_normal_weapon(),
            _ms: MovementSpeed::default(),
            _rai: asset_info,
            _rb: RigidBody::KinematicPositionBased,
            _cs: CollisionShape::Cuboid {
                half_extends: asset_size.extend(0.) / 2.,
                border_radius: None,
            },
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
        ;
    }
}

fn setup_player(
    mut commands: Commands,
    win_size: Res<WinSize>,
    sprite_infos: Res<SpriteInfos>,
) {
    let bottom = -win_size.h / 2.;
    println!("{}, {}", bottom, win_size.h);
    commands
        .spawn()
        .insert_bundle(PlayerBundle::new(0., bottom + 75. / 3., &sprite_infos))
    ;
}

fn player_movement(
    mut q: Query<(&MovementSpeed, &mut Transform), With<Player>>,
    time: Res<Time>,
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

