#include "utils.h"

float GetAngleBetween(Vector2 a, Vector2 b)
{
  float dx = a.x - b.x;
  float dy = a.y - b.y;
  return std::atan2(dy, dx);
}

// see https://gamedev.stackexchange.com/a/49300
Octant GetOctantFrom(float angle)
{
  float value = 8.0 * angle / (2.0 * M_PI) + 8.0;
  int octant = (int)(value + 0.5); // round
  return static_cast<Octant>(octant % 8);
}

Octant GetOctantFrom(const std::string &string)
{
  return OCTANT_MAP.at(string);
}

bool Vector2Equals(const Vector2 &lhs, const Vector2 &rhs)
{
  return std::fabs(lhs.x - rhs.x) <= EPSILON && std::fabs(lhs.y - rhs.y) <= EPSILON;
}

float Vector2Distance(const Vector2 &lhs, const Vector2 &rhs)
{
  const float dx = lhs.x - rhs.x;
  const float dy = lhs.y - rhs.y;
  return std::sqrt((dx * dx) + (dy * dy));
}
