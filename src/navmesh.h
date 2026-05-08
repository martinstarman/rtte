#pragma once

// TODO: sort
#include <array>
#include <mapbox/earcut.hpp>
#include <raylib.h>
#include <vector>
#include <CXXGraph/CXXGraph.hpp>
#include <limits>
#include "utils.h"
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
  std::vector<Triangle> m_triangles;
  std::vector<Vector2> m_path;
  std::vector<Vector2> m_pathCleaned;
};
