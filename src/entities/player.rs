use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{WinSize, SpriteInfos, shared::{Health, WeaponState, MovementSpeed, Projectile, Velocity, RenderedAssetInfo}, GAME_TIME_STEP, AssetScaling};

use super::Enemy;

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
            .add_system(player_hit_enemy)
        ;
    }
}

fn player_spawn (
    mut commands: Commands,
    win_size: Res<WinSize>,
    sprite_infos: Res<SpriteInfos>,
) {
    let asset = sprite_infos.player.clone();
    let asset_size = Vec2::new(
        1. * asset.1.x,
        1. * asset.1.y,
    );
    let asset_info = RenderedAssetInfo::new(asset_size);

    let bottom = -win_size.h / 2.;
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset.0,
            transform: Transform {
                translation: Vec3::new(0., bottom + 75. / 3. + 5., 10.),
                scale: Vec3::new(1., 1., 10.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(asset_info)
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

fn player_hit_enemy(
    mut commands: Commands,
    player_q: Query<(Entity, &Transform, &RenderedAssetInfo), With<Player>>,
    enemy_q: Query<(Entity, &Transform, &RenderedAssetInfo), With<Enemy>>,
) {
    if let Ok((p_en, p_tf, p_rai)) = player_q.get_single() {
        for (en_en, en_tf, en_rai) in enemy_q.iter() {
            let collision = collide (
                p_tf.translation,
                p_rai.size,
                en_tf.translation,
                en_rai.size
            );

            if let Some(_) = collision {
                // TODO: ensure despawned already
                commands.entity(p_en).despawn();
            }
        }
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
            let pos = player_tf.translation;
            let asset_size = 
                asset_scaling.enemy_projectile.truncate() * sprite_infos.player_laser.1;
            let asset_info = RenderedAssetInfo::new(asset_size);

            commands
                .spawn()
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(asset_size),
                        ..Default::default()
                    },
                    texture: sprite_infos.player_laser.0.clone(),
                    transform: Transform {
                        translation: Vec3::new(pos.x, pos.y, 0.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(asset_info)
                .insert(Projectile::default())
                .insert(Velocity::new(0., weapon_state.projectile_speed))
                // .insert(Velocity::new(0., 150.))
                .insert(FromPlayer)
            ;

            // Set weapon state
            weapon_state.fired(time.seconds_since_startup());
        }
    }
}