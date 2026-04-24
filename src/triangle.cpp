#include "triangle.h"

Triangle::Triangle(Vector2 a, Vector2 b, Vector2 c) : m_a(a), m_b(b), m_c(c)
{
  m_centroid = Vector2{(a.x + b.x + c.x) / 3.0f,
                       (a.y + b.y + c.y) / 3.0f};
}

Triangle::~Triangle() = default;

const Vector2 &Triangle::GetA() const
{
  return m_a;
}

const Vector2 &Triangle::GetB() const
{
  return m_b;
}

const Vector2 &Triangle::GetC() const
{
  return m_c;
}

const Vector2 &Triangle::GetCentroid() const
{
  return m_centroid;
}

bool Triangle::Contains(const Vector2 v) const
{
  const float d1 = CrossProduct(v, m_a, m_b);
  const float d2 = CrossProduct(v, m_b, m_c);
  const float d3 = CrossProduct(v, m_c, m_a);

  const bool hasNeg = (d1 < 0.0f) || (d2 < 0.0f) || (d3 < 0.0f);
  const bool hasPos = (d1 > 0.0f) || (d2 > 0.0f) || (d3 > 0.0f);

  return !(hasNeg && hasPos);
}

const std::array<Vector2, 3> Triangle::GetVertices() const
{
  return {m_a, m_b, m_c};
}

bool Triangle::ShareEdge(const Triangle &t) const
{
  int sharedVertices = 0;

  for (const auto &v1 : GetVertices())
  {
    for (const auto &v2 : t.GetVertices())
    {
      if (Vector2Equals(v1, v2))
      {
        sharedVertices++;
        break;
      }
    }
  }

  return sharedVertices >= 2;
}
