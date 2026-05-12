#pragma once

#include <array>
#include <CXXGraph/CXXGraph.hpp>
#include <mapbox/earcut.hpp>
#include <raylib.h>
#include <string>
#include <vector>

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
  Navmesh(Rectangle mapRect);
  ~Navmesh();
  void AddHole(const std::vector<std::array<float, 2>> &hole);
  void Draw() const;
  std::vector<Vector2> GetPath(const Vector2 &start, const Vector2 &target) const;

private:
  void Triangulate();
  size_t GetTriangleIndexFrom(const Vector2 &point) const;
  std::vector<std::vector<std::array<float, 2>>> m_polygons;
  std::vector<Triangle> m_triangles;
};
