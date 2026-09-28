#include "sprite.h"

Sprite::Sprite(const std::string &path, Vector3 position)
    : m_path(path),
      m_position(position),
      m_size({0.0, 0.0}),
      m_shape({}),
      m_texture(LoadTexture(path.c_str())),
      m_animation({1, {(float)m_texture.width, (float)m_texture.height}, 0.0f})
{
}

Sprite::Sprite(const std::string &path, Vector3 position, Animation animation)
    : m_path(path),
      m_position(position),
      m_size({0.0, 0.0}),
      m_shape({}),
      m_texture(LoadTexture(path.c_str())),
      m_animation(animation)
{
}

Sprite::Sprite(const std::string &path, Vector3 position, Vector2 size)
    : m_path(path),
      m_position(position),
      m_size(size),
      m_shape({}),
      m_animation({1, size, 0.0f})
{
  Image src = LoadImage(path.c_str());
  Image dest = GenImageColor(size.x, size.y, BLANK);

  for (size_t x = 0; x < size.x; ++x)
  {
    for (size_t y = 0; y < size.y; ++y)
    {
      Color color = GetImageColor(src, x % src.width, y % src.height);
      ImageDrawPixel(&dest, x, y, color);
    }
  }

  m_texture = LoadTextureFromImage(dest);
  UnloadImage(src);
  UnloadImage(dest);
}

Sprite::Sprite(const std::string &path, Vector3 position, Vector2 size, Animation animation)
    : m_path(path),
      m_position(position),
      m_size(size),
      m_shape({}),
      m_animation(animation)
{
  size_t frameCount = animation.GetFrameCount();
  Vector2 frameSize = animation.GetFrameSize();
  Image src = LoadImage(path.c_str());
  Image dest = GenImageColor(frameCount * size.x, size.y, BLANK);

  for (size_t frame = 0; frame < frameCount; ++frame)
  {
    for (size_t x = 0; x < size.x; ++x)
    {
      for (size_t y = 0; y < size.y; ++y)
      {
        Color color = GetImageColor(
            src,
            frame * frameSize.x + x % (size_t)frameSize.x,
            y % (size_t)frameSize.y);
        ImageDrawPixel(&dest, frame * size.x + x, y, color);
      }
    }
  }

  m_texture = LoadTextureFromImage(dest);
  m_animation.SetFrameSize(size);
  UnloadImage(src);
  UnloadImage(dest);
}

Sprite::Sprite(const std::string &path, Vector3 position, const std::vector<Vector2> shape)
    : m_path(path),
      m_position(position),
      m_size({0.0, 0.0}),
      m_shape(shape),
      m_animation({1, {0.0, 0.0}, 0.0f})
{
  Vector2 topLeft = {
      std::numeric_limits<float>::infinity(),
      std::numeric_limits<float>::infinity()};
  Vector2 bottomRight = {
      -std::numeric_limits<float>::infinity(),
      -std::numeric_limits<float>::infinity()};

  for (auto const &vec : shape)
  {
    if (vec.x < topLeft.x)
    {
      topLeft.x = vec.x;
    }

    if (vec.y < topLeft.y)
    {
      topLeft.y = vec.y;
    }

    if (vec.x > bottomRight.x)
    {
      bottomRight.x = vec.x;
    }

    if (vec.y > bottomRight.y)
    {
      bottomRight.y = vec.y;
    }
  }

  float width = bottomRight.x - topLeft.x;
  float height = bottomRight.y - topLeft.y;

  Image src = LoadImage(path.c_str());
  Image dest = GenImageColor(width, height, BLANK);

  for (size_t x = 0; x < width; ++x)
  {
    for (size_t y = 0; y < height; ++y)
    {
      Vector2 vec = {(float)x, (float)y};
      if (CheckCollisionPointPoly(vec, &shape.at(0), shape.size()))
      {
        Color color = GetImageColor(src, x % src.width, y % src.height);
        ImageDrawPixel(&dest, x, y, color);
      }
    }
  }

  m_texture = LoadTextureFromImage(dest);
  m_animation.SetFrameSize({(float)m_texture.width, (float)m_texture.height});
  UnloadImage(src);
  UnloadImage(dest);
}

Sprite::Sprite(const std::string &path, Vector3 position, const std::vector<Vector2> shape, Animation animation)
    : m_path(path),
      m_position(position),
      m_size({0.0, 0.0}),
      m_shape(shape),
      m_animation(animation)
{
  Vector2 topLeft = {
      std::numeric_limits<float>::infinity(),
      std::numeric_limits<float>::infinity()};
  Vector2 bottomRight = {
      -std::numeric_limits<float>::infinity(),
      -std::numeric_limits<float>::infinity()};

  for (auto const &vec : shape)
  {
    if (vec.x < topLeft.x)
    {
      topLeft.x = vec.x;
    }

    if (vec.y < topLeft.y)
    {
      topLeft.y = vec.y;
    }

    if (vec.x > bottomRight.x)
    {
      bottomRight.x = vec.x;
    }

    if (vec.y > bottomRight.y)
    {
      bottomRight.y = vec.y;
    }
  }

  float width = bottomRight.x - topLeft.x;
  float height = bottomRight.y - topLeft.y;

  size_t frameCount = animation.GetFrameCount();
  Vector2 frameSize = animation.GetFrameSize();
  Image src = LoadImage(path.c_str());
  Image dest = GenImageColor(frameCount * width, height, BLANK);

  for (size_t frame = 0; frame < frameCount; ++frame)
  {
    for (size_t x = 0; x < width; ++x)
    {
      for (size_t y = 0; y < height; ++y)
      {
        Vector2 vec = {(float)x, (float)y};
        if (CheckCollisionPointPoly(vec, &shape.at(0), shape.size()))
        {
          Color color = GetImageColor(
              src,
              (frame * frameSize.x) + (x % (size_t)frameSize.x),
              y % (size_t)frameSize.y);
          ImageDrawPixel(&dest, x + (frame * width), y, color);
        }
      }
    }
  }

  m_texture = LoadTextureFromImage(dest);
  m_animation.SetFrameSize({width, height});
  UnloadImage(src);
  UnloadImage(dest);
}

Sprite::~Sprite()
{
  UnloadTexture(m_texture);
}

void Sprite::Update(float dt)
{
  m_animation.Update(dt);
}

void Sprite::Render()
{
  Vector2 position = {m_position.x, m_position.y};
  Vector2 frameSize = m_animation.GetFrameSize();
  size_t currentFrame = m_animation.GetCurrentFrame();
  Rectangle rect = {
      currentFrame * frameSize.x,
      0.0f,
      frameSize.x,
      frameSize.y,
  };

  DrawTextureRec(m_texture, rect, position, WHITE);
}

const Texture2D &Sprite::GetTexture() const
{
  return m_texture;
}
