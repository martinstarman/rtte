#include "sprite.h"

Sprite::Sprite(const std::string &path, Vector3 position, Vector2 size)
    : Drawable(position, size),
      m_path(path),
      m_texture(LoadTexture(path.c_str())),
      m_animation({1, size, 0})
{
  SetTextureWrap(m_texture, TEXTURE_WRAP_REPEAT);
}

Sprite::Sprite(const std::string &path, Vector3 position, Vector2 size, Animation animation)
    : Drawable(position, size),
      m_path(path),
      m_animation(animation)
{
  size_t frameCount = animation.GetFrameCount();
  Vector2 frameSize = animation.GetFrameSize();
  Image src = LoadImage(path.c_str());
  Image dest = GenImageColor(frameCount * size.x, size.y, BLANK);
  ImageFormat(&src, dest.format);

  for (size_t frame = 0; frame < frameCount; ++frame)
  {
    for (float y = 0.0f; y < size.y; y += frameSize.y)
    {
      float h = std::min(frameSize.y, size.y - y);

      for (float x = 0.0f; x < size.x; x += frameSize.x)
      {
        float w = std::min(frameSize.x, size.x - x);
        Rectangle srcRect = {frame * frameSize.x, 0.0f, w, h};
        Rectangle destRect = {frame * size.x + x, y, w, h};
        ImageDraw(&dest, src, srcRect, destRect, WHITE);
      }
    }
  }

  m_texture = LoadTextureFromImage(dest);

  UnloadImage(src);
  UnloadImage(dest);
}

Sprite::~Sprite()
{
  if (IsTextureValid(m_texture))
  {
    UnloadTexture(m_texture);
  }
}

void Sprite::Update(float dt)
{
  m_animation.Update(dt);
}

void Sprite::Render() const
{
  Vector2 position = {m_position.x, m_position.y};
  Vector2 frameSize = m_animation.GetFrameSize();
  size_t currentFrame = m_animation.GetCurrentFrame();
  Rectangle rect = {
      currentFrame * m_size.x,
      0.0f,
      m_size.x,
      m_size.y,
  };

  DrawTextureRec(m_texture, rect, position, WHITE);
}
