#include "navmesh.h"

float Cross2D(const Vector2 &a, const Vector2 &b, const Vector2 &c)
{
  return ((b.x - a.x) * (c.y - a.y)) - ((b.y - a.y) * (c.x - a.x));
}

bool PointInTriangle(const Vector2 &point, const Triangle &triangle)
{
  const float d1 = Cross2D(point, triangle.a, triangle.b);
  const float d2 = Cross2D(point, triangle.b, triangle.c);
  const float d3 = Cross2D(point, triangle.c, triangle.a);

  const bool hasNeg = (d1 < 0.0f) || (d2 < 0.0f) || (d3 < 0.0f);
  const bool hasPos = (d1 > 0.0f) || (d2 > 0.0f) || (d3 > 0.0f);

  return !(hasNeg && hasPos);
}

bool TrianglesShareEdge(const Triangle &lhs, const Triangle &rhs)
{
  int sharedVertices = 0;
  Vector2 lhsVertices[3] = {lhs.a, lhs.b, lhs.c};
  Vector2 rhsVertices[3] = {rhs.a, rhs.b, rhs.c};

  for (const auto &l : lhsVertices)
  {
    for (const auto &r : rhsVertices)
    {
      if (Vector2Equals(l, r))
      {
        sharedVertices++;
        break;
      }
    }
  }

  return sharedVertices >= 2;
}

size_t FindTriangleForPoint(const std::vector<Triangle> &triangles, const Vector2 &point)
{
  for (size_t i = 0; i < triangles.size(); ++i)
  {
    if (PointInTriangle(point, triangles[i]))
    {
      return i;
    }
  }

  size_t nearestTriangle = 0;
  float bestDistance = std::numeric_limits<float>::max();

  for (size_t i = 0; i < triangles.size(); ++i)
  {
    const float d = Vector2Distance(point, triangles[i].centroid);
    if (d < bestDistance)
    {
      bestDistance = d;
      nearestTriangle = i;
    }
  }

  return nearestTriangle;
}

Navmesh::Navmesh() {};

Navmesh::~Navmesh() {};

void Navmesh::Build()
{
  std::vector<std::vector<std::array<float, 2>>> polygon;
  polygon.push_back({{100, 0}, {100, 100}, {0, 100}, {0, 0}}); // main polygon (map rect)
  polygon.push_back({{75, 25}, {75, 75}, {25, 75}, {25, 25}}); // holes in main polygon
  std::vector<uint32_t> indices = mapbox::earcut<uint32_t>(polygon);
  std::vector<Vector2> trianglesIndices;

  for (int i = 0; i < indices.size(); i++)
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

  for (size_t i = 0; i + 2 < trianglesIndices.size(); i += 3)
  {
    Triangle triangle;
    triangle.a = trianglesIndices.at(i);
    triangle.b = trianglesIndices.at(i + 1);
    triangle.c = trianglesIndices.at(i + 2);
    // TODO: use Triangle(a,b,c) class
    triangle.centroid = Vector2{(triangle.a.x + triangle.b.x + triangle.c.x) / 3.0f,
                                (triangle.a.y + triangle.b.y + triangle.c.y) / 3.0f};
    m_triangles.push_back(triangle);
  }
};

void Navmesh::Draw()
{
  for (int i = 0; i < m_triangles.size(); i++)
  {
    Vector2 a = m_triangles.at(i).a;
    Vector2 b = m_triangles.at(i).b;
    Vector2 c = m_triangles.at(i).c;

    DrawLineV(a, b, WHITE);
    DrawLineV(b, c, WHITE);
    DrawLineV(c, a, WHITE);
  }

  // path
  for (size_t i = 0; i + 1 < m_path.size(); ++i)
  {
    Vector2 a = m_path[i];
    Vector2 b = m_path[i + 1];
    DrawLineV(a, b, BLACK);
  }

  // cleaned path
  for (size_t i = 0; i + 1 < m_pathCleaned.size(); ++i)
  {
    Vector2 a = m_pathCleaned[i];
    Vector2 b = m_pathCleaned[i + 1];
    DrawLineV(a, b, LIME);
  }
};

