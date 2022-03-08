use bevy::{prelude::*, math::Vec3Swizzles, sprite::collide_aabb::collide};

use crate::{WinSize, GAME_TIME_STEP, SpriteInfos, Game};

use super::{Velocity, player::FromPlayer, enemy::Enemy};

const DEFAULT_WEAPON_COOLDOWN: f64 = 0.3;

#[derive(Component)]
pub struct WeaponState {
    pub ready: bool,
    pub cooldown: f64,
    pub last_fired: f64,
    pub projectile_speed: f32,
}
impl Default for WeaponState {
    fn default() -> Self {
        Self {
            ready: true,
            cooldown: DEFAULT_WEAPON_COOLDOWN,
            last_fired: 0.,
            projectile_speed: 600.,
        }
    }
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

#[derive(Component)]
pub struct Projectile {
    pub velocity: Vec3
}

pub struct WeaponsPlugin;
impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(manage_all_weapons_state)
            .add_system(move_player_projectiles)
            .add_system(player_projectiles_hits_enemies)
        ;
    }
}

pub fn manage_all_weapons_state (
    mut commands: Commands,
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

fn move_player_projectiles (
    mut commands: Commands,
    mut q: Query<(Entity, &Projectile, &mut Transform), With<FromPlayer>>,
    win_size: Res<WinSize>,
) {
    for (proj_entity, proj, mut proj_tf) in q.iter_mut() {
        let translation = &mut proj_tf.translation;
        // // TODO calculate vector for diagonal projectiles 
        translation.y += proj.velocity.y * GAME_TIME_STEP;
        translation.x += proj.velocity.x * GAME_TIME_STEP;

        // Despawn laser if it goes beyond bounds
        if translation.y > win_size.h {
            commands.entity(proj_entity).despawn();
        }
    }
}

fn player_projectiles_hits_enemies(
    mut commands: Commands,
    mut game: ResMut<Game>,
    projectile_q: Query<(Entity, &Transform), With<FromPlayer>>, // projectiles
    enemy_q: Query<(Entity, &Transform), With<Enemy>>,
    sprite_infos: Res<SpriteInfos>,
) {
    for (projectile_en, projectile_tf) in projectile_q.iter() {
        let projectile_scale = projectile_tf.scale.truncate();
        let projectile_size = sprite_infos.player_laser.1;

        for (enemy_en, enemy_tf) in enemy_q.iter() {
            let enemy_scale = enemy_tf.scale.truncate();
            let enemy_size = sprite_infos.red_enemy.1;

            let collision = collide(
                projectile_tf.translation,
                projectile_size * projectile_scale,
                enemy_tf.translation,
                enemy_size * enemy_scale
            );

            if let Some(_) = collision {
                println!("Collision!");

                println!("{}, {}", projectile_size, enemy_size);
                println!("{}, {}", sprite_infos.red_enemy.1.x, sprite_infos.red_enemy.1.y);
                // remove enemy
                commands.entity(enemy_en).despawn();
                game.active_enemies -= 1;

                commands.entity(projectile_en).despawn();
            }

            // if let Some(_) = collision {
            //     // remove enemy
            //     commands.entity(enemy_en).despawn();
            //     game.active_enemies -= 1;

            //     commands.entity(projectile_en).despawn();
            // }
        }
    }
}

// region:      SpriteGenerators
// endregion:   SpriteGenerators