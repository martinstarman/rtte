pub mod components;

use bevy_ecs::{component::ComponentId, schedule::Schedule, system::Query, world::World};
use components::{
  enemy::{Enemy, EnemyBundle},
  movable::Movable,
  object::{Object, ObjectBundle, PolygonType},
  player::{Player, PlayerBundle},
  position::Position,
  renderable::Renderable,
  selectable::Selectable,
  size::Size,
  view::{View, ViewMovement},
};
use ggez::{
  event::{self, EventHandler, MouseButton},
  graphics::{Canvas, Color, DrawMode, DrawParam, Image, Rect},
  input::keyboard::KeyCode,
  mint::Point2,
  Context, ContextBuilder, GameError, GameResult,
};
use maths_rs::{distance, line_segment_vs_line_segment, Vec2f, Vec3f};
use std::{f32::consts::PI, path};

/// Show debug stuff?
const DEBUG: bool = true;

/// Window width.
const WINDOW_WIDTH: f32 = 800.;

/// Window height.
const WINDOW_HEIGHT: f32 = 600.;

/// Panning speed,
const PAN_SPEED: f32 = 5.;

/// 1 radian.
const RADIAN: f32 = PI / 180.;

/// View distance.
const VIEW_DISTANCE: f32 = 150.;

/// View inner angle.
const VIEW_INNER_ANGLE: f32 = 60. * RADIAN;

/// View movement angle. Maximum view movement left and right.
const VIEW_MOVEMENT_ANGLE: f32 = 30. * RADIAN;

/// Game data.
pub struct Game {
  /// Game world.
  world: World,

  /// Bevy ECS scheduler.
  schedule: Schedule,

  /// Camera position.
  camera: Point2<f32>,
}

/// Game implementation.
impl Game {
  /// Create new game.
  pub fn new(ctx: &mut Context) -> GameResult<Game> {
    let mut world = World::default();

    world.spawn(ObjectBundle {
      position: Position { x: 0., y: 0. },
      size: Size { w: 1000., h: 800. },
      renderable: Renderable {
        sprite: Image::from_path(ctx, "/ground.png").unwrap(),
        y_indexed: false,
      },
      object: Object {
        polygon: vec![],
        polygon_type: PolygonType::GROUND,
      },
    });

    world.spawn(ObjectBundle {
      position: Position { x: 250., y: 200. },
      size: Size { w: 160., h: 236. },
      renderable: Renderable {
        sprite: Image::from_path(ctx, "/block.png").unwrap(),
        y_indexed: true,
      },
      object: Object {
        polygon: vec![
          Point2 { x: 128., y: 236. },
          Point2 { x: 160., y: 219. },
          Point2 { x: 32., y: 154. },
          Point2 { x: 0., y: 171. },
        ],
        polygon_type: PolygonType::GROUND,
      },
    });

    world.spawn(PlayerBundle {
      position: Position { x: 1., y: 1. },
      size: Size { w: 10., h: 23. },
      renderable: Renderable {
        sprite: Image::from_path(ctx, "/player.png").unwrap(),
        y_indexed: true,
      },
      movable: Movable {
        path: vec![],
        path_default: vec![],
      },
      player: Player {
        id: ComponentId::new(1),
      },
      selectable: Selectable { selected: false },
    });

    world.spawn(PlayerBundle {
      position: Position { x: 30., y: 1. },
      size: Size { w: 10., h: 23. },
      renderable: Renderable {
        sprite: Image::from_path(ctx, "/player.png").unwrap(),
        y_indexed: true,
      },
      movable: Movable {
        path: vec![],
        path_default: vec![],
      },
      player: Player {
        id: ComponentId::new(2),
      },
      selectable: Selectable { selected: false },
    });

    world.spawn(EnemyBundle {
      position: Position { x: 450., y: 400. },
      size: Size { w: 10., h: 23. },
      renderable: Renderable {
        sprite: Image::from_path(ctx, "/player.png").unwrap(), // TODO: enemy.png
        y_indexed: true,
      },
      movable: Movable {
        path: vec![],
        path_default: vec![/*Point2 { x: 200., y: 200. }, Point2 { x: 100., y: 100. }*/],
      },
      view: View {
        points: vec![],
        current_direction: 180. * RADIAN,
        direction: 180. * RADIAN,
        movement: ViewMovement::LEFT,
      },
      enemy: Enemy {
        id: ComponentId::new(1),
      },
      selectable: Selectable { selected: false },
    });

    let mut schedule = Schedule::default();

    schedule.add_systems(movement);
    schedule.add_systems(view);

    let game = Game {
      world,
      schedule,
      camera: Point2 { x: 0., y: 0. },
    };

    Ok(game)
  }
}

