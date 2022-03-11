// region:      Modules
mod system;
mod utils;
mod diagnostics;
mod entities;
mod shared;
// endregion:   Modules

// region:      Namespaces
use bevy::{prelude::*};
use heron::prelude::*;
use diagnostics::DiagnosticsPluginGroup;
use entities::EntitiesPluginGroup;
use shared::SharedPluginGroup;
use utils::load_image;
// endregion:   Namespaces

// region:      Constants
const WINDOW_WIDTH: f32 = 600.0;
const WINDOW_HEIGHT: f32 = 600.0;
const GAME_TIME_STEP: f32 = 1. / 60.;
// endregion:   Constants

// region:      Assets
const SPRITE_DIR: &str = "assets/graphics";
const AUDIO_DIR: &str = "assets/audio";
const FONT_DIR: &str = "assets/font";

const PLAYER_SPRITE: &str = "player.png";
const RED_ENEMY_SPRITE: &str = "red.png";
const GREEN_ENEMY_SPRITE: &str = "green.png";
const YELLOW_ENEMY_SPRITE: &str = "yellow.png";
const TOP_EXTRA_ENEMY_SPRITE: &str = "extra.png";

const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";
const ENEMY_LASER_SPRITE: &str = "laser_b_01.png";
// endregion:   Assets

// region:      States
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    MainMenu,
    InGame,
    Paused,
    GameOver,
}
// endregion:   States

// region:      Resources
#[derive(Default)]
pub struct Game {
    active_enemies: i32,
}

#[derive(Clone)]
pub struct SpriteInfo(Handle<Image>, Vec2);

pub struct SpriteInfos {
    player: SpriteInfo,
    player_laser: SpriteInfo,
    red_enemy: SpriteInfo,
    green_enemy: SpriteInfo,
    yellow_enemy: SpriteInfo,
    alien_laser: SpriteInfo,
    top_alien: SpriteInfo,
}

pub struct AssetScaling {
    player_projectile: Vec3,
    enemy_projectile: Vec3,
}
impl Default for AssetScaling {
    fn default() -> Self {
        Self {
            player_projectile: Vec3::new(0.5, 0.5, 1.),
            enemy_projectile: Vec3::new(0.5, 0.5, 1.),
        }
    }
}

pub struct WinSize {
    w: f32,
    h: f32,
}
impl Default for WinSize {
    fn default() -> Self {
        Self {
            w: WINDOW_WIDTH,
            h: WINDOW_HEIGHT,
        }
    }
}
// endregion:   Resources

fn main() {
    App::new()
        // Initial setup
        .add_state(GameState::InGame)
        .init_resource::<Game>()
        .init_resource::<WinSize>()
        .init_resource::<AssetScaling>()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        // .insert_resource(Gravity::from(Vec2::new(0., 600.)))
        .insert_resource(WindowDescriptor {
            title: "Rust Space Invaders!".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())

        // .add_plugins(DiagnosticsPluginGroup)// Debug
        .add_plugins(EntitiesPluginGroup)
        .add_plugins(SharedPluginGroup)

        // Insert builtins
        .add_startup_system(setup)

        // Plugins

        .run()
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    // asset_server: Res<AssetServer>,
) {
    println!("Main setup");
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Create main resources
    commands.insert_resource(SpriteInfos {
        player: load_image(&mut images, SPRITE_DIR, PLAYER_SPRITE),
        red_enemy: load_image(&mut images, SPRITE_DIR, RED_ENEMY_SPRITE),
        green_enemy: load_image(&mut images, SPRITE_DIR, GREEN_ENEMY_SPRITE),
        yellow_enemy: load_image(&mut images, SPRITE_DIR, YELLOW_ENEMY_SPRITE),
        player_laser: load_image(&mut images, SPRITE_DIR, PLAYER_LASER_SPRITE),
        alien_laser: load_image(&mut images, SPRITE_DIR, ENEMY_LASER_SPRITE),
        top_alien: load_image(&mut images, SPRITE_DIR, TOP_EXTRA_ENEMY_SPRITE),
    });

    // position window
    // window.set_position(IVec2::new(0, 0));
}
