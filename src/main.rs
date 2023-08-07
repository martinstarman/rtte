///
pub mod entity;

///
pub mod geometry;

///
pub mod gui;

///
pub mod resource;

use crate::{
  entity::{character::Character, enemy::Enemy, object::Object},
  geometry::{mesh::Mesh, vec2::Vec2},
};

use ggegui::{
  egui::{pos2, Rect},
  Gui,
};
use ggez::{
  event::{self, EventHandler, MouseButton},
  graphics::{Canvas, Color, DrawParam},
  input::keyboard::KeyCode,
  input::keyboard::KeyMods,
  mint::Point2,
  Context, ContextBuilder, GameError, GameResult,
};
use resource::Resource;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::prelude::*, path};

#[derive(Default, PartialEq)]
enum Mode {
  #[default]
  Runtime = 0,
  Edit = 1,
}

#[derive(Serialize, Deserialize)]
pub struct State {
  #[serde(skip)]
  offset: Vec2,
  #[serde(skip)]
  scale: Vec2,
  mesh: Mesh,
  characters: Vec<Character>,
  enemies: Vec<Enemy>,
  objects: Vec<Object>,
  #[serde(skip)]
  mode: Mode,
  #[serde(skip)]
  gui: Gui,
  #[serde(skip)]
  #[serde(default = "rect_default")]
  gui_window_rect: Option<Rect>,
  #[serde(skip)]
  resources: Vec<Resource>,
}

fn rect_default() -> Option<Rect> {
  Some(Rect::NOTHING)
}

const OFFSET_SPEED: f32 = 10.;

impl State {
  // create new state
  pub fn new(ctx: &mut Context) -> GameResult<State> {
    let mut state = State {
      mesh: Mesh::new(800., 600.),
      offset: Vec2::default(),
      scale: Vec2::new(1., 1.),
      characters: vec![],
      enemies: vec![],
      objects: vec![],
      mode: Mode::Runtime,
      gui: Gui::new(ctx),
      gui_window_rect: Some(Rect::NOTHING),
      // TODO: vector of tuples (name: String, img: Image)?
      resources: vec![],
    };

    let resources: Vec<_> = ctx.fs.read_dir("/")?.collect();

    for item in resources {
      let path = item.to_str().unwrap().to_string();
    
      let res = Resource::new(path, ctx);

      state.resources.push(res);
    }

    Ok(state)
  }

  // serialize game state to TOML file
  pub fn save(&self) {
    let serialized = toml::to_string(self).unwrap();
    let mut file = File::create("demo.toml").unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
  }

  // deserialize TOML file to game state
  pub fn load(&mut self) {
    let mut data = String::new();
    let mut file = File::open("demo.toml").expect("Unable to open save file");

    file.read_to_string(&mut data).expect("Unable to read save file");

    let deserialized: State = toml::from_str(data.as_str()).unwrap();

    self.mesh = deserialized.mesh;
    self.characters = deserialized.characters;
    // TODO: if we deserialize enemy pos, pov_dest does not get updated...
    //       one solution should be serialized pov_dest
    self.enemies = deserialized.enemies;
    self.objects = deserialized.objects;
  }

  // toggle game modes
  pub fn toggle_mode(&mut self) {
    self.mode = match self.mode {
      Mode::Runtime => Mode::Edit,
      Mode::Edit => Mode::Runtime,
    }
  }

  // add new character to the game
  pub fn add_character(&mut self) {
    self.characters.push(Character::default());
  }

  // add new enemy to the game
  pub fn add_enemy(&mut self) {
    self.enemies.push(Enemy::default());
  }

  // add new object to the game
  pub fn add_object(&mut self) {
    self.objects.push(Object::default());
  }

