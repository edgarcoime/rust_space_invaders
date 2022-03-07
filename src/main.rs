mod system;
mod units;
mod utils;

use bevy::prelude::*;
use units::UnitsPluginGroup;
use utils::load_image;

// region:      Constants
const WINDOW_WIDTH: f32 = 600.0;
const WINDOW_HEIGHT: f32 = 600.0;
// endregion:   Constants

// region:      Assets
const SPRITE_DIR: &str = "assets/graphics";
const AUDIO_DIR: &str = "assets/audio";
const FONT_DIR: &str = "assets/font";

const PLAYER_SPRITE: &str = "player.png";
const RED_ENEMY_SPRITE: &str = "red.png";
const GREEN_ENEMY_SPRITE: &str = "green.png";
const YELLOW_ENEMY_SPRITE: &str = "yellow.png";
// endregion:   Assets

// region:      Resources
pub struct SpriteInfos {
    player: (Handle<Image>, Vec2),
    red_enemy: (Handle<Image>, Vec2),
    green_enemy: (Handle<Image>, Vec2),
    yellow_enemy: (Handle<Image>, Vec2),
}
// endregion:   Resources

// region:      Components
// endregion:   Components

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)

        // Insert builtins
        .add_startup_system(setup)

        // Plugins
        .add_plugins(UnitsPluginGroup)

        .run()
}

fn setup(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    mut images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
) {
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // set window
    let mut window = windows.get_primary_mut().unwrap();

    // Create main resources
    commands.insert_resource(SpriteInfos {
        player: load_image(&mut images, PLAYER_SPRITE),
        red_enemy: load_image(&mut images, RED_ENEMY_SPRITE),
        green_enemy: load_image(&mut images, GREEN_ENEMY_SPRITE),
        yellow_enemy: load_image(&mut images, YELLOW_ENEMY_SPRITE),
    });

    // position window
    window.set_position(IVec2::new(0, 0));
}
