use bevy::prelude::*;
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
) {
    if game.active_enemies < 2 {
        // TODO: get random position?
        // compute the rando position
        let mut rng = thread_rng();
        let w_span = win_size.w / 2. - 100.;
        let h_span = win_size.h / 2. - 100.;
        let x = rng.gen_range(-w_span..w_span) as f32;
        let y = rng.gen_range(-h_span..h_span) as f32;
        let next_location = Vec2::new(x, y);

        // spawn enemy
        commands
            .spawn()
            .insert_bundle(SpriteBundle {
                texture: sprite_infos.red_enemy.0.clone(),
                ..Default::default()
            })
            .insert(Enemy);

        game.active_enemies += 1;
    }
}