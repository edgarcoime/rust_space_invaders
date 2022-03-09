use bevy::{prelude::*, sprite};
use rand::{thread_rng, Rng};
use crate::{Game, WinSize, SpriteInfos, shared::{Health, RealAssetSize}, AssetScaling};

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
        let asset = sprite_infos.red_enemy.clone();

        // TODO: get random position?
        // compute the rando position
        let mut rng = thread_rng();
        let w_span = win_size.w / 2. - 100.;
        let h_span = win_size.h / 2. - 100.;
        let x = rng.gen_range(-w_span..w_span) as f32;
        let y = rng.gen_range(-h_span..h_span) as f32;
        let next_location = Vec3::new(x, y, 5.);
        let asset_size = RealAssetSize::new (
                1. * asset.1.x,
                1. * asset.1.y,
        );

        // spawn enemy
        commands
            .spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(asset_size.width, asset_size.height)),
                    ..Default::default()
                },
                texture: asset.0,
                transform: Transform {
                    translation: next_location,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(asset_size)
            .insert(Health::default())
            // .insert(Health::from(2))
            .insert(Enemy);

        game.active_enemies += 1;
    }
}