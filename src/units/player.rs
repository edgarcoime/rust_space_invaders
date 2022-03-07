use bevy::{prelude::*};

use crate::{SpriteInfos, WinSize, GAME_TIME_STEP};

use super::{Health, Velocity};

// region:      Resources
// endregion:   Resources

// region:      Components
#[derive(Component)]
struct PlayerName(String);

#[derive(Component)]
pub struct Player;

// #[derive(Bundle)]
// struct PlayerBundle {
//     name: PlayerName,
//     health: Health,
//     _p: Player,
// }
// endregion:   Components

// region:      Entities
// endregion:   Entities

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_stage(
                "game_setup_player"
                , SystemStage::single(player_spawn)
            )
            .add_system(player_movement)
        ;
    }
}

fn player_spawn (
    mut commands: Commands,
    win_size: Res<WinSize>,
    sprite_infos: Res<SpriteInfos>,
) {
    println!("Player spawn");
    let bottom = -win_size.h / 2.;
    commands
        .spawn_bundle(SpriteBundle {
            texture: sprite_infos.player.0.clone(),
            transform: Transform {
                translation: Vec3::new(0., bottom + 75. / 3. + 5., 10.),
                scale: Vec3::new(1., 1., 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(Velocity::default())
        .insert(Health::default())
        .insert(PlayerName("Player 1".to_string()))
        ;
    println!("Player succesfully spawned");
}

fn player_movement(
    kb_in: Res<Input<KeyCode>>,
    mut q: Query<(&Velocity, &mut Transform), With<Player>>,
) {
    if let Ok((vel, mut tf)) = q.get_single_mut() {
        // TODO: QUERY WILL TRY TO MATCH ALL OF DESIRED
        // SO WILL NOT WORK IF YOUR DESIRED DOES NOT IMPLEMENT BOTH COMPONENTS
        let dir =
        if kb_in.pressed(KeyCode::Left) || kb_in.pressed(KeyCode::A) {
            -1.
        } else if kb_in.pressed(KeyCode::Right) || kb_in.pressed(KeyCode::D) {
            1.
        } else {
            0.
        };

        tf.translation.x += dir * vel.0 * GAME_TIME_STEP;
    }
}