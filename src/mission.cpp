#include "mission.h"

Mission::Mission(const std::string &path)
    : Mission(path, toml::parse(path))
{
}

Mission::Mission(const std::string &path, const toml::value &data)
    : m_path(path),
      m_mapSize(ParseMapSize(data)),
      m_camera({{0.0, 0.0}, {0.0, 0.0}, 0.0f, 1.0f}),
      m_scrollSpeed(500.0f),
      m_navmesh({0.0, 0.0, m_mapSize.x, m_mapSize.y}, ParseHoles(data), 8.0f),
      m_pathStart({0.0, 0.0}),
      m_walkPath({})
{
}

void Mission::Update(float dt)
{
  Vector2 direction = {0.0, 0.0};

  if (IsKeyDown(KEY_LEFT))
  {
    direction.x -= 1.0f;
  }

  if (IsKeyDown(KEY_RIGHT))
  {
    direction.x += 1.0f;
  }

  if (IsKeyDown(KEY_UP))
  {
    direction.y -= 1.0f;
  }

  if (IsKeyDown(KEY_DOWN))
  {
    direction.y += 1.0f;
  }

  float maxX = std::max(0.0f, m_mapSize.x - GetScreenWidth());
  float maxY = std::max(0.0f, m_mapSize.y - GetScreenHeight());

  m_camera.target.x = std::clamp(m_camera.target.x + direction.x * m_scrollSpeed * dt, 0.0f, maxX);
  m_camera.target.y = std::clamp(m_camera.target.y + direction.y * m_scrollSpeed * dt, 0.0f, maxY);

  Vector2 mouse = GetScreenToWorld2D(GetMousePosition(), m_camera);

  if (IsMouseButtonPressed(MOUSE_BUTTON_LEFT))
  {
    m_pathStart = mouse;
  }

  m_walkPath = m_navmesh.FindPath(m_pathStart, mouse);
}

void Mission::Render()
{
  BeginMode2D(m_camera);

  DrawRectangleV({0.0, 0.0}, m_mapSize, BLACK);

  for (float x = 0.0f; x <= m_mapSize.x; x += 100.0f)
  {
    DrawLineV({x, 0.0}, {x, m_mapSize.y}, GRAY);
  }

  for (float y = 0.0f; y <= m_mapSize.y; y += 100.0f)
  {
    DrawLineV({0.0, y}, {m_mapSize.x, y}, GRAY);
  }

  m_navmesh.Render();

  for (size_t i = 1; i < m_walkPath.size(); ++i)
  {
    DrawLineEx(m_walkPath.at(i - 1), m_walkPath.at(i), 2.0f, RED);
  }

  DrawCircleV(m_pathStart, 4.0f, GREEN);

  EndMode2D();
}

Vector2 Mission::GetMapSize() const
{
  return m_mapSize;
}

Vector2 Mission::ParseMapSize(const toml::value &data)
{
  std::array<float, 2> size = toml::find_or<std::array<float, 2>>(
      data, "Map", "Size", std::array<float, 2>{800.0f, 600.0f});

  return {size.at(0), size.at(1)};
}

std::vector<std::vector<Vector2>> Mission::ParseHoles(const toml::value &data)
{
  std::vector<std::vector<Vector2>> holes;
  std::vector<toml::value> tables = toml::find_or<std::vector<toml::value>>(
      data, "Map", "Hole", std::vector<toml::value>{});

  for (const toml::value &table : tables)
  {
    std::vector<std::array<float, 2>> points = toml::find_or<std::vector<std::array<float, 2>>>(
        table, "Points", std::vector<std::array<float, 2>>{});

    if (points.size() < 3)
    {
      continue;
    }

    std::vector<Vector2> hole;

    for (const std::array<float, 2> &point : points)
    {
      hole.push_back({point.at(0), point.at(1)});
    }

    holes.push_back(hole);
  }

  return holes;
}
