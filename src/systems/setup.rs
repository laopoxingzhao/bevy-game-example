use crate::components::Ball;

use super::super::components;
use bevy::{gizmos::gizmos, prelude::*};

pub fn setup(mut commands: Commands) {
    info!("Hello, Bevy!");
    
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn project_positions(
    mut positionables: Query<(&mut Transform, &components::Position)>,
    timestep: Res<Time>,
    mut gizmos: Gizmos, // 添加Gizmos参数
) {
    // let delta = timestep.delta_secs();
    for (mut transform, position) in &mut positionables {
        let target = position.0.extend(0.);
        transform.translation = target;
        gizmos.circle_2d(position.0, 1.0, Color::WHITE);
    }
}
