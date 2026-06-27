#include "entity.h"

Entity::Entity(
    const EntityConfig &entityConfig,
    const EntityTextureConfig &entityTextureConfig,
    const EntityTraceConfig &entityTraceConfig,
    const EntityShapeConfig &entityShapeConfig,
    const EntityMovementConfig &entityMovementConfig)
    : m_entityConfig(entityConfig),
      m_position(entityConfig.defaultPosition),
      m_selected(false),
      m_path({}),
      m_octant(entityConfig.defaultOctant),
      m_entityTextureConfig(entityTextureConfig),
      m_currentTextureFrame(0),
      m_animationTicks(0),
      m_entityTraceConfig(entityTraceConfig),
      m_entityShapeConfig(entityShapeConfig),
      m_entityMovementConfig(entityMovementConfig)
{
  CalculateShapeDimensions();

  if (entityTextureConfig.path != "")
  {
    if (entityTextureConfig.fill)
    {
      CreatePolygonTexture();
    }
    else
    {
      m_texture = LoadTexture(entityTextureConfig.path.c_str());
    }
  }

  if (entityTraceConfig.texturePath != "")
  {
    m_traceTexture = LoadTexture(entityTraceConfig.texturePath.c_str());
  }
}

Entity::~Entity()
{
  if (IsTextureValid(m_texture))
  {
    UnloadTexture(m_texture);
  }

  if (IsTextureValid(m_traceTexture))
  {
    UnloadTexture(m_traceTexture);
  }
}

int Entity::GetId() const
{
  return m_entityConfig.id;
}

int Entity::GetDrawingLayer() const
{
  return m_entityTextureConfig.drawingLayer;
}

float Entity::GetZIndex() const
{
  if (m_entityMovementConfig.movable)
  {
    return m_position.y + (m_texture.height / 8.0 / 2.0);
  }

  return m_position.y + m_texture.height;
}

bool Entity::GetSelected() const
{
  return m_selected;
}

std::vector<Vector2> Entity::GetShape() const
{
  std::vector<Vector2> shape;

  for (size_t i = 0; i < m_entityShapeConfig.points.size(); ++i)
  {
    Vector2 shapePoint = {m_position.x + m_entityShapeConfig.points.at(i).x,
                          m_position.y + m_entityShapeConfig.points.at(i).y};
    shape.emplace_back(shapePoint);
  }

  return shape;
}

Vector2 Entity::GetPosition() const
{
  return m_position;
}

Vector2 Entity::GetShapeCenter() const
{
  Vector2 shapeCenter = {
      m_position.x + m_shapeRectangle.x + (m_shapeRectangle.width / 2),
      m_position.y + m_shapeRectangle.y + (m_shapeRectangle.height / 2),
  };

  return shapeCenter;
}

Rectangle Entity::GetShapeRectangle() const
{
  return m_shapeRectangle;
}

bool Entity::GetShowsTraces() const
{
  return m_entityConfig.showsTraces;
}

bool Entity::GetBlocksMovement() const
{
  return m_entityShapeConfig.blocksMovement;
}

bool Entity::GetMovable() const
{
  return m_entityMovementConfig.movable;
}

bool Entity::IsMoving() const
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
}

