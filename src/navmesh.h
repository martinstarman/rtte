#pragma once

// TODO: sort
#include <array>
#include <mapbox/earcut.hpp>
#include <raylib.h>
#include <vector>
#include <limits>
#include <algorithm>
#include <cassert>

#include "triangle.h"
#include "utils.h"

struct Portal
{
  Vector2 left;
  Vector2 right;
};

class Navmesh
{
public:
  Navmesh();
  ~Navmesh();
  void Draw();
  void GetPath(Vector2 start, Vector2 target);
  
  private:
  void Build();
  void GetPathCleaned();
  std::vector<size_t> m_trianglePath;
  std::vector<Triangle> m_triangles;
  std::vector<Vector2> m_path;
  std::vector<Vector2> m_pathCleaned;
  std::vector<Portal> m_debugPortals;
};
