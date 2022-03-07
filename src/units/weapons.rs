use bevy::prelude::*;

use crate::{WinSize, GAME_TIME_STEP};

use super::{Velocity, player::FromPlayer};

#[derive(Component)]
pub struct WeaponState {
    pub ready: bool,
    pub cooldown: f32,
    pub last_fired: f64,
    pub velocity: Velocity
}
impl Default for WeaponState {
    fn default() -> Self {
        Self {
            ready: true,
            cooldown: 100.,
            last_fired: 0.,
            velocity: Velocity::normal_projectile()
        }
    }
}

impl WeaponState {
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
pub struct Projectile;

pub struct WeaponsPlugin;
impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(manage_all_weapons_state)
            .add_system(move_player_projectiles)

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

        if w_state.last_fired == 0. || now > last_shot {
            w_state.reset();
        }
    }
}

fn move_player_projectiles (
    mut commands: Commands,
    mut q: Query<(Entity, &Velocity, &mut Transform), (With<Projectile>, With<FromPlayer>)>,
    win_size: Res<WinSize>,
) {
    for (proj_entity, vel, mut proj_tf) in q.iter_mut() {
        let translation = &mut proj_tf.translation;
        // // TODO calculate vector for diagonal projectiles 
        translation.y += vel.0 * GAME_TIME_STEP;

        // Despawn laser if it goes beyond bounds
        if translation.y > win_size.h {
            commands.entity(proj_entity).despawn();
        }
    }
}