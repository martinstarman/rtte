#include "obstacle.h"

Obstacle::Obstacle(const std::vector<Vector2> &points, bool blocksMovement, bool blocksVision)
    : m_points(points),
      m_blocksMovement(blocksMovement),
      m_blocksVision(blocksVision)
{
}

const std::vector<Vector2> &Obstacle::GetPoints() const
{
  return m_points;
}

bool Obstacle::BlocksMovement() const
{
  return m_blocksMovement;
}

bool Obstacle::BlocksVision() const
{
  return m_blocksVision;
}
