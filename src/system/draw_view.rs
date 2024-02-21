use crate::{
  component::{
    enemy::EnemyComponent, position::PositionComponent, selection::SelectionComponent,
    view::ViewComponent,
  },
  constants::VIEW_COLOR,
  resource::offset::Offset,
};
use bevy_ecs::{
  query::With,
  system::{Query, Res},
};
use macroquad::{math::Vec2, shapes::draw_triangle};

pub fn run(
  query: Query<(&PositionComponent, &SelectionComponent, &ViewComponent), With<EnemyComponent>>,
  offset: Res<Offset>,
) {
  for (position, selection, view) in &query {
    if selection.active && view.polygon.len() >= 3 {
      let p = Vec2::new(position.x - offset.x, position.y - offset.y);

      for i in 0..view.polygon.len() - 1 {
        let q = Vec2::new(view.polygon[i].x - offset.x, view.polygon[i].y - offset.y);
        let r = Vec2::new(view.polygon[i + 1].x - offset.x, view.polygon[i + 1].y - offset.y);

        draw_triangle(p, q, r, VIEW_COLOR);
      }
    }
  }
}
