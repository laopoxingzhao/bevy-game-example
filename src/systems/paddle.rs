use crate::components::{
    Ai, Ball, Collider, Collision, Paddle, Player, Position, Velocity, collide_with_side,
};
use bevy::{math::bounding::Aabb2d, prelude::*};

use super::super::components;
pub const PADDLE_SPEED: f32 = 5.;
pub const PADDLE_SHAPE: Rectangle = Rectangle::new(20., 50.);
pub const PADDLE_COLOR: Color = Color::srgb(0., 1., 0.);

// pub fn spawn_paddles(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     let mesh = meshes.add(PADDLE_SHAPE);
//     let material = materials.add(PADDLE_COLOR);

//     commands.spawn((
//         components::Paddle,
//         Mesh2d(mesh),
//         MeshMaterial2d(material),
//         components::Position(Vec2::new(250., 0.)),
//     ));
// }

pub fn spawn_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Single<&Window>,
) {
    let mesh = meshes.add(PADDLE_SHAPE);
    let material = materials.add(PADDLE_COLOR);
    let half_window_size = window.resolution.size() / 2.;
    let padding = 20.;

    let player_position = Vec2::new(-half_window_size.x + padding, 0.);

    commands.spawn((
        Name::new("Player Paddle"),
        Player,
        Paddle,
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
        Position(player_position),
        Velocity(Vec2::ZERO),
    ));

    let ai_position = Vec2::new(half_window_size.x - padding, 0.);

    commands.spawn((
        Name::new("AI Paddle"),
        Ai,
        Paddle,
        Collider(PADDLE_SHAPE),
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
        Position(ai_position),
        Velocity(Vec2::ZERO),
    ));
}

pub fn handle_collisions(
    ball: Single<(&mut Velocity, &Position, &Collider), With<Ball>>,
    other_things: Query<(&Position, &Collider, Option<&Velocity>), Without<Ball>>,
) {
    let (mut ball_velocity, ball_position, ball_collider) = ball.into_inner();

    for (other_position, other_collider, other_velocity) in &other_things {
        if let Some(collision) = collide_with_side(
            Aabb2d::new(ball_position.0, ball_collider.half_size()),
            Aabb2d::new(other_position.0, other_collider.half_size()),
        ) {
            match collision {
                Collision::Left => {
                    ball_velocity.0.x *= -1.;
                    if let Some(other_velocity) = other_velocity {
                        ball_velocity.0.y += other_velocity.0.y;
                    }
                }
                Collision::Right => {
                    ball_velocity.0.x *= -1.;
                    if let Some(other_velocity) = other_velocity {
                        ball_velocity.0.y += other_velocity.0.y;
                    }
                }
                Collision::Top => {
                    ball_velocity.0.y *= -1.;
                    if let Some(other_velocity) = other_velocity {
                        ball_velocity.0.x += other_velocity.0.x;
                    }
                }
                Collision::Bottom => {
                    ball_velocity.0.y *= -1.;
                    if let Some(other_velocity) = other_velocity {
                        ball_velocity.0.x += other_velocity.0.x;
                    }
                }
            }
        }
    }
}

/* pub fn ai_paddle(
    ai: Single<(&Position, &mut Velocity), With<Ai>>,
    ball: Single<(&Position, &Velocity), With<Ball>>,
    t: Res<Time<Fixed>>,
) {
    let (ball_position, v) = ball.into_inner();
    let (ai_position, mut ai_velocity) = ai.into_inner();
    // let pos = ai_position.0;

    // let ballpos = ball_position.0;
    if v.0.x < 0. {
        return;
    }

    if ball_position.0.y > ai_position.0.y + 20. {
        if ai_velocity.0.y <= PADDLE_SPEED {
            ai_velocity.0.y += t.delta_secs() * 10.0;
        } else if ai_velocity.0.y.abs() > 0.2 {
            ai_velocity.0.y = PADDLE_SPEED;
        } else {
            ai_velocity.0.y = 0.;
        }
    } else if ball_position.0.y < ai_position.0.y - 20. {
        if ai_velocity.0.y >= -PADDLE_SPEED {
            ai_velocity.0.y -= t.delta_secs() * 10.0;
        } else if ai_velocity.0.y.abs() > 0.2 {
            ai_velocity.0.y = -PADDLE_SPEED;
        } else {
            ai_velocity.0.y = 0.;
        }
    } else {
        ai_velocity.0.y -= ai_velocity.0.y * 0.2;
    }
    // 简化AI逻辑，移除对球速的依赖以避免冲突
}
 */
pub fn ai_paddle(
    mut set: ParamSet<(
        Single<(&Position, &Velocity), With<Ball>>,
        Single<(&Position, &mut Velocity), With<Ai>>,
    )>,
    window: Single<&Window>,
    t: Res<Time<Fixed>>,
) {
    // let (ball_position,ball_velocity) = set.p0().clone();

    // 克隆需要的数据，避免重复借用
    let ball_position = set.p0().0.clone();
    let ball_velocity = set.p0().1.clone();

    let (ai_position, mut ai_velocity) = set.p1().into_inner();
    if ai_position.0.y >= window.resolution.size().y / 2. {
        info!("AI paddle hit top boundary");
        ai_velocity.0.y = -ai_velocity.0.y.abs();
    } else if ai_position.0.y <= -window.resolution.size().y / 2. {
        info!("AI paddle hit top boundary");
        ai_velocity.0.y = ai_velocity.0.y.abs();
    }
    // 当球向左移动时，AI不进行操作
    if ball_velocity.0.x < 0. { 
        return;
    }

    if ball_position.0.y > ai_position.0.y + 10. {
        if ai_velocity.0.y <= PADDLE_SPEED {
            ai_velocity.0.y += t.delta_secs() * 5.0;
        } else {
            ai_velocity.0.y = PADDLE_SPEED;
        }
    } else if ball_position.0.y < ai_position.0.y - 10. {
        if ai_velocity.0.y >= -PADDLE_SPEED {
            ai_velocity.0.y -= t.delta_secs() * 5.0;
        } else {
            ai_velocity.0.y = -PADDLE_SPEED;
        }
    } else {
        ai_velocity.0.y += -ai_velocity.0.y * 0.16;
    }
}
pub fn move_paddles(
    mut paddles: Query<(&mut Position, &Velocity), With<Paddle>>,
    t: Res<Time<Fixed>>,
) {
    for (mut pos, vel) in &mut paddles {
        // info!("Moving paddles");
        if vel.0 == Vec2::ZERO {
            continue;
        }
        pos.0 += vel.0 * t.delta_secs() * PADDLE_SPEED * 19.0;
        // info!("Moving paddles");0
    }
}