/// ggez event handler for Game.
impl EventHandler<GameError> for Game {
  /// Every tick update.
  fn update(&mut self, ctx: &mut Context) -> GameResult {
    // Run bevy_ecs systems.
    self.schedule.run(&mut self.world);

    // Change camera position by arrow keys.
    for key in ctx.keyboard.pressed_keys() {
      match key {
        KeyCode::Left => self.camera.x -= PAN_SPEED,
        KeyCode::Right => self.camera.x += PAN_SPEED,
        KeyCode::Up => self.camera.y -= PAN_SPEED,
        KeyCode::Down => self.camera.y += PAN_SPEED,
        _ => (),
      }
    }

    // Change camera position by mouse.
    let pos = ctx.mouse.position();

    if pos.x == 0. {
      self.camera.x -= PAN_SPEED;
    }

    if pos.x == WINDOW_WIDTH - 1. {
      self.camera.x += PAN_SPEED;
    }

    if pos.y == 0. {
      self.camera.y -= PAN_SPEED;
    }

    if pos.y == WINDOW_HEIGHT - 1. {
      self.camera.y += PAN_SPEED;
    }

    Ok(())
  }

  /// Every tick draw.
  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    // Reset canvas.
    let mut canvas = Canvas::from_frame(ctx, Color::from_rgb(255, 0, 255));

    // Draw non Y indexed entities.
    draw_entity(self, ctx, &mut canvas, false);

    // Draw view.
    draw_view(self, ctx, &mut canvas);

    // Draw Y indexed entities.
    draw_entity(self, ctx, &mut canvas, true);

    // Draw debug stuff.
    if DEBUG {
      draw_entity_debug(self, ctx, &mut canvas);
    }

    canvas.finish(ctx)?;

    Ok(())
  }

  fn mouse_button_down_event(
    &mut self,
    _ctx: &mut Context,
    _button: MouseButton,
    _x: f32,
    _y: f32,
  ) -> Result<(), GameError> {
    // TODO: multiple player selection

    Ok(())
  }

  fn mouse_button_up_event(
    &mut self,
    _ctx: &mut Context,
    btn: MouseButton,
    x: f32,
    y: f32,
  ) -> Result<(), GameError> {
    let x = x + self.camera.x;
    let y = y + self.camera.y;

    match btn {
      MouseButton::Left => select_or_move_player(self, x, y),
      MouseButton::Right => select_or_stop_player(self, x, y),
      _ => {}
    }

    Ok(())
  }
}

/// Draw entity.
fn draw_entity(game: &mut Game, _ctx: &mut Context, canvas: &mut Canvas, y_indexed: bool) {
  let mut query = game.world.query::<(&Position, &Size, &Renderable)>();
  let mut entities: Vec<_> = query
    .iter_mut(&mut game.world)
    .filter(|(_, _, renderable)| renderable.y_indexed == y_indexed)
    .collect();

  // Sort by Y index.
  if y_indexed {
    entities.sort_by(|(a_pos, a_size, _), (b_pos, b_size, _)| {
      (a_pos.y + a_size.h).partial_cmp(&(b_pos.y + b_size.h)).unwrap()
    });
  }

  for (position, _, renderable) in entities {
    let dest = Point2 {
      x: position.x - game.camera.x,
      y: position.y - game.camera.y,
    };

    canvas.draw(&renderable.sprite, DrawParam::new().dest(dest));
  }
}

