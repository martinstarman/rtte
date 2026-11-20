#include "sector.h"

Sector::Sector(const std::vector<Vector2> &points, bool blocksMovement, bool blocksVision, bool showFootprints)
    : m_points(points),
      m_blocksMovement(blocksMovement),
      m_blocksVision(blocksVision)
{
}

const std::vector<Vector2> &Sector::GetPoints() const
{
  return m_points;
}

bool Sector::BlocksMovement() const
{
  return m_blocksMovement;
}

bool Sector::BlocksVision() const
{
  return m_blocksVision;
}

bool Sector::ShowFootprints() const
{
  return m_showFootprints;
}

bool Sector::Contains(Vector2 point) const
{
  return CheckCollisionPointPoly(point, &m_points.at(0), m_points.size());
}
