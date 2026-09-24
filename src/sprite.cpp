#include "sprite.h"

Sprite::Sprite(const std::string &path, Vector3 position)
    : m_path(path),
      m_position(position),
      m_dimensions({0.0, 0.0}),
      m_shape({}),
      m_texture(LoadTexture(path.c_str()))
{
}

Sprite::Sprite(const std::string &path, Vector3 position, Vector2 dimensions)
    : m_path(path),
      m_position(position),
      m_dimensions(dimensions),
      m_shape({})
{
  Image src = LoadImage(path.c_str());
  Image dest = GenImageColor(dimensions.x, dimensions.y, BLANK);

  for (size_t x = 0; x < dimensions.x; ++x)
  {
    for (size_t y = 0; y < dimensions.y; ++y)
    {
      Color color = GetImageColor(src, x % src.width, y % src.height);
      ImageDrawPixel(&dest, x, y, color);
    }
  }

  m_texture = LoadTextureFromImage(dest);
  UnloadImage(src);
  UnloadImage(dest);
}

Sprite::Sprite(const std::string &path, Vector3 position, const std::vector<Vector2> shape)
    : m_path(path),
      m_position(position),
      m_dimensions({0.0, 0.0}),
      m_shape(shape)
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
  UnloadImage(src);
  UnloadImage(dest);
}

Sprite::~Sprite()
{
  UnloadTexture(m_texture);
}

void Sprite::Render()
{
  Vector2 position = {m_position.x, m_position.y};
  DrawTextureV(m_texture, position, WHITE);
}

const Texture2D &Sprite::GetTexture() const
{
  return m_texture;
}
