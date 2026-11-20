#pragma once

#include <raylib.h>
#include <vector>

class Sector
{
public:
  Sector(const std::vector<Vector2> &points, bool blocksMovement, bool blocksVision, bool showFootprints);
  const std::vector<Vector2> &GetPoints() const;
  bool BlocksMovement() const;
  bool BlocksVision() const;
  bool ShowFootprints() const;
  bool Contains(Vector2 point) const;

private:
  std::vector<Vector2> m_points;
  bool m_blocksMovement;
  bool m_blocksVision;
  bool m_showFootprints;
};
