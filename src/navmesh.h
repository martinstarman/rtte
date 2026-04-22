#pragma once

// TODO: sort
#include <array>
#include <mapbox/earcut.hpp>
#include <raylib.h>
#include <vector>
#include <CXXGraph/CXXGraph.hpp>
#include <string> // TODO: remove me
#include <limits>
#include <memory> // shared_ptr

// #include "triangle.h"
#include "utils.h"

struct Triangle
{
  Vector2 a;
  Vector2 b;
  Vector2 c;
  Vector2 centroid;
};

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
  void Build();
  void Draw();
  void GetPath(Vector2 start, Vector2 target);
  void GetPathCleaned();

private:
  std::vector<Triangle> m_triangles;

  std::vector<Vector2> m_path;
  std::vector<Vector2> m_pathCleaned;
};
