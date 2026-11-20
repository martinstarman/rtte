#pragma once

#include <filesystem>
#include <optional>
#include <string>
#include <toml.hpp>
#include <vector>

#include "sprite.h"

class Mission
{
public:
  Mission(const std::string &path);
  ~Mission();
    void Update(float dt);
  void Render();
  Vector3 ParsePosition(const toml::value &table);
  Vector2 ParseSize(const toml::value &table);
  std::optional<Animation> ParseAnimation(const toml::value &table);
  void ParseSprites(const toml::value &data);

private:
  std::string m_path;
  std::vector<Sprite *> m_sprites;
};
