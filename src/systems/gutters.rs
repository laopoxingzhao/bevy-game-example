
use  bevy::prelude::*;

use crate::components::*;


const GUTTER_COLOR: Color = Color::srgb(0., 0., 1.);
const GUTTER_HEIGHT: f32 = 20.;

pub fn spawn_gutters(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  window: Single<&Window>,
) {
  let material = materials.add(GUTTER_COLOR);
  let padding = 20.;

  let gutter_shape = Rectangle::new(window.resolution.width(), GUTTER_HEIGHT);
  let mesh = meshes.add(gutter_shape);

  let top_gutter_position =
    Vec2::new(0., window.resolution.height() / 2. - padding);

  commands.spawn((
    Name::new("Top Gutter"),
    Gutter,
    Mesh2d(mesh.clone()),
    MeshMaterial2d(material.clone()),
    Position(top_gutter_position),
    Collider(gutter_shape),
    Transform {
       translation: top_gutter_position.extend(0.),
         ..default()},

  ));

  let bottom_gutter_position =
    Vec2::new(0., -window.resolution.height() / 2. + padding);

  commands.spawn((
    Name::new("Bottom Gutter"),
    Gutter,
    Mesh2d(mesh.clone()),
    MeshMaterial2d(material.clone()),
    Position(bottom_gutter_position),
    Collider(gutter_shape),
    Transform {
       translation: bottom_gutter_position.extend(0.),
         ..default()},

  ));
}

