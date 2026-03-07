#include "entity.h"

Entity::Entity(
    std::tuple<int, int> position,
    std::tuple<int, int> size,
    int layerIndex,
    const std::vector<std::tuple<int, int>> &polygon,
    bool selectable,
    const std::string &texturePath,
    TextureTransformation textureTransformation,
    int textureFrames,
    int textureFramesPerSecond)
    : m_position(position),
      m_selectable(selectable),
      m_selected(false),
      m_size(size),
      m_layerIndex(layerIndex),
      m_polygon(polygon),
      m_textureFrames(textureFrames),
      m_textureFramesPerSecond(textureFramesPerSecond),
      m_textureFrame(0),
      m_frames(0)
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

void Entity::Update()
{
  if (m_textureFrames > 1)
  {
    Animate();
  }
}

void Entity::Draw()
{
  Rectangle rectangle = {
      (float)(m_textureFrame * (m_texture.width / m_textureFrames)),
      0.0f,
      (float)(m_texture.width / m_textureFrames),
      (float)m_texture.height};

  int x = std::get<0>(m_position);
  int y = std::get<1>(m_position);
  Vector2 position = {(float)x, (float)y};

  DrawTextureRec(m_texture, rectangle, position, WHITE);

  for (int i = 0; i < m_polygon.size(); i++)
  {
    DrawLine(
        x + std::get<0>(m_polygon.at(i)),
        y + std::get<1>(m_polygon.at(i)),
        x + std::get<0>(m_polygon.at((i + 1) % m_polygon.size())),
        y + std::get<1>(m_polygon.at((i + 1) % m_polygon.size())),
        m_selected ? GREEN : WHITE);
  }
}

bool Entity::Selectable()
{
  return m_selectable;
}

void Entity::Selected(bool selected)
{
  m_selected = selected;
}

std::vector<Vector2> Entity::Polygon()
{
  std::vector<Vector2> points;
  int x = std::get<0>(m_position);
  int y = std::get<1>(m_position);

  for (int i = 0; i < m_polygon.size(); i++)
  {
    int pX = std::get<0>(m_polygon.at(i));
    int pY = std::get<1>(m_polygon.at(i));
    Vector2 point = {(float)(x + pX), (float)(y + pY)};
    points.emplace_back(point);
  }

  return points;
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
    int pX = std::get<0>(m_polygon.at(i));
    int pY = std::get<1>(m_polygon.at(i));
    Vector2 polygonPoint = {(float)pX, (float)pY};

    points.emplace_back(polygonPoint);

    if (pX < minX)
    {
      minX = pX;
    }

    if (pX > maxX)
    {
      maxX = pX;
    }

    if (pY < minY)
    {
      minY = pY;
    }

    if (pY > maxY)
    {
      maxY = pY;
    }
  }

  int targetImageFrameWidth = maxX - minX;
  int height = maxY - minY;

  Image sourceImage = LoadImage(texturePath.c_str());
  Image targetImage = GenImageColor(targetImageFrameWidth * m_textureFrames, height, BLANK);
  int sourceImageFrameWidth = sourceImage.width / m_textureFrames;

  for (int frame = 0; frame < m_textureFrames; frame++)
  {
    for (int x = 0; x < targetImageFrameWidth; x++)
    {
      for (int y = 0; y < height; y++)
      {
        Vector2 point = {(float)x, (float)y};

        if (CheckCollisionPointPoly(point, &points[0], points.size()))
        {
          Color color = GetImageColor(sourceImage,
                                      (x % sourceImageFrameWidth) + (frame * sourceImageFrameWidth),
                                      y % sourceImage.height);
          ImageDrawPixel(&targetImage, x + (frame * targetImageFrameWidth), y, color);
        }
      }
    }
  }

  m_texture = LoadTextureFromImage(targetImage);
  UnloadImage(sourceImage);
  UnloadImage(targetImage);
}

void Entity::Animate()
{
  m_frames++;

  if (m_frames >= (60 / m_textureFramesPerSecond))
  {
    m_frames = 0;
    m_textureFrame++;

    if (m_textureFrame >= m_textureFrames)
    {
      m_textureFrame = 0;
    }
  }
}
