#include "entity.h"

Entity::Entity(
    const EntityConfig &entityConfig,
    const EntityTextureConfig &entityTextureConfig,
    const TraceTextureConfig &traceTextureConfig)
    : m_entityConfig(entityConfig),
      m_position(entityConfig.defaultPosition),
      m_selected(false),
      m_path({}),
      m_octant(entityConfig.defaultOctant),
      m_entityTextureConfig(entityTextureConfig),
      m_currentTextureFrame(0),
      m_animationTicks(0),
      m_traceTextureConfig(traceTextureConfig)
{
  if (entityConfig.drawingLayer == 0)
  {
    CreatePolygonTexture();
  }
  else
  {
    m_texture = LoadTexture(entityTextureConfig.path.c_str());
  }

  if (traceTextureConfig.path != "")
  {
    m_traceTexture = LoadTexture(traceTextureConfig.path.c_str());
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
  return m_entityConfig.id;
}

int Entity::GetDrawingLayer()
{
  return m_entityConfig.drawingLayer;
}

float Entity::GetZIndex()
{
  return m_position.y + (m_texture.height / 2.0);
}

bool Entity::GetSelected()
{
  return m_selected;
}

std::vector<Vector2> Entity::GetShape()
{
  std::vector<Vector2> shape;

  for (int i = 0; i < m_entityConfig.shape.size(); i++)
  {
    Vector2 shapePoint = {m_position.x + m_entityConfig.shape.at(i).x,
                          m_position.y + m_entityConfig.shape.at(i).y};
    shape.emplace_back(shapePoint);
  }

  return shape;
}

Vector2 Entity::GetPosition()
{
  return m_position;
}

bool Entity::GetShowsTraces()
{
  return m_entityConfig.showsTraces;
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
  m_octant = GetOctantFrom(-GetAngleBetween(path[0], m_position));
}

void Entity::SetTrace()
{
  if (m_traces.size() == 0 || m_traces.back().ticks >= (60 / m_traceTextureConfig.tracesPerSecond))
  {
    Trace trace = {
        m_position,
        (float)(GetAngleBetween(m_path[0], m_position) * (180.0f / M_PI)),
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

  if (m_entityTextureConfig.framesInRow > 1)
  {
    HandleAnimation();
  }

  for (auto &trace : m_traces)
  {
    trace.ticks += 1;
  }

  RemoveOldTraces();
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
  int textureFramesInColumn = m_entityConfig.drawingLayer == 0 ? 1 : 8;
  float rectX = (float)(m_currentTextureFrame * (m_texture.width / m_entityTextureConfig.framesInRow));
  float rectY = (float)(m_texture.height / textureFramesInColumn * m_octant);
  float rectWidth = (float)(m_texture.width / m_entityTextureConfig.framesInRow);
  float rectHeight = (float)(m_texture.height / textureFramesInColumn);
  Rectangle rectangle = {rectX, rectY, rectWidth, rectHeight};
  Vector2 position = {m_position.x - (m_entityConfig.drawingLayer == 0 ? 0 : rectWidth / 2),
                      m_position.y - (m_entityConfig.drawingLayer == 0 ? 0 : rectHeight / 2)};

  DrawTextureRec(m_texture, rectangle, position, WHITE);

  // draw shape
  std::vector<Vector2> shape = GetShape();

  for (int i = 0; i < shape.size(); i++)
  {
    DrawLineV(shape.at(i),
              shape.at((i + 1) % shape.size()),
              m_selected ? GREEN : WHITE);
  }
}

void Entity::CreatePolygonTexture()
{
  int minX = INT_MAX;
  int maxX = INT_MIN;
  int minY = INT_MAX;
  int maxY = INT_MIN;

  for (int i = 0; i < m_entityConfig.shape.size(); i++)
  {
    if (m_entityConfig.shape.at(i).x < minX)
    {
      minX = m_entityConfig.shape.at(i).x;
    }

    if (m_entityConfig.shape.at(i).x > maxX)
    {
      maxX = m_entityConfig.shape.at(i).x;
    }

    if (m_entityConfig.shape.at(i).y < minY)
    {
      minY = m_entityConfig.shape.at(i).y;
    }

    if (m_entityConfig.shape.at(i).y > maxY)
    {
      maxY = m_entityConfig.shape.at(i).y;
    }
  }

  int targetImageFrameWidth = maxX - minX;
  int height = maxY - minY;

  Image sourceImage = LoadImage(m_entityTextureConfig.path.c_str());
  Image targetImage = GenImageColor(targetImageFrameWidth * m_entityTextureConfig.framesInRow, height, BLANK);
  int sourceImageFrameWidth = sourceImage.width / m_entityTextureConfig.framesInRow;

  for (int frame = 0; frame < m_entityTextureConfig.framesInRow; frame++)
  {
    for (int x = 0; x < targetImageFrameWidth; x++)
    {
      for (int y = 0; y < height; y++)
      {
        Vector2 pixel = {(float)x, (float)y};

        if (CheckCollisionPointPoly(pixel, &m_entityConfig.shape[0], m_entityConfig.shape.size()))
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

  if (m_animationTicks >= (60 / m_entityTextureConfig.framesPerSecond))
  {
    m_animationTicks = 0;
    m_currentTextureFrame++;

    if (m_currentTextureFrame >= m_entityTextureConfig.framesInRow)
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

void Entity::RemoveOldTraces()
{
  m_traces.erase(std::remove_if(m_traces.begin(), m_traces.end(),
                                [this](Trace trace)
                                { return trace.ticks >= TRACE_VISIBILITY_TICKS_COUNT; }),
                 m_traces.end());
}
