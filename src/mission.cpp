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
      m_obstacles(ParseObstacles(data)),
      m_navmesh({0.0, 0.0, m_mapSize.x, m_mapSize.y}, GetMovementHoles(m_obstacles), 8.0f),
      m_surfaces(ParseSurfaces(data)),
      m_sprites(ParseSprites(data, std::filesystem::path(path).parent_path())),
      m_characters(ParseCharacters(data)),
      m_enemies(ParseEnemies(data)),
      m_selectedEnemy(std::nullopt),
      m_footprints({}),
      m_dragging(false),
      m_dragStart({0.0, 0.0})
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
    m_dragging = true;
    m_dragStart = mouse;
  }

  if (m_dragging && IsMouseButtonReleased(MOUSE_BUTTON_LEFT))
  {
    if (IsDragSelecting(mouse))
    {
      SelectInRectangle(MakeRectangle(m_dragStart, mouse));
    }
    else
    {
      Click(mouse);
    }

    m_dragging = false;
  }

  for (Sprite &sprite : m_sprites)
  {
    sprite.Update(dt);
  }

  for (Footprint &footprint : m_footprints)
  {
    footprint.Update(dt);
  }

  m_footprints.erase(
      std::remove_if(m_footprints.begin(), m_footprints.end(), [](const Footprint &footprint)
                     { return footprint.IsExpired(); }),
      m_footprints.end());

  for (Enemy &enemy : m_enemies)
  {
    enemy.Update(dt);
  }

  for (Character &character : m_characters)
  {
    character.Update(dt);

    for (const Footprint &footprint : character.GetFootprints())
    {
      Vector3 position = footprint.GetPosition();

      if (HasFootprints({position.x, position.y}))
      {
        m_footprints.push_back(footprint);
      }
    }
  }
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

  for (const Surface &surface : m_surfaces)
  {
    surface.Render();
  }

  for (const Obstacle &obstacle : m_obstacles)
  {
    if (obstacle.BlocksVision())
    {
      const std::vector<Vector2> &points = obstacle.GetPoints();

      for (size_t i = 0; i < points.size(); ++i)
      {
        DrawLineEx(points.at(i), points.at((i + 1) % points.size()), 3.0f, SKYBLUE);
      }
    }
  }

  m_navmesh.Render();

  std::vector<const Renderable *> renderables;

  for (const Sprite &sprite : m_sprites)
  {
    renderables.push_back(&sprite);
  }

  for (const Character &character : m_characters)
  {
    renderables.push_back(&character);
  }

  for (const Enemy &enemy : m_enemies)
  {
    renderables.push_back(&enemy);
  }

  for (const Footprint &footprint : m_footprints)
  {
    renderables.push_back(&footprint);
  }

  std::stable_sort(renderables.begin(), renderables.end(), [](const Renderable *a, const Renderable *b)
                   {
                     float az = a->GetPosition().z;
                     float bz = b->GetPosition().z;
                     return az != bz ? az < bz : a->GetYSortIndex() < b->GetYSortIndex(); });

  for (const Renderable *renderable : renderables)
  {
    renderable->Render();
  }

  if (m_selectedEnemy)
  {
    RenderViewCone(m_enemies.at(*m_selectedEnemy));
  }

  Vector2 mouse = GetScreenToWorld2D(GetMousePosition(), m_camera);

  if (IsDragSelecting(mouse))
  {
    Rectangle rect = MakeRectangle(m_dragStart, mouse);
    DrawRectangleRec(rect, Fade(YELLOW, 0.2f));
    DrawRectangleLinesEx(rect, 1.0f, YELLOW);
  }

  EndMode2D();
}

Vector2 Mission::GetMapSize() const
{
  return m_mapSize;
}

const std::vector<Character> &Mission::GetCharacters() const
{
  return m_characters;
}

const std::vector<Enemy> &Mission::GetEnemies() const
{
  return m_enemies;
}

std::optional<size_t> Mission::GetSelectedEnemy() const
{
  return m_selectedEnemy;
}

const std::vector<Sprite> &Mission::GetSprites() const
{
  return m_sprites;
}

const std::vector<Obstacle> &Mission::GetObstacles() const
{
  return m_obstacles;
}

