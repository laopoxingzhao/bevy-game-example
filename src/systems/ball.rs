use crate::components::{self, Ball};
use bevy::{platform::time, prelude::*};

pub const BALL_SIZE: f32 = 10.;
pub const BALL_SHAPE: Circle = Circle::new(BALL_SIZE);
pub const BALL_COLOR: Color = Color::srgb(1., 0., 0.);
pub const BALL_SPEED: f32 = 17.;

pub fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(BALL_SHAPE);
    let material = materials.add(BALL_COLOR);

    commands.spawn((
        Name::new("Ball"),
        components::Ball,
        Mesh2d(mesh),
        MeshMaterial2d(material),
        // components::Position(Vec2::ZERO), // 添加初始位置
    ));
}

pub fn move_ball(
    ball: Single<(&mut components::Position, &components::Velocity), With<Ball>>,
    timestep: Res<Time<Fixed>>,
) {
    let (mut position, velocity) = ball.into_inner();
    position.0 += velocity.0 *  timestep.delta_secs() * BALL_SPEED;
}
