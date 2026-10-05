#pragma once

#include <limits>
#include <raylib.h>
#include <raymath.h>
#include <vector>

class Navmesh
{
public:
  Navmesh(Rectangle bounds, const std::vector<std::vector<Vector2>> &holes, float agentRadius);
  std::vector<Vector2> FindPath(Vector2 start, Vector2 target) const;
  void Render() const;

private:
  static constexpr float kEpsilon = 1e-3f;

  static Vector2 OutwardNormal(Vector2 a, Vector2 b);
  bool IsWalkable(Vector2 v) const;
  bool IsVisible(Vector2 a, Vector2 b) const;

  Rectangle m_bounds;
  std::vector<std::vector<Vector2>> m_holes;
  std::vector<std::vector<Vector2>> m_expandedHoles;
  std::vector<Vector2> m_nodes;
  std::vector<std::vector<int>> m_edges;
};
