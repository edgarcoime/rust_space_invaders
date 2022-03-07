use bevy::{prelude::*, ecs::system::Insert};

use crate::{SpriteInfos, WinSize};

use super::Health;

// region:      Resources
// endregion:   Resources

// region:      Components
#[derive(Component)]
struct PlayerName(String);

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    name: PlayerName,
    health: Health,
    _p: Player,
}
// endregion:   Components

// region:      Entities
// endregion:   Entities

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_stage(
                "game_setup_player"
                , SystemStage::single(player_spawn));
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
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Health { original_hp: 0, current_hp: 0 })
        .insert(PlayerName("Player 1".to_string()))
        ;
}

fn setup() {}
