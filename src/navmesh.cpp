#include "navmesh.h"

Navmesh::Navmesh()
{
  Build();
};

Navmesh::~Navmesh() = default;

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

  // path
  for (size_t i = 0; i + 1 < m_path.size(); ++i)
  {
    Vector2 a = m_path.at(i);
    Vector2 b = m_path.at(i + 1);
    DrawLineV(a, b, BLACK);
  }

  // cleaned path
  for (size_t i = 0; i + 1 < m_pathCleaned.size(); ++i)
  {
    Vector2 a = m_pathCleaned.at(i);
    Vector2 b = m_pathCleaned.at(i + 1);
    DrawLineV(a, b, GREEN);
  }
};

void Navmesh::GetPath(const Vector2 &start, const Vector2 &target)
{
  m_path.clear();
  m_path.push_back(start);

  const size_t startTriangleIndex = FindTriangleForPoint(start);
  const size_t targetTriangleIndex = FindTriangleForPoint(target);

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
      if (!m_triangles.at(i).ShareEdge(m_triangles.at(j)))
      {
        continue;
      }

      const double weight = static_cast<double>(Vector2Distance(m_triangles.at(i).GetCentroid(), m_triangles.at(j).GetCentroid()));
      edgeSet.insert(std::make_shared<const CXXGraph::UndirectedWeightedEdge<size_t>>(edgeId++, nodes.at(i), nodes.at(j), weight));
    }
  }

  CXXGraph::Graph<size_t> graph(edgeSet);
  const auto result = graph.dijkstra(*nodes.at(startTriangleIndex), *nodes.at(targetTriangleIndex));

  if (!result.success || result.path.empty())
  {
    if (startTriangleIndex == targetTriangleIndex)
    {
      m_path.push_back(m_triangles.at(startTriangleIndex).GetCentroid());
      m_path.push_back(target);
    }
    return;
  }

  for (const auto &nodeId : result.path)
  {
    const size_t triangleIndex = static_cast<size_t>(std::stoull(nodeId));
    if (triangleIndex < m_triangles.size())
    {
      m_path.push_back(m_triangles.at(triangleIndex).GetCentroid());
    }
  }

  m_path.push_back(target);

  m_pathCleaned.clear();

  std::vector<size_t> corridor;
  corridor.reserve(m_path.size());
  for (size_t i = 1; i + 1 < m_path.size(); ++i)
  {
    const size_t triangleIndex = FindTriangleForPoint(m_path.at(i));
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

  auto addIfDifferent = [&](const Vector2 &point)
  {
    if (m_pathCleaned.empty() || !Vector2Equals(m_pathCleaned.back(), point))
    {
      m_pathCleaned.push_back(point);
    }
  };

  auto sharedEdge = [&](size_t lhs, size_t rhs, Vector2 &outA, Vector2 &outB) -> bool
  {
    auto lhsVertices = m_triangles.at(lhs).GetVertices();
    auto rhsVertices = m_triangles.at(rhs).GetVertices();

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
    Vector2 edgeA;
    Vector2 edgeB;

    if (!sharedEdge(corridor.at(i), corridor.at(i + 1), edgeA, edgeB))
    {
      m_pathCleaned = m_path;
      return;
    }

    const Vector2 from = m_triangles.at(corridor.at(i)).GetCentroid();
    const Vector2 to = m_triangles.at(corridor.at(i + 1)).GetCentroid();
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

  Vector2 portalApex = portals.at(0).left;
  Vector2 portalLeft = portals.at(0).left;
  Vector2 portalRight = portals.at(0).right;
  size_t apexIndex = 0;
  size_t leftIndex = 0;
  size_t rightIndex = 0;

  addIfDifferent(portalApex);

  for (size_t i = 1; i < portals.size(); ++i)
  {
    const Vector2 left = portals.at(i).left;
    const Vector2 right = portals.at(i).right;

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

  addIfDifferent(m_path.back());
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
