#include "navmesh.h"

Navmesh::Navmesh()
{
  Build();
};

Navmesh::~Navmesh() = default;

void Navmesh::Build()
{
  m_triangles.clear();
  m_trianglePath.clear();
  m_path.clear();

  std::vector<std::vector<std::array<float, 2>>> polygon;
  polygon.push_back({{100, 0}, {100, 100}, {0, 100}, {0, 0}}); // main polygon (map rect)
  polygon.push_back({{75, 25}, {75, 75}, {25, 75}, {25, 25}}); // holes in main polygon

  std::vector<uint32_t> indices = mapbox::earcut<uint32_t>(polygon);
  std::vector<Vector2> trianglesIndices;

  for (size_t i = 0; i < indices.size(); ++i)
  {
    auto index = indices.at(i);
    std::array<float, 2> p;

    if (index < 4)
    {
      p = polygon.at(0).at(index);
    }
    else
    {
      p = polygon.at(1).at(index - polygon.at(0).size());
    }

    trianglesIndices.emplace_back(Vector2{p.at(0), p.at(1)});
  }

  m_triangles.reserve(trianglesIndices.size() / 3);

  for (size_t i = 0; i < trianglesIndices.size(); i += 3)
  {
    Triangle triangle = Triangle(
        trianglesIndices.at(i),
        trianglesIndices.at(i + 1),
        trianglesIndices.at(i + 2));
    m_triangles.push_back(triangle);
  }
};

void Navmesh::Draw()
{
  for (const auto &t : m_triangles)
  {
    DrawLineV(t.GetA(), t.GetB(), WHITE);
    DrawLineV(t.GetB(), t.GetC(), WHITE);
    DrawLineV(t.GetC(), t.GetA(), WHITE);
  }

  for (size_t i = 0; i + 1 < m_trianglePath.size(); ++i)
  {
    Vector2 a = m_triangles.at(m_trianglePath.at(i)).GetCentroid();
    Vector2 b = m_triangles.at(m_trianglePath.at(i + 1)).GetCentroid();
    DrawLineV(a, b, BLACK);
  }

  for (size_t i = 0; i + 1 < m_path.size(); ++i)
  {
    Vector2 a = m_path.at(i);
    Vector2 b = m_path.at(i + 1);
    DrawLineV(a, b, GREEN);
  }

  for (const auto &p : m_portals)
  {
    DrawCircleV(p.left, 4.0f, BLUE);
    DrawCircleV(p.right, 4.0f, RED);
    DrawLineV(p.left, p.right, YELLOW);
  }
};

