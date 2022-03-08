use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{WinSize, SpriteInfos, shared::{Health, WeaponState, MovementSpeed, Projectile}, GAME_TIME_STEP};

#[derive(Component)]
pub struct FromPlayer;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerState {
    name: String
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_stage(
                "player_setup", 
                SystemStage::single(player_spawn.after("main_setup"))
            )
            .add_system(player_movement)
            .add_system(player_shooting)
        ;
    }
}

fn player_spawn (
    mut commands: Commands,
    win_size: Res<WinSize>,
    sprite_infos: Res<SpriteInfos>,
) {
    let bottom = -win_size.h / 2.;
    commands
        .spawn_bundle(SpriteBundle {
            texture: sprite_infos.player.0.clone(),
            transform: Transform {
                translation: Vec3::new(0., bottom + 75. / 3. + 5., 10.),
                scale: Vec3::new(1., 1., 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(Health::default())
        .insert(MovementSpeed { value: 250. })
        .insert(PlayerState { name: "Player 1".to_string() })
        .insert(WeaponState::fast_normal_weapon())
    ;
}

fn player_movement(
    kb_in: Res<Input<KeyCode>>,
    win_size: Res<WinSize>,
    sprite_infos: Res<SpriteInfos>,
    mut q: Query<(&MovementSpeed, &mut Transform), With<Player>>,
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
    rapier_config: ResMut<RapierConfiguration>,
) {
    let scl = rapier_config.scale;
    if let Ok((player_tf, mut weapon_state)) = q.get_single_mut() {
        if weapon_state.ready && (kb.pressed(KeyCode::Space) || kb.pressed(KeyCode::Z)) {
            let x = player_tf.translation.x;
            let y = player_tf.translation.y;
            let proposed_location = Vec2::new(x/scl, y/scl);

            let rigid_body = RigidBodyBundle {
                position: RigidBodyPosition {
                    position: proposed_location.into(),
                    ..Default::default()
                }.into(),
                velocity: RigidBodyVelocity { 
                    linvel: Vec2::new(0., weapon_state.projectile_speed/scl).into(), 
                    angvel: 0.0 
                }.into(),
                ..Default::default()
            };
            let collider = ColliderBundle {
                collider_type: ColliderType::Solid.into(),
                shape: ColliderShape::cuboid(
                    (sprite_infos.player_laser.1.x / 2.) / scl,
                    (sprite_infos.player_laser.1.y / 2.) / scl
                ).into(),
                ..Default::default()
            };
            commands
                .spawn()
                .insert_bundle(SpriteBundle {
                    texture: sprite_infos.player_laser.0.clone(),
                    transform: Transform {
                        scale: Vec3::new(0.5, 0.5, 1.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert_bundle(rigid_body)
                .insert_bundle(collider)
                .insert(RigidBodyPositionSync::Discrete)
                .insert(Projectile { velocity: Vec3::default() })
                .insert(FromPlayer)
            ;

            // Set weapon state
            weapon_state.fired(time.seconds_since_startup());
        }
    }
}