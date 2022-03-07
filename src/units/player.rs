use bevy::{prelude::*};

use crate::{SpriteInfos, WinSize, GAME_TIME_STEP};

use super::{Health, Velocity, weapons::{WeaponState, Projectile}, MovementSpeed};

// region:      Resources
// endregion:   Resources

// region:      Components
#[derive(Component)]
struct PlayerName(String);

#[derive(Component)]
struct Player;

#[derive(Component)]
pub struct FromPlayer;

#[derive(Bundle)]
struct PlayerBundle {
    _p: Player,
    name: PlayerName,
    health: Health,
    velocity: Velocity,
    movement_speed: MovementSpeed,
    main_weapon_state: WeaponState,
}
impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            _p: Player,
            name: PlayerName("Player 1".to_string()),
            health: Health::default(),
            velocity: Velocity::from(Vec2::new(0., 0.)),
            main_weapon_state: WeaponState::fast_normal_weapon(),
            movement_speed: MovementSpeed(250.),
        }
    }
}

// endregion:   Components

// region:      Entities
// endregion:   Entities

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_stage(
                "game_setup_player"
                , SystemStage::single(player_spawn)
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
    println!("Player spawn");
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
        .insert_bundle(PlayerBundle::default());
}

fn player_movement(
    kb_in: Res<Input<KeyCode>>,
    win_size: Res<WinSize>,
    sprite_infos: Res<SpriteInfos>,
    mut q: Query<(&Velocity, &MovementSpeed, &mut Transform), With<Player>>,
) {
    if let Ok((vel, mov_spd, mut tf)) = q.get_single_mut() {
        // TODO: QUERY WILL TRY TO MATCH ALL OF DESIRED
        // SO WILL NOT WORK IF YOUR DESIRED DOES NOT IMPLEMENT BOTH COMPONENTS
        let player_dimensions = sprite_infos.player.1;
        let player_sprite_x = player_dimensions.x;
        let target_bounds_x = win_size.w/2. - player_sprite_x/2.;
        if kb_in.pressed(KeyCode::Left) || kb_in.pressed(KeyCode::A) {
            let desired_x = tf.translation.x + (-1. * mov_spd.0 * GAME_TIME_STEP);
            if desired_x > -target_bounds_x {
                tf.translation.x = desired_x
            }
        } else if kb_in.pressed(KeyCode::Right) || kb_in.pressed(KeyCode::D) {
            let desired_x = tf.translation.x + (1. * mov_spd.0 * GAME_TIME_STEP);
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
) {
    if let Ok((player_tf, mut weapon_state)) = q.get_single_mut() {
        if weapon_state.ready && (kb.pressed(KeyCode::Space) || kb.pressed(KeyCode::Z)) {
            let x = player_tf.translation.x;
            let y = player_tf.translation.y;

            let mut spawn_lasers = |x_offset: f32| {
                commands
                    .spawn_bundle(SpriteBundle {
                        texture: sprite_infos.player_laser.0.clone(),
                        transform: Transform {
                            translation: Vec3::new(x + x_offset, y, 0.),
                            scale: Vec3::new(0.5, 0.5, 1.),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Projectile)
                    .insert(FromPlayer)
                    // Set new Velocity based on weapon state
                    // TODO: How to create different weapons?
                    .insert(Velocity::from(Vec2::new(0., weapon_state.projectile_speed)))
                    ;
            };

            spawn_lasers(0.);

            // Set weapon state
            weapon_state.fired(time.seconds_since_startup());
        }
    }

}