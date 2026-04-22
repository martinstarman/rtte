#pragma once

#include <raylib.h>

class Triangle
{
public:
  Triangle(Vector2 a, Vector2 b, Vector2 c);
  ~Triangle();
  Vector2 GetA();
  Vector2 GetB();
  Vector2 GetC();
  Vector2 GetCentroid();

private:
  Vector2 m_a;
  Vector2 m_b;
  Vector2 m_c;
  Vector2 m_centroid;
};