void Navmesh::GetPath(Vector2 start, Vector2 target)
{
  m_path.clear();

  if (m_triangles.empty())
  {
    return;
  }

  m_path.push_back(start);

  const size_t startTriangleIndex = FindTriangleForPoint(m_triangles, start);
  const size_t targetTriangleIndex = FindTriangleForPoint(m_triangles, target);

  std::vector<std::shared_ptr<const CXXGraph::Node<size_t>>> nodes;
  nodes.reserve(m_triangles.size());
  for (size_t i = 0; i < m_triangles.size(); ++i)
  {
    nodes.push_back(std::make_shared<const CXXGraph::Node<size_t>>(std::to_string(i), i));
  }

  CXXGraph::T_EdgeSet<size_t> edgeSet;
  CXXGraph::id_t edgeId = 0;

  for (size_t i = 0; i < m_triangles.size(); ++i)
  {
    for (size_t j = i + 1; j < m_triangles.size(); ++j)
    {
      if (!TrianglesShareEdge(m_triangles[i], m_triangles[j]))
      {
        continue;
      }

      const double weight = static_cast<double>(Vector2Distance(m_triangles[i].centroid, m_triangles[j].centroid));
      edgeSet.insert(std::make_shared<const CXXGraph::UndirectedWeightedEdge<size_t>>(edgeId++, nodes[i], nodes[j], weight));
    }
  }

  CXXGraph::Graph<size_t> graph(edgeSet);
  const auto result = graph.dijkstra(*nodes[startTriangleIndex], *nodes[targetTriangleIndex]);

  if (!result.success || result.path.empty())
  {
    if (startTriangleIndex == targetTriangleIndex)
    {
      m_path.push_back(m_triangles[startTriangleIndex].centroid);
      m_path.push_back(target);
    }
    return;
  }

  for (const auto &nodeId : result.path)
  {
    const size_t triangleIndex = static_cast<size_t>(std::stoull(nodeId));
    if (triangleIndex < m_triangles.size())
    {
      m_path.push_back(m_triangles[triangleIndex].centroid);
    }
  }

  m_path.push_back(target);
}

void Navmesh::GetPathCleaned()
{
  m_pathCleaned.clear();

  if (m_path.empty())
  {
    return;
  }

  if (m_path.size() <= 2)
  {
    m_pathCleaned = m_path;
    return;
  }

  if (m_triangles.empty())
  {
    m_pathCleaned = m_path;
    return;
  }

  std::vector<size_t> corridor;
  corridor.reserve(m_path.size());
  for (size_t i = 1; i + 1 < m_path.size(); ++i)
  {
    const size_t triangleIndex = FindTriangleForPoint(m_triangles, m_path[i]);
    if (corridor.empty() || corridor.back() != triangleIndex)
    {
      corridor.push_back(triangleIndex);
    }
  }

  if (corridor.empty())
  {
    m_pathCleaned = m_path;
    return;
  }

  auto triArea2 = [](const Vector2 &a, const Vector2 &b, const Vector2 &c) -> float
  {
    return ((b.x - a.x) * (c.y - a.y)) - ((b.y - a.y) * (c.x - a.x));
  };

  auto addIfDifferent = [&](const Vector2 &point)
  {
    if (m_pathCleaned.empty() || !Vector2Equals(m_pathCleaned.back(), point))
    {
      m_pathCleaned.push_back(point);
    }
  };

  auto sharedEdge = [&](size_t lhs, size_t rhs, Vector2 &outA, Vector2 &outB) -> bool
  {
    Vector2 lhsVertices[3] = {m_triangles[lhs].a, m_triangles[lhs].b, m_triangles[lhs].c};
    Vector2 rhsVertices[3] = {m_triangles[rhs].a, m_triangles[rhs].b, m_triangles[rhs].c};

    int sharedCount = 0;
    Vector2 shared[2] = {};
    for (const Vector2 &l : lhsVertices)
    {
      for (const Vector2 &r : rhsVertices)
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
  };

  std::vector<Portal> portals;
  portals.reserve(corridor.size() + 1);
  portals.push_back(Portal{m_path.front(), m_path.front()});

  for (size_t i = 0; i + 1 < corridor.size(); ++i)
  {
    Vector2 edgeA{};
    Vector2 edgeB{};
    if (!sharedEdge(corridor[i], corridor[i + 1], edgeA, edgeB))
    {
      m_pathCleaned = m_path;
      return;
    }

    const Vector2 from = m_triangles[corridor[i]].centroid;
    const Vector2 to = m_triangles[corridor[i + 1]].centroid;
    const float sideA = triArea2(from, to, edgeA);
    const float sideB = triArea2(from, to, edgeB);

    Portal portal{};
    if (sideA >= sideB)
    {
      portal.left = edgeA;
      portal.right = edgeB;
    }
    else
    {
      portal.left = edgeB;
      portal.right = edgeA;
    }

    portals.push_back(portal);
  }

  portals.push_back(Portal{m_path.back(), m_path.back()});

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

    if (triArea2(portalApex, portalRight, right) <= 0.0f)
    {
      if (Vector2Equals(portalApex, portalRight) || triArea2(portalApex, portalLeft, right) > 0.0f)
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

    if (triArea2(portalApex, portalLeft, left) >= 0.0f)
    {
      if (Vector2Equals(portalApex, portalLeft) || triArea2(portalApex, portalRight, left) < 0.0f)
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
