#include "entity.h"

Entity::Entity(
    int id,
    Vector2 position,
    int layerIndex,
    const std::vector<Vector2> &polygon,
    const std::string &texturePath,
    int textureFramesInRow,
    int textureFramesPerSecond,
    bool showsTraces,
    const std::string &traceTexturePath,
    int traceTicksToLive,
    int traceTracesPerSecond) : m_id(id),
                                m_position(position),
                                m_selected(false),
                                m_layerIndex(layerIndex),
                                m_polygon(polygon),
                                m_textureFramesInRow(textureFramesInRow),
                                m_textureFramesPerSecond(textureFramesPerSecond),
                                m_currentTextureFrame(0),
                                m_animationTicks(0),
                                m_path({}),
                                m_showsTraces(showsTraces),
                                m_traceTicksToLive(traceTicksToLive),
                                m_traceTracesPerSecond(traceTracesPerSecond),
                                m_textureRectangleY(0.0)
{
  if (layerIndex == 0)
  {
    CreatePolygonTexture(texturePath);
  }
  else
  {
    m_texture = LoadTexture(texturePath.c_str());
  }

  if (traceTexturePath != "")
  {
    m_traceTexture = LoadTexture(traceTexturePath.c_str());
  }
}

Entity::~Entity()
{
  UnloadTexture(m_texture);

  if (IsTextureValid(m_traceTexture))
  {
    UnloadTexture(m_traceTexture);
  }
}

int Entity::GetId()
{
  return m_id;
}

int Entity::GetLayerIndex()
{
  return m_layerIndex;
}

float Entity::GetZIndex()
{
  return m_position.y + (m_texture.height / 2.0);
}

bool Entity::GetSelected()
{
  return m_selected;
}

std::vector<Vector2> Entity::GetPolygon()
{
  std::vector<Vector2> polygon;

  for (int i = 0; i < m_polygon.size(); i++)
  {
    Vector2 polygonPoint = {m_position.x + m_polygon.at(i).x,
                            m_position.y + m_polygon.at(i).y};
    polygon.emplace_back(polygonPoint);
  }

  return polygon;
}

Vector2 Entity::GetPosition()
{
  return m_position;
}

bool Entity::GetShowsTraces()
{
  return m_showsTraces;
}

bool Entity::IsMoving()
{
  return m_path.size() > 0;
}

void Entity::SetSelected(bool selected)
{
  m_selected = selected;
}

void Entity::SetPath(const std::vector<Vector2> &path)
{
  m_path = path;

  // set correct octant sprite frame
  int textureFramesInColumn = m_layerIndex == 0 ? 1 : 8;
  float angle = GetAngleBetween(path[0], m_position);
  Octant octant = GetOctantFrom(-angle);
  m_textureRectangleY = (m_texture.height / (float)textureFramesInColumn) * (float)(octant);
}

void Entity::SetTrace()
{
  if (m_traces.size() == 0 || m_traces.back().ticks >= (60 / m_traceTracesPerSecond))
  {
    float degrees = GetAngleBetween(m_path[0], m_position) * (180.0f / M_PI);

    Trace trace = {
        m_position,
        degrees,
        0,
    };

    m_traces.emplace_back(trace);
  }
}

void Entity::Update()
{
  if (m_path.size() > 0)
  {
    HandleMovement();
  }

  if (m_textureFramesInRow > 1)
  {
    HandleAnimation();
  }

  for (auto &trace : m_traces)
  {
    trace.ticks += 1;
  }

  m_traces.erase(std::remove_if(m_traces.begin(), m_traces.end(), [this](Trace trace)
                                { return trace.ticks >= m_traceTicksToLive; }),
                 m_traces.end());
}

void Entity::Draw()
{
  // draw traces
  float textureWidth = (float)m_traceTexture.width;
  float textureHeight = (float)m_traceTexture.height;

  for (const auto &trace : m_traces)
  {
    DrawTexturePro(m_traceTexture,
                   {0, 0, textureWidth, textureHeight},
                   {trace.position.x, trace.position.y, textureWidth, textureHeight},
                   {textureWidth / 2, textureHeight / 2},
                   trace.rotation,
                   WHITE);
  }

  // draw texture
  int textureFramesInColumn = m_layerIndex == 0 ? 1 : 8;
  float rectX = (float)(m_currentTextureFrame * (m_texture.width / m_textureFramesInRow));
  float rectWidth = (float)(m_texture.width / m_textureFramesInRow);
  float rectHeight = (float)(m_texture.height / textureFramesInColumn);

  Rectangle rectangle = {rectX, m_textureRectangleY, rectWidth, rectHeight};
  Vector2 position = {m_position.x - (m_layerIndex == 0 ? 0 : rectWidth / 2),
                      m_position.y - (m_layerIndex == 0 ? 0 : rectHeight / 2)};

  DrawTextureRec(m_texture, rectangle, position, WHITE);

  // draw polygon
  std::vector<Vector2> polygon = GetPolygon();

  for (int i = 0; i < polygon.size(); i++)
  {
    DrawLineV(polygon.at(i),
              polygon.at((i + 1) % polygon.size()),
              m_selected ? GREEN : WHITE);
  }
}

void Entity::CreatePolygonTexture(const std::string &texturePath)
{
  int minX = INT_MAX;
  int maxX = INT_MIN;
  int minY = INT_MAX;
  int maxY = INT_MIN;

  for (int i = 0; i < m_polygon.size(); i++)
  {
    if (m_polygon.at(i).x < minX)
    {
      minX = m_polygon.at(i).x;
    }

    if (m_polygon.at(i).x > maxX)
    {
      maxX = m_polygon.at(i).x;
    }

    if (m_polygon.at(i).y < minY)
    {
      minY = m_polygon.at(i).y;
    }

    if (m_polygon.at(i).y > maxY)
    {
      maxY = m_polygon.at(i).y;
    }
  }

  int targetImageFrameWidth = maxX - minX;
  int height = maxY - minY;

  Image sourceImage = LoadImage(texturePath.c_str());
  Image targetImage = GenImageColor(targetImageFrameWidth * m_textureFramesInRow, height, BLANK);
  int sourceImageFrameWidth = sourceImage.width / m_textureFramesInRow;

  for (int frame = 0; frame < m_textureFramesInRow; frame++)
  {
    for (int x = 0; x < targetImageFrameWidth; x++)
    {
      for (int y = 0; y < height; y++)
      {
        Vector2 pixel = {(float)x, (float)y};

        if (CheckCollisionPointPoly(pixel, &m_polygon[0], m_polygon.size()))
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

void Entity::HandleAnimation()
{
  m_animationTicks++;

  if (m_animationTicks >= (60 / m_textureFramesPerSecond))
  {
    m_animationTicks = 0;
    m_currentTextureFrame++;

    if (m_currentTextureFrame >= m_textureFramesInRow)
    {
      m_currentTextureFrame = 0;
    }
  }
}

void Entity::HandleMovement()
{
  int dx = m_path.at(0).x - m_position.x;
  int dy = m_path.at(0).y - m_position.y;
  float magnitude = std::sqrt((dx * dx) + (dy * dy));

  if (magnitude != 0)
  {
    m_position = {m_position.x + ((dx / magnitude) * MOVEMENT_SPEED),
                  m_position.y + ((dy / magnitude) * MOVEMENT_SPEED)};
  }

  dx = m_path.at(0).x - m_position.x;
  dy = m_path.at(0).y - m_position.y;
  magnitude = std::sqrt((dx * dx) + (dy * dy));

  if (magnitude < MOVEMENT_SPEED / 2)
  {
    m_path.clear();
  }
}
