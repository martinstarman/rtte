#pragma once

#include <raylib.h>
#include <vector>

class Surface
{
public:
  Surface(const std::vector<Vector2> &points, bool footprints);
  bool Contains(Vector2 point) const;
  bool HasFootprints() const;
  void Render() const;

private:
  std::vector<Vector2> m_points;
  bool m_footprints;
};
