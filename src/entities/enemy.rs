use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{thread_rng, Rng};

use crate::{Game, WinSize, SpriteInfos};

#[derive(Component)]
pub struct FromEnemy;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct EnemyState;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(enemy_spawn) // Change to spawn per level?
            // .add_system(log_enemy_rigid_and_collider)
        ;
    }
}

fn enemy_spawn(
    mut commands: Commands,
    mut game: ResMut<Game>,
    win_size: Res<WinSize>,
    sprite_infos: Res<SpriteInfos>,
    rapier_config: ResMut<RapierConfiguration>,
) {
    let scl = rapier_config.scale;
    if game.active_enemies < 2 {
        // TODO: get random position?
        // compute the rando position
        let mut rng = thread_rng();
        let w_span = win_size.w / 2. - 100.;
        let h_span = win_size.h / 2. - 100.;
        let x = rng.gen_range(-w_span..w_span) as f32;
        let y = rng.gen_range(-h_span..h_span) as f32;
        let proposed_location = Vec2::new(x/scl, y/scl);

        // spawn enemy
        commands
            .spawn()
            .insert_bundle(SpriteBundle {
                texture: sprite_infos.red_enemy.0.clone(),
                ..Default::default()
            })
            .insert_bundle(RigidBodyBundle {
                position: RigidBodyPosition {
                    position: proposed_location.into(),
                    ..Default::default()
                }.into(),
                // velocity: RigidBodyVelocity {
                //     linvel: Vec2::new(1., 1.).into(),
                //     angvel: 0.
                // }.into(),
                ..Default::default()
            })
            .insert_bundle(ColliderBundle {
                collider_type: ColliderType::Sensor.into(),
                // flags: (ActiveEvents::CONTACT_EVENTS | ActiveEvents::INTERSECTION_EVENTS).into(),
                flags: ColliderFlags {
                    active_events: (ActiveEvents::CONTACT_EVENTS | ActiveEvents::INTERSECTION_EVENTS).into(),
                    ..Default::default()
                }.into(),
                // position: proposed_location.into(),
                shape: ColliderShape::cuboid(
                    (sprite_infos.red_enemy.1.x / 2.) / scl,
                    (sprite_infos.red_enemy.1.y / 2.) / scl,
                ).into(),
                ..Default::default()
            })
            .insert(RigidBodyPositionSync::Discrete)
            .insert(Enemy);

        game.active_enemies += 1;
    }
}

fn log_enemy_rigid_and_collider(
    q: Query<(&RigidBodyVelocityComponent, &RigidBodyPositionComponent, &ColliderPositionComponent), With<Enemy>>,
) {
    for (rigid_vel, rigid_pos, col_pos) in q.iter() {
        println!("Rigid Vector: {:?}", rigid_vel.as_vector());
        println!("Rigid Position: {:?}", rigid_pos.position);
        println!("Collider pos: {:?}", col_pos.0);
    }
}