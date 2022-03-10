use bevy::{prelude::*, ecs::bundle, utils::HashSet};

use crate::{shared::{Health, RenderedAssetInfo}, WinSize};

const DEFAULT_OBSTACLE_AMOUNT: u32 = 4;
const OBSTACLE_BLOCK_SIZE: f32 = 6.;
const OBSTACLE_SHAPE: [&str; 7] = [
    "  xxxxxxx",
    " xxxxxxxxx",
    "xxxxxxxxxxx",
    "xxxxxxxxxxx",
    "xxxxxxxxxxx",
    "xxx     xxx",
    "xx       xx" 
];

#[derive(Bundle)]
struct BlockBundle {
    #[bundle]
    _sb: SpriteBundle,
    _o: Obstacle,
    _hp: Health,
    _rai: RenderedAssetInfo,
}
impl BlockBundle {
    fn new(x: f32, y: f32, color: Color) -> Self {
        let obs_size = Vec2::new(OBSTACLE_BLOCK_SIZE, OBSTACLE_BLOCK_SIZE) ;
        Self {
            _sb: SpriteBundle {
                sprite: Sprite {
                    color: color,
                    custom_size: Some(obs_size.clone()),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(x, y, 5.), 
                    ..Default::default()
                },
                ..Default::default()
            },
            _o: Obstacle,
            _hp: Health::default(),
            _rai: RenderedAssetInfo::new(obs_size.clone())
        }
    }
}

#[derive(Component)]
pub struct Obstacle;

pub struct ObstaclesPlugin;
impl Plugin for ObstaclesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_obstacles)
        ;
    }
}

fn setup_obstacles(
    mut commands: Commands,
    win_size: Res<WinSize>,
) {
    let x_start = (-win_size.w / 2.) + (win_size.w / 15.);
    let y_start = -(win_size.h/2.) + 100.;
    let num_obstacles = DEFAULT_OBSTACLE_AMOUNT;

    let obstacle_x_offsets = 
        (0..num_obstacles)
        .map(|x| x as f32 * (win_size.w / num_obstacles as f32))
        .collect::<Vec<f32>>();
        
    println!("{:?}", obstacle_x_offsets);
    for offset in obstacle_x_offsets {
        create_obstacle(
            &mut commands, 
            Vec2::new(x_start, y_start), 
            offset
        );
    }
}

fn create_obstacle(commands: &mut Commands, start_position: Vec2, offset_x: f32) {
    for (row_idx, row) in OBSTACLE_SHAPE.iter().rev().enumerate() {
        for (col_idx, col) in row.chars().enumerate() {
            if col == 'x' {
                let x = start_position.x + (col_idx as f32 * (OBSTACLE_BLOCK_SIZE)) + offset_x;
                let y = start_position.y + (row_idx as f32 * (OBSTACLE_BLOCK_SIZE));

                let color = Color::hex("F14F50").unwrap();
                commands
                    .spawn_bundle(BlockBundle::new(x, y, color));
                    // .spawn_bundle(BlockBundle::new(x, y, Color::ORANGE_RED));
            }
        }
    }
}