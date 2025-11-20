use crate::systems::ball::{BALL_COLOR, BALL_SHAPE, BALL_SIZE, BALL_SPEED};
use crate::systems::paddle::PADDLE_SHAPE;
use bevy::prelude::*;

#[derive(Component, Default,Clone, Copy,Reflect)]
#[reflect(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component, Default,Clone, Copy)]
#[require(Transform)]
pub struct Position(pub Vec2);

#[derive(Component)]
#[require(
  Position = Position(Vec2::new(0., 0.)),
  Velocity = Velocity(Vec2::new(BALL_SPEED, 0.)),
  Collider = Collider(Rectangle::new(BALL_SIZE, BALL_SIZE))
)]
pub struct Ball;

#[derive(Component)]
#[require(Position ,Collider = Collider(PADDLE_SHAPE))]
pub struct Paddle;

use bevy::math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

// Returns `Some` if `ball` collides with `wall`. The returned `Collision` is the
// side of `wall` that `ball` hit.
pub fn collide_with_side(ball: Aabb2d, wall: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&wall) {
        return None;
    }

    let closest_point = wall.closest_point(ball.center());
    let offset = ball.center() - closest_point;

    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}

#[derive(Component, Default)]
pub struct Collider(pub Rectangle);

impl Collider {
    pub fn half_size(&self) -> Vec2 {
        self.0.half_size
    }
}


#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Ai;


#[derive(Component)]
#[require(Position, Collider)]
pub struct Gutter;



#[derive(Component)]
pub struct Menu;