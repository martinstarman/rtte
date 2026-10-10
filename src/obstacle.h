#pragma once

#include <raylib.h>
#include <vector>

class Obstacle
{
public:
  Obstacle(const std::vector<Vector2> &points, bool blocksMovement, bool blocksVision);
  const std::vector<Vector2> &GetPoints() const;
  bool BlocksMovement() const;
  bool BlocksVision() const;

private:
  std::vector<Vector2> m_points;
  bool m_blocksMovement;
  bool m_blocksVision;
};
