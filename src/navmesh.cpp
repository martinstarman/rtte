#include "navmesh.h"

bool GetSharedEdge(const Triangle &lhs, const Triangle &rhs, Vector2 &outA, Vector2 &outB)
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

int FindTriangleForPoint(const std::vector<Triangle> &triangles, const Vector2 &point)
{
  for (size_t i = 0; i < triangles.size(); ++i)
  {
    if (triangles.at(i).Contains(point))
    {
      return static_cast<int>(i);
    }
  }

  return -1;
}

Navmesh::Navmesh()
{
  Build();
};

Navmesh::~Navmesh() = default;

void Navmesh::Build()
{
  m_triangles.clear();
  m_path.clear();
  m_pathCleaned.clear();

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

  for (size_t i = 0; i + 1 < m_path.size(); ++i)
  {
    Vector2 a = m_path.at(i);
    Vector2 b = m_path.at(i + 1);
    DrawLineV(a, b, BLACK);
  }

  for (size_t i = 0; i + 1 < m_pathCleaned.size(); ++i)
  {
    Vector2 a = m_pathCleaned.at(i);
    Vector2 b = m_pathCleaned.at(i + 1);
    DrawLineV(a, b, LIME);
  }

  for (size_t i = 1; i + 1 < m_debugPortals.size(); ++i)
  {
    const Portal &portal = m_debugPortals.at(i);
    DrawCircleV(portal.left, 4.0f, BLUE);
    DrawCircleV(portal.right, 4.0f, RED);
    DrawLineV(portal.left, portal.right, YELLOW);
  }
};

void Navmesh::GetPath(Vector2 start, Vector2 target)
{
  m_trianglePath.clear();
  m_path.clear();
  m_path.push_back(start);
  m_pathCleaned.clear();

  const int startTriangleIndex = FindTriangleForPoint(m_triangles, start);
  const int targetTriangleIndex = FindTriangleForPoint(m_triangles, target);

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

  const float inf = std::numeric_limits<float>::max();
  std::vector<float> distance(m_triangles.size(), inf);
  std::vector<size_t> previous(m_triangles.size(), m_triangles.size());
  std::vector<bool> visited(m_triangles.size(), false);

  distance[static_cast<size_t>(startTriangleIndex)] = 0.0f;

  for (size_t step = 0; step < m_triangles.size(); ++step)
  {
    size_t current = m_triangles.size();
    float bestDistance = inf;

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

    if (current == static_cast<size_t>(targetTriangleIndex))
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

  if (distance[static_cast<size_t>(targetTriangleIndex)] == inf)
  {
    if (startTriangleIndex == targetTriangleIndex)
    {
      m_trianglePath.push_back(static_cast<size_t>(startTriangleIndex));
      m_path.push_back(m_triangles[static_cast<size_t>(startTriangleIndex)].GetCentroid());
      m_path.push_back(target);
    }
    return;
  }

  std::vector<size_t> trianglePath;
  for (size_t node = static_cast<size_t>(targetTriangleIndex); node != m_triangles.size(); node = previous[node])
  {
    trianglePath.push_back(node);
    if (node == static_cast<size_t>(startTriangleIndex))
    {
      break;
    }
  }

  if (trianglePath.empty() || trianglePath.back() != static_cast<size_t>(startTriangleIndex))
  {
    return;
  }

  std::reverse(trianglePath.begin(), trianglePath.end());
  m_trianglePath = trianglePath;

  for (const size_t triangleIndex : trianglePath)
  {
    m_path.push_back(m_triangles[triangleIndex].GetCentroid());
  }

  m_path.push_back(target);

  GetPathCleaned();
}

void Navmesh::GetPathCleaned()
{
  auto addIfDifferent = [&](const Vector2 &point)
  {
    if (m_pathCleaned.empty() || !Vector2Equals(m_pathCleaned.back(), point))
    {
      m_pathCleaned.push_back(point);
    }
  };

  std::vector<Portal> portals;
  portals.reserve(m_trianglePath.size() + 1);
  portals.push_back(Portal{m_path.front(), m_path.front()});

  for (size_t i = 0; i + 1 < m_trianglePath.size(); ++i)
  {
    Vector2 edgeA;
    Vector2 edgeB;
    const size_t fromTriangle = m_trianglePath[i];
    const size_t toTriangle = m_trianglePath[i + 1];
    if (!GetSharedEdge(m_triangles[fromTriangle], m_triangles[toTriangle], edgeA, edgeB))
    {
      m_pathCleaned = m_path;
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

    portals.push_back(portal);
  }

  portals.push_back(Portal{m_path.back(), m_path.back()});
  m_debugPortals = portals;

  Vector2 portalApex = portals[0].left;
  Vector2 portalLeft = portals[0].left;
  Vector2 portalRight = portals[0].right;
  size_t apexIndex = 0;
  size_t leftIndex = 0;
  size_t rightIndex = 0;

  addIfDifferent(portalApex);

  for (size_t i = 1; i < portals.size(); ++i)
  {
    const Vector2 left = portals[i].left;
    const Vector2 right = portals[i].right;

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
        portalLeft = portalApex;
        portalRight = portalApex;
        leftIndex = apexIndex;
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
        portalRight = portalApex;
        leftIndex = apexIndex;
        rightIndex = apexIndex;
        i = apexIndex;
        continue;
      }
    }
  }

  addIfDifferent(m_path.back());
}