/// Draw view.
fn draw_view(game: &mut Game, ctx: &mut Context, canvas: &mut Canvas) {
  // TODO: show view only for 1 selected enemy
  let mut query = game.world.query::<&View>();

  for view in query.iter_mut(&mut game.world) {
    if view.points.len() >= 3 {
      let mesh = ggez::graphics::Mesh::new_polygon(
        ctx,
        DrawMode::fill(),
        &view.points[..],
        Color::from_rgba(255, 0, 0, 127),
      )
      .unwrap();
      canvas.draw(&mesh, DrawParam::new().offset(game.camera));
    }
  }
}

/// Draw entity debug.
fn draw_entity_debug(game: &mut Game, ctx: &mut Context, canvas: &mut Canvas) {
  // Draw entity rect.
  let mut query = game.world.query::<(&Position, &Size)>();

  for (position, size) in query.iter_mut(&mut game.world) {
    let rect = Rect::new(position.x, position.y, size.w, size.h);
    let mesh =
      ggez::graphics::Mesh::new_rectangle(ctx, DrawMode::stroke(1.), rect, Color::WHITE).unwrap();
    canvas.draw(&mesh, DrawParam::new().offset(game.camera));
  }

  // Draw object poly.
  let mut query = game.world.query::<(&Object, &Position)>();

  for (object, position) in query.iter_mut(&mut game.world) {
    let mut points: Vec<Point2<f32>> = vec![];

    if object.polygon.len() >= 3 {
      for point in &object.polygon {
        points.push(Point2 {
          x: position.x + point.x,
          y: position.y + point.y,
        });
      }

      let mesh =
        ggez::graphics::Mesh::new_polygon(ctx, DrawMode::stroke(1.), &points[..], Color::WHITE)
          .unwrap();
      canvas.draw(&mesh, DrawParam::new().offset(game.camera));
    }
  }
}

/// Left mouse button click handler.
fn select_or_move_player(game: &mut Game, x: f32, y: f32) {
  let mut selected_player_id: Option<ComponentId> = None;

  // Try to select player.
  let mut query = game.world.query::<(&Player, &mut Selectable, &Position, &Size)>();

  for (player, mut selectable, position, size) in query.iter_mut(&mut game.world) {
    let rect = Rect::new(position.x, position.y, size.w, size.h);

    if rect.contains(Point2 { x, y }) {
      selectable.selected = true;
      selected_player_id = Some(player.id);
    }
  }

  // Deselect all players if some was selected.
  if let Some(id) = selected_player_id {
    let mut query = game.world.query::<(&Player, &mut Selectable)>();

    for (player, mut selectable) in query.iter_mut(&mut game.world) {
      if player.id != id {
        selectable.selected = false;
      }
    }
  }

  // Set path to selected player when no player was selected.
  if selected_player_id.is_none() {
    let mut query = game.world.query::<(&Player, &Selectable, &mut Movable)>();

    for (_, selectable, mut movable) in query.iter_mut(&mut game.world) {
      if selectable.selected {
        movable.path = vec![Point2 { x, y }]; // TODO: check where is clicked
      }
    }
  }
}

/// Right mouse button click handler.
fn select_or_stop_player(game: &mut Game, _x: f32, _y: f32) {
  // TODO: multiple player selection

  // Stop player movement.
  let mut query = game.world.query::<(&Player, &mut Movable)>();

  for (_, mut movable) in query.iter_mut(&mut game.world) {
    movable.path = vec![];
  }
}

