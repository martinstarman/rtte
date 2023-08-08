use ggez::{graphics::Image, Context};

#[derive(Clone)]
pub struct Resource {
  /// image path
  pub path: String,

  /// image
  pub image: Image,

  /// image width
  pub w: f32,

  /// image height
  pub h: f32,
}

impl Resource {
  pub fn new(path: String, ctx: &mut Context) -> Self {
    let image = Image::from_path(ctx, path.clone()).unwrap();
    let w = image.width() as f32;
    let h = image.height() as f32;

    Resource { path, image, w, h }
  }
}
