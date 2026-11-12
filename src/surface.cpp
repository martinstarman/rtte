#include "surface.h"

Surface::Surface(const std::vector<Vector2> &points)
    : m_points(points)
{
}

bool Surface::Contains(Vector2 point) const
{
  return CheckCollisionPointPoly(point, &m_points.at(0), m_points.size());
}

void Surface::Render() const
{
  if (g_debug)
  {
    DrawLineStrip(m_points.data(), (int)m_points.size(), Fade(WHITE, 0.5f));
    DrawLineV(m_points.back(), m_points.front(), Fade(WHITE, 0.5f));
  }
}
