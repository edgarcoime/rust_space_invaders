use bevy::prelude::*;

use crate::{WinSize, SpriteInfos, shared::{Health, WeaponState, MovementSpeed}};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerState {
    name: String
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_stage(
                "player_setup", 
                SystemStage::single(player_spawn)
            )
        ;
    }
}

fn player_spawn (
    mut commands: Commands,
    win_size: Res<WinSize>,
    sprite_infos: Res<SpriteInfos>,
) {
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
        .insert(Health::default())
        .insert(MovementSpeed { value: 250. })
        .insert(PlayerState { name: "Player 1".to_string() })
        .insert(WeaponState::fast_normal_weapon())
    ;
}