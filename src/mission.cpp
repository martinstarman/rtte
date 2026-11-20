#include "mission.h"

Mission::Mission(const std::string &path)
    : m_path(path),
      m_sprites({})
{
  toml::value tomlData = toml::parse(path);
  ParseSprites(tomlData);
}

Mission::~Mission()
{
  for (auto sprite : m_sprites)
  {
    delete sprite;
  }
}

void Mission::Update(float dt)
{
  for (auto sprite : m_sprites)
  {
    sprite->Update(dt);
  }
}

void Mission::Render()
{
  for (auto sprite : m_sprites)
  {
    sprite->Render();
  }
}

Vector3 Mission::ParsePosition(const toml::value &table)
{
  std::array<int, 3> position = toml::find<std::array<int, 3>>(table, "position");

  return {(float)position.at(0), (float)position.at(1), (float)position.at(2)};
}

Vector2 Mission::ParseSize(const toml::value &table)
{
  std::array<int, 2> size = toml::find<std::array<int, 2>>(table, "size");

  return {(float)size.at(0), (float)size.at(1)};
}

std::optional<Animation> Mission::ParseAnimation(const toml::value &table)
{
  if (!table.contains("animation"))
  {
    return std::nullopt;
  }

  const toml::value &animation = toml::find(table, "animation");
  int frameCount = toml::find<int>(animation, "frame_count");
  std::array<int, 2> frameSize = toml::find<std::array<int, 2>>(animation, "frame_size");
  float fps = toml::find_or<int>(animation, "fps", 1);

  return Animation(frameCount, {(float)frameSize.at(0), (float)frameSize.at(1)}, fps);
}

void Mission::ParseSprites(const toml::value &data)
{
  std::vector<toml::value> tables = toml::find_or<std::vector<toml::value>>(
      data, "sprites", std::vector<toml::value>{});

  std::filesystem::path dir = std::filesystem::path(m_path).parent_path();

  for (const toml::value &table : tables)
  {
    std::string path = (dir / toml::find<std::string>(table, "path")).string();
    Vector3 position = ParsePosition(table);
    Vector2 size = ParseSize(table);
    std::optional<Animation> animation = ParseAnimation(table);

    if (animation.has_value())
    {
      Sprite *sprite = new Sprite(path, position, size, animation.value());
      m_sprites.emplace_back(sprite);
    }
    else
    {
      Sprite *sprite = new Sprite(path, position, size);
      m_sprites.emplace_back(sprite);
    }
  }
}
