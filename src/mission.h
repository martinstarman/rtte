#pragma once

#include <algorithm>
#include <array>
#include <cmath>
#include <filesystem>
#include <optional>
#include <raylib.h>
#include <raymath.h>
#include <string>
#include <toml.hpp>
#include <vector>

#include "animation.h"
#include "character.h"
#include "navmesh.h"
#include "renderable.h"
#include "sprite.h"

class Mission
{
public:
  Mission(const std::string &path);
  void Update(float dt);
  void Render();
  Vector2 GetMapSize() const;
  const std::vector<Character> &GetCharacters() const;
  const std::vector<Sprite> &GetSprites() const;

private:
  Mission(const std::string &path, const toml::value &data);
  static Vector2 ParseMapSize(const toml::value &data);
  static std::vector<std::vector<Vector2>> ParseObstacles(const toml::value &data);
  static std::vector<Character> ParseCharacters(const toml::value &data);
  static std::vector<Sprite> ParseSprites(const toml::value &data, const std::filesystem::path &directory);
  static std::optional<Animation> ParseAnimation(const toml::value &table);
  static Vector3 ParsePosition(const toml::value &table);
  static Vector2 ParseSize(const toml::value &table);
  static std::vector<Vector2> ParsePoints(const toml::value &table, const std::string &key);
  static Rectangle MakeRectangle(Vector2 a, Vector2 b);
  bool IsDragSelecting(Vector2 mouse) const;
  void Click(Vector2 point);
  void SelectInRectangle(Rectangle rect);
  void MoveSelected(Vector2 target);

  static constexpr float kDragThreshold = 4.0f;

  std::string m_path;
  Vector2 m_mapSize;
  Camera2D m_camera;
  float m_scrollSpeed;
  Navmesh m_navmesh;
  std::vector<Sprite> m_sprites;
  std::vector<Character> m_characters;
  bool m_dragging;
  Vector2 m_dragStart;
};
