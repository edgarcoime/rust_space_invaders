use bevy::{prelude::*, sprite};
use rand::{thread_rng, Rng};
use crate::{Game, WinSize, SpriteInfos, shared::{Health, RenderedAssetInfo, WeaponState}, AssetScaling};

enum AlienType {
    RED,
    GREEN,
    YELLOW,
}

#[derive(Component)]
pub struct FromEnemy;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct EnemyState;

#[derive(Bundle)]
struct AlienBundle {
    #[bundle]
    _sb: SpriteBundle,
    _e: Enemy,
    _h: Health,
    _ws: WeaponState,
    _if: RenderedAssetInfo,
}
impl AlienBundle {
    fn new(x: f32, y: f32, alien_type: AlienType, sprite_infos: &Res<SpriteInfos>) -> Self {
        let asset = match alien_type {
            AlienType::RED => sprite_infos.red_enemy.clone(),
            AlienType::GREEN => sprite_infos.green_enemy.clone(),
            AlienType::YELLOW => sprite_infos.yellow_enemy.clone(),
        };

        let asset_size = Vec2::new (
            1. * asset.1.x,
            1. * asset.1.y,
        );
        let asset_info = RenderedAssetInfo::new(asset_size);

        Self {
            _sb: SpriteBundle {
                texture: asset.0,
                transform: Transform {
                    translation: Vec3::new(x, y, 5.),
                    ..Default::default()
                },
                ..Default::default()
            },
            _e: Enemy,
            _h: Health::default(),
            _ws: WeaponState::fast_normal_weapon(),
            _if: asset_info,
        }
    }
}


pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(setup_enemies) // Change to spawn per level?
            // .add_system(enemy_spawn) // Change to spawn per level?
            // .add_system(log_enemy_rigid_and_collider)
        ;
    }
}

fn setup_enemies(
    mut commands: Commands,
    mut game: ResMut<Game>,
    win_size: Res<WinSize>,
    sprite_infos: Res<SpriteInfos>
) {
    if game.active_enemies < 2 {
        let mut rng = thread_rng();
        let w_span = win_size.w / 2. - 100.;
        let h_span = win_size.h / 2. - 100.;
        let x = rng.gen_range(-w_span..w_span) as f32;
        let y = rng.gen_range(-h_span..h_span) as f32;

        commands.spawn_bundle(AlienBundle::new(
            x, 
            y, 
            AlienType::YELLOW, 
            &sprite_infos)
        );

        game.active_enemies += 1;
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
        let asset_size = Vec2::new (
                1. * asset.1.x,
                1. * asset.1.y,
        );
        let asset_info = RenderedAssetInfo::new(asset_size);

        // spawn enemy
        commands
            .spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(asset_size),
                    ..Default::default()
                },
                texture: asset.0,
                transform: Transform {
                    translation: next_location,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(asset_info)
            .insert(Health::default())
            // .insert(Health::from(2))
            .insert(Enemy);

        game.active_enemies += 1;
    }
}