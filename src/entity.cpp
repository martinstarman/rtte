#include "entity.h"

Entity::Entity(
    const Config &config,
    const TextureConfig &textureConfig,
    const TraceConfig &traceConfig)
    : m_config(config),
      m_position(config.position),
      m_selected(false),
      m_path({}),
      m_octant(Octant::East),
      m_textureConfig(textureConfig),
      m_currentTextureFrame(0),
      m_animationTicks(0),
      m_traceConfig(traceConfig)
{
  if (config.layerIndex == 0)
  {
    CreatePolygonTexture();
  }
  else
  {
    m_texture = LoadTexture(textureConfig.path.c_str());
  }

  if (traceConfig.path != "")
  {
    m_traceTexture = LoadTexture(traceConfig.path.c_str());
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
  return m_config.id;
}

int Entity::GetLayerIndex()
{
  return m_config.layerIndex;
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

  for (int i = 0; i < m_config.polygon.size(); i++)
  {
    Vector2 polygonPoint = {m_position.x + m_config.polygon.at(i).x,
                            m_position.y + m_config.polygon.at(i).y};
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
  return m_config.showsTraces;
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
  if (m_traces.size() == 0 || m_traces.back().ticks >= (60 / m_traceConfig.tracesPerSecond))
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

  if (m_textureConfig.framesInRow > 1)
  {
    HandleAnimation();
  }

  for (auto &trace : m_traces)
  {
    trace.ticks += 1;
  }

  m_traces.erase(std::remove_if(m_traces.begin(), m_traces.end(), [this](Trace trace)
                                { return trace.ticks >= m_traceConfig.ticksToLive; }),
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
  int textureFramesInColumn = m_config.layerIndex == 0 ? 1 : 8;
  float rectX = (float)(m_currentTextureFrame * (m_texture.width / m_textureConfig.framesInRow));
  float rectY = (float)(m_texture.height / textureFramesInColumn * m_octant);
  float rectWidth = (float)(m_texture.width / m_textureConfig.framesInRow);
  float rectHeight = (float)(m_texture.height / textureFramesInColumn);
  Rectangle rectangle = {rectX, rectY, rectWidth, rectHeight};
  Vector2 position = {m_position.x - (m_config.layerIndex == 0 ? 0 : rectWidth / 2),
                      m_position.y - (m_config.layerIndex == 0 ? 0 : rectHeight / 2)};

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

void Entity::CreatePolygonTexture()
{
  int minX = INT_MAX;
  int maxX = INT_MIN;
  int minY = INT_MAX;
  int maxY = INT_MIN;

  for (int i = 0; i < m_config.polygon.size(); i++)
  {
    if (m_config.polygon.at(i).x < minX)
    {
      minX = m_config.polygon.at(i).x;
    }

    if (m_config.polygon.at(i).x > maxX)
    {
      maxX = m_config.polygon.at(i).x;
    }

    if (m_config.polygon.at(i).y < minY)
    {
      minY = m_config.polygon.at(i).y;
    }

    if (m_config.polygon.at(i).y > maxY)
    {
      maxY = m_config.polygon.at(i).y;
    }
  }

  int targetImageFrameWidth = maxX - minX;
  int height = maxY - minY;

  Image sourceImage = LoadImage(m_textureConfig.path.c_str());
  Image targetImage = GenImageColor(targetImageFrameWidth * m_textureConfig.framesInRow, height, BLANK);
  int sourceImageFrameWidth = sourceImage.width / m_textureConfig.framesInRow;

  for (int frame = 0; frame < m_textureConfig.framesInRow; frame++)
  {
    for (int x = 0; x < targetImageFrameWidth; x++)
    {
      for (int y = 0; y < height; y++)
      {
        Vector2 pixel = {(float)x, (float)y};

        if (CheckCollisionPointPoly(pixel, &m_config.polygon[0], m_config.polygon.size()))
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

  if (m_animationTicks >= (60 / m_textureConfig.framesPerSecond))
  {
    m_animationTicks = 0;
    m_currentTextureFrame++;

    if (m_currentTextureFrame >= m_textureConfig.framesInRow)
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
