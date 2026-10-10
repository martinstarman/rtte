#include "surface.h"

Surface::Surface(const std::vector<Vector2> &points, bool footprints)
    : m_points(points),
      m_footprints(footprints)
{
}

bool Surface::Contains(Vector2 point) const
{
  return CheckCollisionPointPoly(point, &m_points.at(0), m_points.size());
}

bool Surface::HasFootprints() const
{
  return m_footprints;
}

void Surface::Render() const
{
  DrawLineStrip(m_points.data(), (int)m_points.size(), Fade(WHITE, 0.5f));
  DrawLineV(m_points.back(), m_points.front(), Fade(WHITE, 0.5f));
}
