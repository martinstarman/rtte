#pragma once

#include <algorithm>
#include <array>
#include <raylib.h>
#include <string>
#include <toml.hpp>
#include <vector>

#include "navmesh.h"

class Mission
{
public:
  Mission(const std::string &path);
  void Update(float dt);
  void Render();
  Vector2 GetMapSize() const;

private:
  Mission(const std::string &path, const toml::value &data);
  static Vector2 ParseMapSize(const toml::value &data);
  static std::vector<std::vector<Vector2>> ParseHoles(const toml::value &data);

  std::string m_path;
  Vector2 m_mapSize;
  Camera2D m_camera;
  float m_scrollSpeed;
  Navmesh m_navmesh;
  Vector2 m_pathStart;
  std::vector<Vector2> m_walkPath;
};
