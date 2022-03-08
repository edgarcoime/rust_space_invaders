// region:      Modules
mod system;
mod utils;
mod diagnostics;
mod entities;
mod shared;
// endregion:   Modules

use bevy::{prelude::*, ecs::{archetype::Archetypes, component::ComponentId}};
use bevy_rapier2d::prelude::*;
use diagnostics::DiagnosticsPluginGroup;
use entities::EntitiesPluginGroup;
use shared::SharedPluginGroup;
use utils::load_image;
use nalgebra::vector;

use crate::entities::Enemy;

// region:      Constants
const WINDOW_WIDTH: f32 = 600.0;
const WINDOW_HEIGHT: f32 = 600.0;
const GAME_TIME_STEP: f32 = 1. / 60.;
// const PHYSICS_SCALE: f32 = 50.;
const PHYSICS_SCALE: f32 = 50.;
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

#[derive(Debug)]
pub struct SpriteInfos {
    player: (Handle<Image>, Vec2),
    player_laser: (Handle<Image>, Vec2),
    red_enemy: (Handle<Image>, Vec2),
    green_enemy: (Handle<Image>, Vec2),
    yellow_enemy: (Handle<Image>, Vec2),
}

pub struct WinSize {
    w: f32,
    h: f32,
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

        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())

        // Insert builtins
        .add_startup_system(setup.label("main_setup"))
        .add_plugins(DiagnosticsPluginGroup)// Debug
        .add_plugins(SharedPluginGroup)// Debug
        .add_plugins(EntitiesPluginGroup)

        // Plugins

        // .add_startup_stage(
        //     "testing", 
        //     SystemStage::single(test_spawn.after("main_setup"))
        // )

        .run()
}

fn setup(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    mut images: ResMut<Assets<Image>>,
    mut rapier_config: ResMut<RapierConfiguration>,
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
    });
    commands.insert_resource(WinSize {
        w: window.width(),
        h: window.height(),
    });

    // position window
    window.set_position(IVec2::new(0, 0));

    // Scale is saying
    // given 1 physics meter how many pixels?
    rapier_config.scale = PHYSICS_SCALE;
    rapier_config.gravity = vector![0., 0.];
}

fn test_spawn(
    mut commands: Commands,
    win_size: Res<WinSize>,
    sprite_infos: Res<SpriteInfos>,
    rapier_config: ResMut<RapierConfiguration>,
) {
    println!("{:?}", sprite_infos.green_enemy.1);

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: sprite_infos.green_enemy.0.clone(),
            transform: Transform::default(),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::KinematicVelocityBased.into(),
            velocity: RigidBodyVelocity {
                linvel: Vec2::new(0., 1.0).into(),
                angvel: 0.0,
            }.into(),
            ..Default::default()
        })
        .insert(Enemy)
        ;
}

pub fn get_components_for_entity<'a>(
    entity: &Entity,
    archetypes: &'a Archetypes,
) -> Option<impl Iterator<Item = ComponentId> + 'a> {
    for archetype in archetypes.iter() {
        if archetype.entities().contains(entity) {
            return Some(archetype.components());
        }
    }
    None
}