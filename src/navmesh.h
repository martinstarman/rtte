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
  void GetPath(const Vector2 &start, const Vector2 &target);

private:
  void Build();
  size_t FindTriangleForPoint(const Vector2 &point);
  bool GetSharedEdge(const Triangle &lhs, const Triangle &rhs, Vector2 &outA, Vector2 &outB);
  std::vector<Triangle> m_triangles;
  std::vector<Vector2> m_path;
  std::vector<Vector2> m_pathCleaned;
  std::vector<Portal> m_debugPortals;
};
