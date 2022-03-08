use bevy::{prelude::*, app::PluginGroupBuilder};
use bevy_rapier2d::{prelude::{RigidBodyVelocityComponent, RigidBodyPositionComponent, ColliderPosition, ColliderPositionComponent}, physics::RapierConfiguration};

use crate::entities::Enemy;

pub struct DiagnosticsPluginGroup;
impl PluginGroup for DiagnosticsPluginGroup {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            // Adds frame time diagnostics
            // .add(FrameTimeDiagnosticsPlugin::default())
            // Adds a system that prints diagnostics to the console
            // .add(LogDiagnosticsPlugin::default())
            // Any plugin can register diagnostics
            // Uncomment this to add some render resource diagnostics:
            // .add_plugin(bevy::wgpu::diagnostic::WgpuResourceDiagnosticsPlugin::default())
            // Uncomment this to add an entity count diagnostics:
            .add(bevy::diagnostic::EntityCountDiagnosticsPlugin::default())
            // Uncomment this to add an asset count diagnostics:
            // .add_plugin(bevy::asset::diagnostic::AssetCountDiagnosticsPlugin::<Texture>::default())
            ;
    }
}