  // handle LMB runtime mode click
  pub fn handle_runtime_mode_click(&mut self, ctx: &mut Context, v: Vec2) {
    if ctx.keyboard.is_mod_active(KeyMods::SHIFT) {
      // TODO: same code for every entity
      for character in self.characters.iter_mut() {
        if character.get_rect().contains::<Point2<f32>>(v.into()) {
          character.is_selected = !character.is_selected
        }
      }

      for enemy in self.enemies.iter_mut() {
        if enemy.get_rect().contains::<Point2<f32>>(v.into()) {
          enemy.is_selected = !enemy.is_selected;
        }
      }

      for object in self.objects.iter_mut() {
        if object.get_rect().contains::<Point2<f32>>(v.into()) {
          object.is_selected = !object.is_selected;
        }
      }
    } else {
      for character in self.characters.iter_mut() {
        if character.is_selected {
          character.path = self.mesh.find_path(character.pos, v);
        }
      }
    }
  }

  // handle LMB edit mode click
  pub fn handle_edit_mode_click(&mut self, ctx: &mut Context, v: Vec2) {
    if ctx.keyboard.is_mod_active(KeyMods::SHIFT) {
      for triangle in self.mesh.triangles.iter_mut() {
        if triangle.contains(v) {
          triangle.is_selected = !triangle.is_selected;
        }
      }
    } else {
      self.mesh.add_vec2(v);
    }
  }
}

impl EventHandler<GameError> for State {
  // main update fn
  fn update(&mut self, ctx: &mut Context) -> GameResult {
    for key in ctx.keyboard.pressed_keys() {
      match key {
        KeyCode::Left => self.offset.x -= OFFSET_SPEED / self.scale.x,
        KeyCode::Right => self.offset.x += OFFSET_SPEED / self.scale.x,
        KeyCode::Up => self.offset.y -= OFFSET_SPEED / self.scale.x,
        KeyCode::Down => self.offset.y += OFFSET_SPEED / self.scale.x,
        _ => (),
      }
    }

    // update characters
    for character in self.characters.iter_mut() {
      character.update();
    }

    // update enemies
    for enemy in self.enemies.iter_mut() {
      enemy.update();
      enemy.pov = self.mesh.get_pov(enemy.pos, enemy.pov_dest); // TODO: move to enemy.update()
    }

    // update objects
    for object in self.objects.iter_mut() {
      object.update(ctx);
    }

    // gui
    self.gui_window_rect = gui::update(self);
    self.gui.update(ctx);

    Ok(())
  }

  // main draw fn
  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    let mut canvas = Canvas::from_frame(ctx, Color::from_rgb(255, 0, 255));

    // draw mesh
    self.mesh.draw(&mut canvas, ctx, &self);

    // draw characters
    for character in &self.characters {
      character.draw(&mut canvas, ctx, &self);
    }

    // draw enemies
    for enemy in &self.enemies {
      enemy.draw(&mut canvas, ctx, &self);
    }

    // draw objects
    for object in &self.objects {
      object.draw(&mut canvas, ctx, &self);
    }

    // draw gui
    canvas.draw(&self.gui, DrawParam::default());

    canvas.finish(ctx)?;

    Ok(())
  }

  fn mouse_button_up_event(
    &mut self,
    ctx: &mut Context,
    btn: MouseButton,
    x: f32,
    y: f32,
  ) -> Result<(), GameError> {
    let v = (Vec2::new(x, y) / self.scale.x) + self.offset;

    if self.mesh.rect.contains::<Point2<f32>>(v.into())
      && self.gui_window_rect.is_some()
      && !self.gui_window_rect.unwrap().contains(pos2(x, y))
    {
      match btn {
        MouseButton::Left => match self.mode {
          Mode::Runtime => self.handle_runtime_mode_click(ctx, v),
          Mode::Edit => self.handle_edit_mode_click(ctx, v),
        },
        _ => {}
      }
    }

    Ok(())
  }

  fn text_input_event(&mut self, _ctx: &mut Context, character: char) -> Result<(), GameError> {
    self.gui.input.text_input_event(character);

    Ok(())
  }
}

fn main() -> GameResult {
  let resource_dir = path::PathBuf::from("./resources");

  let context_builder = ContextBuilder::new("rtte", "rtte")
    .window_setup(ggez::conf::WindowSetup::default().title("rtte"))
    .window_mode(ggez::conf::WindowMode::default().dimensions(800., 600.))
    .add_resource_path(resource_dir);

  let (mut ctx, event_loop) = context_builder.build()?;
  let state = State::new(&mut ctx)?;

  event::run(ctx, event_loop, state)
}
