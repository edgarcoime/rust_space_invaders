mod system;
mod units;

use bevy::prelude::*;

const WINDOW_WIDTH: f32 = 600.0;
const WINDOW_HEIGHT: f32 = 700.0;

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

        .run()
}
