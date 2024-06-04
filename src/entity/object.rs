use crate::component::{
  animation::AnimationComponent,
  object::{ObjectBundle, ObjectComponent},
  position::PositionComponent,
  shape::{ShapeComponent, ShapeType},
  size::SizeComponent,
  sprite::{SpriteBundle, SpriteComponent},
};
use bevy_ecs::component::ComponentId;
use macroquad::{math::Vec2, texture::load_texture};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ObjectEntity {
  image: String,
  position: (f32, f32),
  shape: Vec<(f32, f32)>,
  r#type: String,
  ysorted: bool,
}

impl ObjectEntity {
  pub async fn into(
    &self,
    index: usize,
    // rigid_body_set: &mut RigidBodySet,
    // collider_set: &mut ColliderSet,
  ) -> ObjectBundle {
    let texture = load_texture(self.image.as_str()).await.unwrap();
    let mut points: Vec<Vec2> = vec![];
    let mut lines: Vec<(Vec2, Vec2)> = vec![];
    let animation = AnimationComponent::default(); // TODO: implement me

    if self.shape.len() > 2 {
      for i in 0..self.shape.len() {
        let p = self.shape[i];
        let q = self.shape[if i == self.shape.len() - 1 { 0 } else { i + 1 }];

        points.push(p.into());
        lines.push((p.into(), q.into()));
      }

      // close line
      let first = self.shape.first().unwrap();
      let last = self.shape.last().unwrap();

      lines.push((Vec2::new(last.0, last.1), Vec2::new(first.0, first.1)));
    }

    // TODO: from_str
    let r#type = match self.r#type.as_str() {
      "none" => ShapeType::None,
      "transparent" => ShapeType::Transparent,
      "water" => ShapeType::Water,
      "snow" => ShapeType::Snow,
      _ => ShapeType::Block,
    };

    // TODO: implement me when rapier supports top down https://github.com/dimforge/rapier/issues/449
    // TODO: avoid non blocking objects
    // let rigid_body = RigidBodyBuilder::new(RigidBodyType::Fixed).build();
    // let rigid_body_handle = rigid_body_set.insert(rigid_body);
    // let collider_points: Vec<_> =
    //   points.iter().map(|p| Point2::new(self.position.0 + p.x, self.position.1 + p.y)).collect();
    // let collider = ColliderBuilder::convex_hull(&collider_points[..]).unwrap().build();
    // let collider_handle =
    //   collider_set.insert_with_parent(collider, rigid_body_handle, rigid_body_set);

    ObjectBundle {
      // body: BodyComponent {
      //   collider_handle,
      //   rigid_body_handle,
      // },
      object: ObjectComponent {
        id: ComponentId::new(index),
      },
      position: PositionComponent {
        x: self.position.0,
        y: self.position.1,
      },
      shape: ShapeComponent {
        lines,
        points,
        r#type,
      },
      size: SizeComponent {
        height: texture.height(),
        width: texture.width(),
      },
      sprite: SpriteBundle {
        sprite: SpriteComponent {
          texture,
          ysorted: self.ysorted,
        },
        animation,
      },
    }
  }
}
