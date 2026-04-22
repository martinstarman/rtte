#include "triangle.h"

Triangle::Triangle(Vector2 a, Vector2 b, Vector2 c) : m_a(a), m_b(b), m_c(c)
{
  m_centroid = Vector2{(a.x + b.x + c.x) / 3.0f,
                       (a.y + b.y + c.y) / 3.0f};
}

Triangle::~Triangle() = default;

Vector2 Triangle::GetA()
{
  return m_a;
}

Vector2 Triangle::GetB()
{
  return m_b;
}

Vector2 Triangle::GetC()
{
  return m_c;
}

Vector2 Triangle::GetCentroid()
{
  return m_centroid;
}
