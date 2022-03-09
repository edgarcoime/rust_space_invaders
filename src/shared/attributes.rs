use bevy::prelude::*;

#[derive(Component)]
pub struct MovementSpeed {
    pub value: f32,
}

#[derive(Component)]
pub struct Health {
    pub original_hp: u32,
    pub current_hp: u32,
}
impl Default for Health {
    fn default() -> Self {
        Self {
            original_hp: 1,
            current_hp: 1,
        }
    }
}
impl Health {
    pub fn from(hp: u32) -> Self {
        Self {
            original_hp: hp,
            current_hp: hp,
        }
    }
}

pub struct AttributesPlugin;
impl Plugin for AttributesPlugin {
    fn build(&self, app: &mut App) {}
}
