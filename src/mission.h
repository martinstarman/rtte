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
#include <utility>
#include <vector>

#include "animation.h"
#include "character.h"
#include "enemy.h"
#include "footprint.h"
#include "navmesh.h"
#include "obstacle.h"
#include "renderable.h"
#include "sprite.h"
#include "surface.h"

class Mission
{
public:
  Mission(const std::string &path);
  void Update(float dt);
  void Render();
  Vector2 GetMapSize() const;
  const std::vector<Character> &GetCharacters() const;
  const std::vector<Enemy> &GetEnemies() const;
  std::optional<size_t> GetSelectedEnemy() const;
  const std::vector<Sprite> &GetSprites() const;
  const std::vector<Obstacle> &GetObstacles() const;
  const std::vector<Surface> &GetSurfaces() const;

private:
  Mission(const std::string &path, const toml::value &data);
  static Vector2 ParseMapSize(const toml::value &data);
  static std::vector<Obstacle> ParseObstacles(const toml::value &data);
  static std::vector<std::vector<Vector2>> GetMovementHoles(const std::vector<Obstacle> &obstacles);
  static std::vector<Surface> ParseSurfaces(const toml::value &data);
  static std::vector<Character> ParseCharacters(const toml::value &data);
  static std::vector<Enemy> ParseEnemies(const toml::value &data);
  static std::vector<Sprite> ParseSprites(const toml::value &data, const std::filesystem::path &directory);
  static std::optional<Animation> ParseAnimation(const toml::value &table);
  static Vector3 ParsePosition(const toml::value &table);
  static Vector2 ParseSize(const toml::value &table);
  static std::vector<Vector2> ParsePoints(const toml::value &table, const std::string &key);
  static Rectangle MakeRectangle(Vector2 a, Vector2 b);
  bool IsDragSelecting(Vector2 mouse) const;
  void Click(Vector2 point);
  void RenderViewCone(const Enemy &enemy) const;
  void SelectInRectangle(Rectangle rect);
  void MoveSelected(Vector2 target);
  bool HasFootprints(Vector2 point) const;

  static constexpr float kDragThreshold = 4.0f;

  std::string m_path;
  Vector2 m_mapSize;
  Camera2D m_camera;
  float m_scrollSpeed;
  std::vector<Obstacle> m_obstacles;
  Navmesh m_navmesh;
  std::vector<Surface> m_surfaces;
  std::vector<Sprite> m_sprites;
  std::vector<Character> m_characters;
  std::vector<Enemy> m_enemies;
  std::optional<size_t> m_selectedEnemy;
  std::vector<Footprint> m_footprints;
  bool m_dragging;
  Vector2 m_dragStart;
};