const std::vector<Surface> &Mission::GetSurfaces() const
{
  return m_surfaces;
}

Rectangle Mission::MakeRectangle(Vector2 a, Vector2 b)
{
  return {std::min(a.x, b.x), std::min(a.y, b.y), std::abs(a.x - b.x), std::abs(a.y - b.y)};
}

bool Mission::IsDragSelecting(Vector2 mouse) const
{
  return m_dragging && Vector2Distance(m_dragStart, mouse) >= kDragThreshold;
}

void Mission::Click(Vector2 point)
{
  for (size_t i = 0; i < m_characters.size(); ++i)
  {
    if (m_characters.at(i).Contains(point))
    {
      for (size_t j = 0; j < m_characters.size(); ++j)
      {
        m_characters.at(j).SetSelected(i == j);
      }

      return;
    }
  }

  for (size_t i = 0; i < m_enemies.size(); ++i)
  {
    if (m_enemies.at(i).Contains(point))
    {
      if (m_selectedEnemy == i)
      {
        m_selectedEnemy = std::nullopt;
      }
      else
      {
        m_selectedEnemy = i;
      }

      return;
    }
  }

  MoveSelected(point);
}

void Mission::RenderViewCone(const Enemy &enemy) const
{
  std::vector<Vector2> polygon = enemy.GetViewPolygon(m_obstacles);
  Vector2 origin = polygon.at(0);

  for (size_t i = 1; i + 1 < polygon.size(); ++i)
  {
    Vector2 a = polygon.at(i);
    Vector2 b = polygon.at(i + 1);

    float cross = (a.x - origin.x) * (b.y - origin.y) - (a.y - origin.y) * (b.x - origin.x);

    if (cross > 0.0f)
    {
      std::swap(a, b);
    }

    DrawTriangle(origin, a, b, Fade(RED, 0.25f));
  }
}

void Mission::SelectInRectangle(Rectangle rect)
{
  for (Character &character : m_characters)
  {
    Vector3 position = character.GetPosition();
    character.SetSelected(CheckCollisionPointRec({position.x, position.y}, rect));
  }
}

void Mission::MoveSelected(Vector2 target)
{
  for (Character &character : m_characters)
  {
    if (character.IsSelected())
    {
      Vector3 position = character.GetPosition();
      character.SetPath(m_navmesh.FindPath({position.x, position.y}, target));
    }
  }
}

bool Mission::HasFootprints(Vector2 point) const
{
  for (const Surface &surface : m_surfaces)
  {
    if (surface.HasFootprints() && surface.Contains(point))
    {
      return true;
    }
  }

  return false;
}

Vector2 Mission::ParseMapSize(const toml::value &data)
{
  std::array<float, 2> size = toml::find_or<std::array<float, 2>>(
      data, "Map", "Size", std::array<float, 2>{800.0f, 600.0f});

  return {size.at(0), size.at(1)};
}

std::vector<Obstacle> Mission::ParseObstacles(const toml::value &data)
{
  std::vector<Obstacle> obstacles;
  std::vector<toml::value> tables = toml::find_or<std::vector<toml::value>>(
      data, "Obstacle", std::vector<toml::value>{});

  for (const toml::value &table : tables)
  {
    std::vector<Vector2> points = ParsePoints(table, "Points");

    if (points.size() < 3)
    {
      continue;
    }

    bool blocksMovement = toml::find_or<bool>(table, "BlocksMovement", true);
    bool blocksVision = toml::find_or<bool>(table, "BlocksVision", false);

    obstacles.emplace_back(points, blocksMovement, blocksVision);
  }

  return obstacles;
}

std::vector<std::vector<Vector2>> Mission::GetMovementHoles(const std::vector<Obstacle> &obstacles)
{
  std::vector<std::vector<Vector2>> holes;

  for (const Obstacle &obstacle : obstacles)
  {
    if (obstacle.BlocksMovement())
    {
      holes.push_back(obstacle.GetPoints());
    }
  }

  return holes;
}