void Entity::SetTrace()
{
  if (m_traces.size() == 0 || m_traces.back().ticks >= (60 / m_entityTraceConfig.tracesPerSecond))
  {
    Vector2 shapeCenter = GetShapeCenter();
    Trace trace = {
        shapeCenter,
        (float)(GetAngleBetween(m_path[0], shapeCenter) * (180.0f / M_PI)),
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
  int textureFramesInColumn = m_entityMovementConfig.movable ? 8 : 1;
  float rectX = (float)(m_currentTextureFrame * (m_texture.width / m_entityTextureConfig.framesInRow));
  float rectY = (float)(m_texture.height / textureFramesInColumn * m_octant);
  float rectWidth = (float)(m_texture.width / m_entityTextureConfig.framesInRow);
  float rectHeight = (float)(m_texture.height / textureFramesInColumn);
  Rectangle rectangle = {rectX, rectY, rectWidth, rectHeight};

  DrawTextureRec(m_texture, rectangle, m_position, WHITE);
}

void Entity::DrawPath()
{
  if (m_path.size() > 0)
  {
    DrawLineV(GetShapeCenter(), m_path.at(0), MAGENTA);

    for (size_t i = 0; i < m_path.size() - 1; ++i)
    {
      Vector2 a = m_path.at(i);
      Vector2 b = m_path.at(i + 1);
      DrawLineV(a, b, MAGENTA);
    }
  }
}

void Entity::DrawShape()
{
  std::vector<Vector2> shape = GetShape();

  for (size_t i = 0; i < shape.size(); ++i)
  {
    DrawLineV(shape.at(i),
              shape.at((i + 1) % shape.size()),
              m_selected ? GREEN : MAGENTA);
  }
}

void Entity::CalculateShapeDimensions()
{
  float minX = FLT_MAX;
  float maxX = FLT_MIN;
  float minY = FLT_MAX;
  float maxY = FLT_MIN;

  for (size_t i = 0; i < m_entityShapeConfig.points.size(); ++i)
  {
    if (m_entityShapeConfig.points.at(i).x < minX)
    {
      minX = m_entityShapeConfig.points.at(i).x;
    }

    if (m_entityShapeConfig.points.at(i).x > maxX)
    {
      maxX = m_entityShapeConfig.points.at(i).x;
    }

    if (m_entityShapeConfig.points.at(i).y < minY)
    {
      minY = m_entityShapeConfig.points.at(i).y;
    }

    if (m_entityShapeConfig.points.at(i).y > maxY)
    {
      maxY = m_entityShapeConfig.points.at(i).y;
    }
  }

  m_shapeRectangle = Rectangle{
      minX, minY, maxX - minX, maxY - minY};
}

void Entity::CreatePolygonTexture()
{
  Image sourceImage = LoadImage(m_entityTextureConfig.path.c_str());
  Image targetImage = GenImageColor(m_shapeRectangle.width * m_entityTextureConfig.framesInRow,
                                    m_shapeRectangle.height, BLANK);
  int sourceImageFrameWidth = sourceImage.width / m_entityTextureConfig.framesInRow;

  for (size_t frame = 0; frame < m_entityTextureConfig.framesInRow; ++frame)
  {
    for (size_t x = 0; x < m_shapeRectangle.width; ++x)
    {
      for (size_t y = 0; y < m_shapeRectangle.height; ++y)
      {
        Vector2 pixel = {(float)x, (float)y};

        if (CheckCollisionPointPoly(pixel, &m_entityShapeConfig.points.at(0), m_entityShapeConfig.points.size()))
        {
          Color color = GetImageColor(sourceImage,
                                      (x % sourceImageFrameWidth) + (frame * sourceImageFrameWidth),
                                      y % sourceImage.height);
          ImageDrawPixel(&targetImage, x + (frame * m_shapeRectangle.width), y, color);
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
  Vector2 shapeCenter = GetShapeCenter();

  int dx = m_path.at(0).x - shapeCenter.x;
  int dy = m_path.at(0).y - shapeCenter.y;
  float magnitude = std::sqrt((dx * dx) + (dy * dy));

  if (magnitude != 0)
  {
    m_position = {m_position.x + ((dx / magnitude) * MOVEMENT_SPEED),
                  m_position.y + ((dy / magnitude) * MOVEMENT_SPEED)};
  }

  shapeCenter = GetShapeCenter();

  dx = m_path.at(0).x - shapeCenter.x;
  dy = m_path.at(0).y - shapeCenter.y;
  magnitude = std::sqrt((dx * dx) + (dy * dy));

  if (magnitude < MOVEMENT_SPEED / 2)
  {
    m_path.erase(m_path.begin());

    if (m_path.size() > 0)
    {
      m_octant = GetOctantFrom(-GetAngleBetween(m_path.at(0), shapeCenter));
    }
  }
}

void Entity::RemoveOldTraces()
{
  m_traces.erase(std::remove_if(m_traces.begin(), m_traces.end(),
                                [this](Trace trace)
                                { return trace.ticks >= TRACE_VISIBILITY_TICKS_COUNT; }),
                 m_traces.end());
}
