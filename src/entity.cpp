#include "entity.h"

Entity::Entity(
    std::tuple<int, int> position,
    std::tuple<int, int> size,
    int layerIndex,
    const std::vector<std::tuple<int, int>> &polygon,
    const std::string &texturePath,
    TextureTransformation textureTransformation)
    : m_position(position),
      m_size(size),
      m_layerIndex(layerIndex),
      m_polygon(polygon)
{
  if (textureTransformation == TextureTransformation::None)
  {
    m_texture = LoadTexture(texturePath.c_str());
  }
  else
  {
    CreatePolygonTexture(texturePath);
  }
}

Entity::~Entity()
{
  UnloadTexture(m_texture);
}

int Entity::LayerIndex()
{
  return m_layerIndex;
}

int Entity::ZIndex()
{
  return std::get<1>(m_position) + std::get<1>(m_size);
}

void Entity::Draw()
{
  int x = std::get<0>(m_position);
  int y = std::get<1>(m_position);

  DrawTexture(m_texture, x, y, WHITE);

  for (int i = 0; i < m_polygon.size(); i++)
  {
    DrawLine(
        x + std::get<0>(m_polygon.at(i)),
        y + std::get<1>(m_polygon.at(i)),
        x + std::get<0>(m_polygon.at((i + 1) % m_polygon.size())),
        y + std::get<1>(m_polygon.at((i + 1) % m_polygon.size())),
        WHITE);
  }
}

void Entity::CreatePolygonTexture(const std::string &texturePath)
{
  int minX = INT_MAX;
  int maxX = INT_MIN;
  int minY = INT_MAX;
  int maxY = INT_MIN;
  std::vector<Vector2> points;

  for (int i = 0; i < m_polygon.size(); i++)
  {
    int x = std::get<0>(m_polygon.at(i));
    int y = std::get<1>(m_polygon.at(i));

    Vector2 point{(float)x, (float)y};
    points.emplace_back(point);

    if (x < minX)
    {
      minX = x;
    }

    if (x > maxX)
    {
      maxX = x;
    }

    if (y < minY)
    {
      minY = y;
    }

    if (y > maxY)
    {
      maxY = y;
    }
  }

  int width = maxX - minX;
  int height = maxY - minY;

  Image sourceImage = LoadImage(texturePath.c_str());
  Image targetImage = GenImageColor(width, height, BLANK);

  for (int x = 0; x < width; x++)
  {
    for (int y = 0; y < height; y++)
    {
      Vector2 point{(float)x, (float)y};

      if (CheckCollisionPointPoly(point, &points[0], points.size()))
      {
        Color color = GetImageColor(sourceImage, x % sourceImage.width, y % sourceImage.height);
        ImageDrawPixel(&targetImage, x, y, color);
      }
    }
  }

  m_texture = LoadTextureFromImage(targetImage);
  UnloadImage(sourceImage);
  UnloadImage(targetImage);
}
