use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{WinSize, SpriteInfos, Game};
// Bosses, mobs
// Can even extend to another module if needed

// region:      Resources
// endregion:   Resources

// region:      Components
#[derive(Component)]
pub struct Enemy;
// endregion:   Components

// region:      Entities
// endregion:   Entities

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(enemy_spawn)
        ;
    }
}

fn enemy_spawn(
    mut commands: Commands,
    mut game: ResMut<Game>,
    win_size: Res<WinSize>,
    sprite_infos: Res<SpriteInfos>,
) {
    if game.active_enemies < 2 {
        // TODO: get random position?
        // compute the rando position
        let mut rng = thread_rng();
        let w_span = win_size.w / 2. - 100.;
        let h_span = win_size.h / 2. - 100.;
        let x = rng.gen_range(-w_span..w_span) as f32;
        let y = rng.gen_range(-h_span..h_span) as f32;

        // spawn enemy
        commands
            .spawn_bundle(SpriteBundle {
                texture: sprite_infos.red_enemy.0.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 10.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Enemy);

        game.active_enemies += 1;
    }
}
