#pragma once

#include <array>
#include <mapbox/earcut.hpp>
#include <raylib.h>
#include <vector>

#include <string>

class Navmesh
{
public:
  Navmesh();
  ~Navmesh();
  void Build();
  void Draw();

private:
  std::vector<Vector2> m_triangles;
};
