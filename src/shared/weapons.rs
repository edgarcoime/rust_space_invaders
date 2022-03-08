use bevy::prelude::*;
use bevy_rapier2d::{physics::{RapierConfiguration, IntoEntity}, prelude::{RigidBodyVelocity, ColliderPosition, RigidBodyVelocityComponent, ColliderPositionComponent, IntersectionEvent, ContactEvent, ColliderHandle}};

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
            .add_system(player_projectiles_hit_enemy)
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
// https://rapier.rs/docs/user_guides/bevy_plugin/advanced_collision_detection/
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
    mut intersection_events: EventReader<IntersectionEvent>,
    q_enemy: Query<Entity, With<Enemy>>,
    q_player_proj: Query<Entity, (With<Projectile>, With<FromPlayer>)>,
) {
    for intersection_event in intersection_events.iter() {
        // Find out how to specify only for specific colission groups
        let entity1 = intersection_event.collider1.entity();
        let entity2 = intersection_event.collider2.entity();

        let mut en1_valid = false;
        let mut en2_valid = false;

        for e in q_enemy.iter() {
            if e == entity1 || e == entity2 {
                en1_valid = true;
            }
        }

        for proj in q_player_proj.iter() {
            if proj == entity1 || proj == entity2 {
                en2_valid = true;
            }
        }

        if en1_valid && en2_valid {
            commands.entity(entity1).despawn();
            commands.entity(entity2).despawn();
            game.active_enemies -= 1;
        }
    }
}