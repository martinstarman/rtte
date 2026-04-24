#pragma once

#include <array>
#include <raylib.h>

#include "utils.h"

class Triangle
{
public:
  Triangle(Vector2 a, Vector2 b, Vector2 c);
  ~Triangle();
  const Vector2 &GetA() const;
  const Vector2 &GetB() const;
  const Vector2 &GetC() const;
  const Vector2 &GetCentroid() const;
  bool Contains(const Vector2 v) const;
  const std::array<Vector2, 3> GetVertices() const;
  bool ShareEdge(const Triangle &t) const;

private:
  Vector2 m_a;
  Vector2 m_b;
  Vector2 m_c;
  Vector2 m_centroid;
};
