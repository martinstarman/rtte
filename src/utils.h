#pragma once

#include <cmath>
#include <raylib.h>

enum Octant
{
  East,
  NorthEast,
  North,
  NorthWest,
  West,
  SouthWest,
  South,
  SouthEast,
};

float GetAngleBetween(Vector2 a, Vector2 b);
Octant GetOctantFrom(float angle);