/// Entity movement.
fn movement(mut query: Query<(&mut Movable, &mut Position)>) {
  for (mut movable, mut position) in &mut query {
    if movable.path.len() > 0 {
      let next = movable.path[0];
      let dist =
        distance::<f32, Vec2f>(Vec2f::new(next.x, next.y), Vec2f::new(position.x, position.y));

      if dist < 1. {
        movable.path.remove(0);

        position.x = next.x;
        position.y = next.y;
      } else {
        let dx = next.x - position.x;
        let dy = next.y - position.y;

        position.x = position.x + (dx / dist);
        position.y = position.y + (dy / dist);
      }
    } else {
      if movable.path_default.len() > 0 {
        movable.path = movable.path_default.clone();
      }
    }
  }
}

/// Entity view.
// TODO: update view position when enemy position change. Use bevy ecs change detection.
fn view(query1: Query<(&Object, &Position)>, mut query2: Query<(&mut View, &Position)>) {
  // Build barriers from objects.
  let mut barriers: Vec<(Point2<f32>, Point2<f32>)> = vec![];

  for (object, position) in &query1 {
    if object.polygon.len() >= 3 {
      for i in 0..object.polygon.len() - 1 {
        let curr = object.polygon[i];
        let next = object.polygon[i + 1];

        barriers.push((
          Point2 {
            x: curr.x + position.x,
            y: curr.y + position.y,
          },
          Point2 {
            x: next.x + position.x,
            y: next.y + position.y,
          },
        ));
      }

      let first = object.polygon.first().unwrap();
      let last = object.polygon.last().unwrap();

      barriers.push((
        Point2 {
          x: last.x + position.x,
          y: last.y + position.y,
        },
        Point2 {
          x: first.x + position.x,
          y: first.y + position.y,
        },
      ));
    }
  }

  // Build view.
  for (mut view, position) in &mut query2 {
    // Change view movement.
    let d = view.current_direction - view.direction;

    if d > VIEW_MOVEMENT_ANGLE || d < -VIEW_MOVEMENT_ANGLE {
      view.movement =
        if view.movement == ViewMovement::LEFT { ViewMovement::RIGHT } else { ViewMovement::LEFT };
    }

    // Move view.
    view.direction += if view.movement == ViewMovement::LEFT { -RADIAN } else { RADIAN };

    // Get new view.
    let mut points: Vec<Point2<f32>> = vec![Point2 {
      x: position.x,
      y: position.y,
    }];

    let min = view.direction - (VIEW_INNER_ANGLE / 2.);
    let max = view.direction + (VIEW_INNER_ANGLE / 2.);
    let mut current = min;

    while current < max {
      let mut min_dist = VIEW_DISTANCE;

      // View point.
      let mut p = Vec2f::new(
        f32::cos(current) * VIEW_DISTANCE + position.x,
        f32::sin(current) * VIEW_DISTANCE + position.y,
      );

      for barrier in &barriers {
        // Ray from entity position to view point vs barriers.
        let res = line_segment_vs_line_segment(
          Vec3f::new(position.x, position.y, 0.),
          p.into(),
          Vec3f::new(barrier.0.x, barrier.0.y, 0.),
          Vec3f::new(barrier.1.x, barrier.1.y, 0.),
        );

        // Ray was intersected by some barrier.
        if let Some(intersection) = res {
          let dist =
            distance::<f32, Vec2f>(Vec2f::new(position.x, position.y), intersection.into());

          if dist < min_dist {
            p = intersection.into();
            min_dist = dist;
          }
        }
      }

      points.push(Point2 { x: p.x, y: p.y });
      current += RADIAN;
    }

    view.points = points;
  }
}

/// Main function.
fn main() -> GameResult {
  let resource_dir = path::PathBuf::from("./resources");

  let context_builder = ContextBuilder::new("rtte", "rtte")
    .window_setup(ggez::conf::WindowSetup::default().title("rtte"))
    .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
    .add_resource_path(resource_dir);

  let (mut ctx, event_loop) = context_builder.build()?;
  let state = Game::new(&mut ctx)?;

  // Lock mouse to window.
  ggez::input::mouse::set_cursor_grabbed(&mut ctx, true)?;

  event::run(ctx, event_loop, state)
}
