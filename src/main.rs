// region:      Modules
mod system;
mod utils;
mod diagnostics;
mod entities;
mod shared;
// endregion:   Modules

use bevy::{prelude::*};
use diagnostics::DiagnosticsPluginGroup;
use entities::EntitiesPluginGroup;
use shared::SharedPluginGroup;
use utils::load_image;

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

#[derive(Debug)]
pub struct SpriteInfos {
    player: (Handle<Image>, Vec2),
    player_laser: (Handle<Image>, Vec2),
    red_enemy: (Handle<Image>, Vec2),
    green_enemy: (Handle<Image>, Vec2),
    yellow_enemy: (Handle<Image>, Vec2),
    alien_laser: (Handle<Image>, Vec2),
}
// endregion:   Resources

// region:      Components
// endregion:   Components

// region:      Entities
// endregion:   Entities

fn main() {
    App::new()
        // Initial setup
        .add_state(GameState::InGame)
        .init_resource::<Game>()
        .init_resource::<WinSize>()
        .init_resource::<AssetScaling>()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        // Insert builtins
        .add_startup_system(setup.label("main_setup"))
        .add_plugins(DiagnosticsPluginGroup)// Debug
        .add_plugins(SharedPluginGroup)// Debug
        .add_plugins(EntitiesPluginGroup)

        .run()
}

fn setup(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    mut images: ResMut<Assets<Image>>,
) {
    let v1 = Vec2::new(12., 12.);
    let v2 = Vec2::new(12., 12.);
    println!("{}", (v1 + v2));
    println!("{}", -(v1 + v2));

    println!("Main setup");
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // set window
    let window = windows.get_primary_mut().unwrap();

    // Create main resources
    commands.insert_resource(SpriteInfos {
        player: load_image(&mut images, SPRITE_DIR, PLAYER_SPRITE),
        red_enemy: load_image(&mut images, SPRITE_DIR, RED_ENEMY_SPRITE),
        green_enemy: load_image(&mut images, SPRITE_DIR, GREEN_ENEMY_SPRITE),
        yellow_enemy: load_image(&mut images, SPRITE_DIR, YELLOW_ENEMY_SPRITE),
        player_laser: load_image(&mut images, SPRITE_DIR, PLAYER_LASER_SPRITE),
        alien_laser: load_image(&mut images, SPRITE_DIR, ENEMY_LASER_SPRITE),
    });

    // position window
    window.set_position(IVec2::new(0, 0));
}