void Navmesh::GetPath(const Vector2 &start, const Vector2 &target)
{
  m_path.clear();
  m_portals.clear();
  m_trianglePath.clear();

  const size_t startTriangleIndex = FindTriangleForPoint(start);
  const size_t targetTriangleIndex = FindTriangleForPoint(target);

  if (startTriangleIndex < 0 || targetTriangleIndex < 0)
  {
    return;
  }

  std::vector<std::vector<size_t>> neighbors(m_triangles.size());

  for (size_t i = 0; i < m_triangles.size(); ++i)
  {
    for (size_t j = i + 1; j < m_triangles.size(); ++j)
    {
      Vector2 sharedA;
      Vector2 sharedB;
      if (!GetSharedEdge(m_triangles[i], m_triangles[j], sharedA, sharedB))
      {
        continue;
      }

      neighbors[i].push_back(j);
      neighbors[j].push_back(i);
    }
  }

  std::vector<float> distance(m_triangles.size(), std::numeric_limits<float>::max());
  std::vector<size_t> previous(m_triangles.size(), m_triangles.size());
  std::vector<bool> visited(m_triangles.size(), false);

  distance[startTriangleIndex] = 0.0f;

  for (size_t step = 0; step < m_triangles.size(); ++step)
  {
    size_t current = m_triangles.size();
    float bestDistance = std::numeric_limits<float>::max();

    for (size_t i = 0; i < m_triangles.size(); ++i)
    {
      if (!visited[i] && distance[i] < bestDistance)
      {
        bestDistance = distance[i];
        current = i;
      }
    }

    if (current == m_triangles.size())
    {
      break;
    }

    if (current == targetTriangleIndex)
    {
      break;
    }

    visited[current] = true;

    for (const size_t neighbor : neighbors[current])
    {
      if (visited[neighbor])
      {
        continue;
      }

      const float weight = Vector2Distance(m_triangles[current].GetCentroid(), m_triangles[neighbor].GetCentroid());
      const float candidateDistance = distance[current] + weight;
      if (candidateDistance < distance[neighbor])
      {
        distance[neighbor] = candidateDistance;
        previous[neighbor] = current;
      }
    }
  }

  if (distance[targetTriangleIndex] == std::numeric_limits<float>::max())
  {
    if (startTriangleIndex == targetTriangleIndex)
    {
      m_trianglePath.push_back(startTriangleIndex);
      m_path.push_back(start);
      m_path.push_back(target);
    }
    return;
  }

  for (size_t node = targetTriangleIndex; node != m_triangles.size(); node = previous[node])
  {
    m_trianglePath.push_back(node);
    if (node == startTriangleIndex)
    {
      break;
    }
  }

  if (m_trianglePath.empty() || m_trianglePath.back() != startTriangleIndex)
  {
    return;
  }

  std::reverse(m_trianglePath.begin(), m_trianglePath.end());

  auto addIfDifferent = [&](const Vector2 &point)
  {
    if (m_path.empty() || !Vector2Equals(m_path.back(), point))
    {
      m_path.push_back(point);
    }
  };

  m_portals.reserve(m_trianglePath.size() + 1);
  m_portals.push_back(Portal{start, start});

  for (size_t i = 0; i + 1 < m_trianglePath.size(); ++i)
  {
    Vector2 edgeA;
    Vector2 edgeB;
    const size_t fromTriangle = m_trianglePath[i];
    const size_t toTriangle = m_trianglePath[i + 1];
    if (!GetSharedEdge(m_triangles[fromTriangle], m_triangles[toTriangle], edgeA, edgeB))
    {
      return;
    }

    const Vector2 from = m_triangles[fromTriangle].GetCentroid();
    const Vector2 to = m_triangles[toTriangle].GetCentroid();
    const float sideA = CrossProduct(from, to, edgeA);
    const float sideB = CrossProduct(from, to, edgeB);

    Portal portal;
    if (sideA >= sideB)
    {
      portal.left = edgeB;
      portal.right = edgeA;
    }
    else
    {
      portal.left = edgeA;
      portal.right = edgeB;
    }

    m_portals.push_back(portal);
  }

  m_portals.push_back(Portal{target, target});

  Vector2 portalApex = m_portals[0].left;
  Vector2 portalLeft = m_portals[0].left;
  Vector2 portalRight = m_portals[0].right;
  size_t apexIndex = 0;
  size_t leftIndex = 0;
  size_t rightIndex = 0;

  addIfDifferent(portalApex);

  for (size_t i = 1; i < m_portals.size(); ++i)
  {
    const Vector2 left = m_portals[i].left;
    const Vector2 right = m_portals[i].right;

    if (CrossProduct(portalApex, portalRight, right) <= 0.0f)
    {
      if (Vector2Equals(portalApex, portalRight) || CrossProduct(portalApex, portalLeft, right) > 0.0f)
      {
        portalRight = right;
        rightIndex = i;
      }
      else
      {
        addIfDifferent(portalLeft);

        portalApex = portalLeft;
        apexIndex = leftIndex;
        portalRight = portalApex;
        rightIndex = apexIndex;
        i = apexIndex;
        continue;
      }
    }

    if (CrossProduct(portalApex, portalLeft, left) >= 0.0f)
    {
      if (Vector2Equals(portalApex, portalLeft) || CrossProduct(portalApex, portalRight, left) < 0.0f)
      {
        portalLeft = left;
        leftIndex = i;
      }
      else
      {
        addIfDifferent(portalRight);

        portalApex = portalRight;
        apexIndex = rightIndex;
        portalLeft = portalApex;
        leftIndex = apexIndex;
        i = apexIndex;
        continue;
      }
    }
  }

  addIfDifferent(target);
}

size_t Navmesh::FindTriangleForPoint(const Vector2 &point)
{
  for (size_t i = 0; i < m_triangles.size(); ++i)
  {
    if (m_triangles.at(i).Contains(point))
    {
      return i;
    }
  }

  return -1;
}

bool Navmesh::GetSharedEdge(const Triangle &lhs, const Triangle &rhs, Vector2 &outA, Vector2 &outB)
{
  int sharedCount = 0;
  Vector2 shared[2] = {};

  for (const Vector2 &l : lhs.GetVertices())
  {
    for (const Vector2 &r : rhs.GetVertices())
    {
      if (Vector2Equals(l, r))
      {
        if (sharedCount == 0 || !Vector2Equals(shared[0], l))
        {
          if (sharedCount < 2)
          {
            shared[sharedCount] = l;
          }
          ++sharedCount;
        }
        break;
      }
    }
  }

  if (sharedCount < 2)
  {
    return false;
  }

  outA = shared[0];
  outB = shared[1];
  return true;
}
