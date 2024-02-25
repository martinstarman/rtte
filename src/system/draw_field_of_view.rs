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
  for (position, selection, field_of_view) in &query {
    if selection.active && field_of_view.points.len() > 2 {
      let p = Vec2::new(position.x - offset.x, position.y - offset.y);

      for i in 0..field_of_view.points.len() - 1 {
        let q =
          Vec2::new(field_of_view.points[i].x - offset.x, field_of_view.points[i].y - offset.y);
        let r = Vec2::new(
          field_of_view.points[i + 1].x - offset.x,
          field_of_view.points[i + 1].y - offset.y,
        );

        draw_triangle(p, q, r, FIELD_OF_VIEW_COLOR);
      }
    }
  }
}