std::vector<Surface> Mission::ParseSurfaces(const toml::value &data)
{
  std::vector<Surface> surfaces;
  std::vector<toml::value> tables = toml::find_or<std::vector<toml::value>>(
      data, "Surface", std::vector<toml::value>{});

  for (const toml::value &table : tables)
  {
    std::vector<Vector2> points = ParsePoints(table, "Points");

    if (points.size() < 3)
    {
      continue;
    }

    bool footprints = toml::find_or<bool>(table, "Footprints", false);

    surfaces.emplace_back(points, footprints);
  }

  return surfaces;
}

std::vector<Character> Mission::ParseCharacters(const toml::value &data)
{
  std::vector<Character> characters;
  std::vector<toml::value> tables = toml::find_or<std::vector<toml::value>>(
      data, "Character", std::vector<toml::value>{});

  for (const toml::value &table : tables)
  {
    if (!table.contains("Position"))
    {
      continue;
    }

    characters.push_back(Character(ParsePosition(table)));
  }

  return characters;
}

std::vector<Enemy> Mission::ParseEnemies(const toml::value &data)
{
  std::vector<Enemy> enemies;
  std::vector<toml::value> tables = toml::find_or<std::vector<toml::value>>(
      data, "Enemy", std::vector<toml::value>{});

  for (const toml::value &table : tables)
  {
    if (!table.contains("Position"))
    {
      continue;
    }

    Vector3 position = ParsePosition(table);
    std::vector<Vector2> path = ParsePoints(table, "Path");
    float direction = toml::find_or<float>(table, "Direction", 0.0f);

    enemies.emplace_back(position, path, direction);
  }

  return enemies;
}

std::vector<Sprite> Mission::ParseSprites(const toml::value &data, const std::filesystem::path &directory)
{
  std::vector<Sprite> sprites;
  std::vector<toml::value> tables = toml::find_or<std::vector<toml::value>>(
      data, "Sprite", std::vector<toml::value>{});

  for (const toml::value &table : tables)
  {
    if (!table.contains("Path") || !table.contains("Position"))
    {
      continue;
    }

    std::string path = (directory / toml::find<std::string>(table, "Path")).string();
    Vector3 position = ParsePosition(table);
    std::optional<Animation> animation = ParseAnimation(table);

    if (table.contains("Shape"))
    {
      std::vector<Vector2> shape = ParsePoints(table, "Shape");

      if (shape.size() < 3)
      {
        continue;
      }

      if (animation)
      {
        sprites.emplace_back(path, position, shape, *animation);
      }
      else
      {
        sprites.emplace_back(path, position, shape);
      }
    }
    else if (table.contains("Size"))
    {
      Vector2 size = ParseSize(table);

      if (animation)
      {
        sprites.emplace_back(path, position, size, *animation);
      }
      else
      {
        sprites.emplace_back(path, position, size);
      }
    }
    else
    {
      if (animation)
      {
        sprites.emplace_back(path, position, *animation);
      }
      else
      {
        sprites.emplace_back(path, position);
      }
    }
  }

  return sprites;
}

std::optional<Animation> Mission::ParseAnimation(const toml::value &table)
{
  if (!table.contains("Animation"))
  {
    return std::nullopt;
  }

  const toml::value &animation = toml::find(table, "Animation");
  size_t frameCount = toml::find<size_t>(animation, "FrameCount");
  std::array<float, 2> frameSize = toml::find<std::array<float, 2>>(animation, "FrameSize");
  float fps = toml::find_or<float>(animation, "Fps", 1.0f);

  return Animation(frameCount, {frameSize.at(0), frameSize.at(1)}, fps);
}

Vector3 Mission::ParsePosition(const toml::value &table)
{
  std::array<float, 3> position = toml::find<std::array<float, 3>>(table, "Position");

  return {position.at(0), position.at(1), position.at(2)};
}

Vector2 Mission::ParseSize(const toml::value &table)
{
  std::array<float, 2> size = toml::find<std::array<float, 2>>(table, "Size");

  return {size.at(0), size.at(1)};
}

std::vector<Vector2> Mission::ParsePoints(const toml::value &table, const std::string &key)
{
  std::vector<std::array<float, 2>> points = toml::find_or<std::vector<std::array<float, 2>>>(
      table, key, std::vector<std::array<float, 2>>{});
  std::vector<Vector2> result;

  for (const std::array<float, 2> &point : points)
  {
    result.push_back({point.at(0), point.at(1)});
  }

  return result;
}
