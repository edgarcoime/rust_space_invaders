use bevy::prelude::*;
use bevy_rapier2d::{physics::RapierConfiguration, prelude::{RigidBodyVelocity, ColliderPosition, RigidBodyVelocityComponent, ColliderPositionComponent, IntersectionEvent, ContactEvent}};

use crate::{Game, entities::{FromPlayer, Enemy}};

#[derive(Component)]
pub struct Projectile {
    pub velocity: Vec3,
}

#[derive(Component)]
pub struct WeaponState {
    pub ready: bool,
    pub cooldown: f64,
    pub last_fired: f64,
    pub projectile_speed: f32,
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

pub struct WeaponsPlugin;
impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(manage_all_weapons_state)
            .add_system(display_events)
        ;
    }
}

fn manage_all_weapons_state (
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

/* A system that displays the events. */
fn display_events(
    mut intersection_events: EventReader<IntersectionEvent>,
    mut contact_events: EventReader<ContactEvent>,
) {
    for intersection_event in intersection_events.iter() {
        println!("Received intersection event: {:?}", intersection_event);
    }

    for contact_event in contact_events.iter() {
        println!("Received contact event: {:?}", contact_event);
    }
}

fn player_projectiles_hit_enemy(
    mut commands: Commands,
    mut game: ResMut<Game>,
    projectile_q: Query<Entity, (With<FromPlayer>, With<Projectile>)>,
    enemy_q: Query<Entity, With<Enemy>>,
    rapier_config: ResMut<RapierConfiguration>,
) {

}