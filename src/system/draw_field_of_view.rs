use crate::{
  component::{
    enemy::EnemyComponent, field_of_view::FieldOfViewComponent, position::PositionComponent,
    selection::SelectionComponent,
  },
  constants::FIELD_OF_VIEW_COLOR,
  resource::offset::Offset,
};
use bevy_ecs::{
  query::With,
  system::{Query, Res},
};
use macroquad::{math::Vec2, shapes::draw_triangle};

pub fn draw_field_of_view(
  query: Query<
    (&PositionComponent, &SelectionComponent, &FieldOfViewComponent),
    With<EnemyComponent>,
  >,
  offset: Res<Offset>,
) {
  for (position, selection, view) in &query {
    if selection.active && view.points.len() >= 3 {
      let p = Vec2::new(position.x - offset.x, position.y - offset.y);

      for i in 0..view.points.len() - 1 {
        let q = Vec2::new(view.points[i].x - offset.x, view.points[i].y - offset.y);
        let r = Vec2::new(view.points[i + 1].x - offset.x, view.points[i + 1].y - offset.y);

        draw_triangle(p, q, r, FIELD_OF_VIEW_COLOR);
      }
    }
  }
}
