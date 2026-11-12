#pragma once

#include <raylib.h>
#include <vector>

#include "globals.h"

class Surface
{
public:
  Surface(const std::vector<Vector2> &points);
  bool Contains(Vector2 point) const;
  void Render() const;

private:
  std::vector<Vector2> m_points;
};
