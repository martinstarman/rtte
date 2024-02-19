use crate::{
  component::{
    enemy::EnemyComponent, position::PositionComponent, selection::SelectionComponent,
    view::ViewComponent,
  },
  constants::VIEW_COLOR,
};
use bevy_ecs::{query::With, system::Query};
use macroquad::{math::Vec2, shapes::draw_triangle};

pub fn run(
  query: Query<(&PositionComponent, &SelectionComponent, &ViewComponent), With<EnemyComponent>>,
) {
  for (position, selection, view) in &query {
    if selection.active && view.polygon.len() >= 3 {
      let pos = Vec2::new(position.x, position.y);

      for i in 0..view.polygon.len() - 1 {
        let p = view.polygon[i];
        let q = view.polygon[i + 1];

        draw_triangle(p, q, pos, VIEW_COLOR);
      }
    }
  }
}
