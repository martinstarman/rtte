#include "navmesh.h"

Navmesh::Navmesh(Rectangle mapRect)
{
  m_polygons.push_back({{
      {mapRect.x, mapRect.y},
      {mapRect.x, mapRect.y + mapRect.height},
      {mapRect.x + mapRect.width, mapRect.y + mapRect.height},
      {mapRect.x + mapRect.width, mapRect.y},
  }});
};

Navmesh::~Navmesh() = default;

void Navmesh::AddHole(const std::vector<std::array<float, 2>> &hole)
{
  m_polygons.push_back(hole);
  Triangulate();
}

void Navmesh::Draw()
{
  for (const auto &t : m_triangles)
  {
    DrawLineV(t.GetA(), t.GetB(), WHITE);
    DrawLineV(t.GetB(), t.GetC(), WHITE);
    DrawLineV(t.GetC(), t.GetA(), WHITE);
  }
};

std::vector<Vector2> Navmesh::GetPath(const Vector2 &start, const Vector2 &target)
{
  const size_t startTriangleIndex = GetTriangleIndexFrom(start);
  const size_t targetTriangleIndex = GetTriangleIndexFrom(target);

  if (startTriangleIndex == targetTriangleIndex)
  {
    return {start, target};
  }

  std::vector<CXXGraph::Node<size_t>> nodes;
  nodes.reserve(m_triangles.size());

  for (size_t i = 0; i < m_triangles.size(); ++i)
  {
    nodes.emplace_back(std::to_string(i), i);
  }

  CXXGraph::T_EdgeSet<size_t> edges;
  CXXGraph::id_t edgeId = 0;

  for (size_t i = 0; i < m_triangles.size(); ++i)
  {
    for (size_t j = i + 1; j < m_triangles.size(); ++j)
    {
      if (!m_triangles.at(i).ShareEdge(m_triangles.at(j)))
      {
        continue;
      }

      const double weight = static_cast<double>(Vector2Distance(m_triangles.at(i).GetCentroid(),
                                                                m_triangles.at(j).GetCentroid()));
      edges.insert(std::make_shared<const CXXGraph::UndirectedWeightedEdge<size_t>>(edgeId++,
                                                                                    nodes.at(i),
                                                                                    nodes.at(j),
                                                                                    weight));
    }
  }

  CXXGraph::Graph<size_t> graph(edges);
  CXXGraph::DijkstraResult result = graph.dijkstra(nodes.at(startTriangleIndex),
                                                   nodes.at(targetTriangleIndex));

  if (!result.success || result.path.empty())
  {
    return {};
  }

  std::vector<Vector2> centroidsPath;
  centroidsPath.push_back(start);

  for (const auto &nodeId : result.path)
  {
    // TODO: i hate this
    const size_t triangleIndex = static_cast<size_t>(std::stoull(nodeId));
    centroidsPath.push_back(m_triangles.at(triangleIndex).GetCentroid());
  }

  centroidsPath.push_back(target);
  std::vector<Vector2> path;

  std::vector<size_t> trianglesPath;
  trianglesPath.reserve(centroidsPath.size());

  for (size_t i = 1; i + 1 < centroidsPath.size(); ++i)
  {
    const size_t triangleIndex = GetTriangleIndexFrom(centroidsPath.at(i));
    trianglesPath.push_back(triangleIndex);
  }

  // TODO
  auto addIfDifferent = [&](const Vector2 &point)
  {
    if (path.empty() || !Vector2Equals(path.back(), point))
    {
      path.push_back(point);
    }
  };

  std::vector<Portal> portals;
  portals.reserve(trianglesPath.size() + 1);
  portals.push_back(Portal{centroidsPath.front(), centroidsPath.front()});

  for (size_t i = 0; i + 1 < trianglesPath.size(); ++i)
  {
    Triangle triangle1 = m_triangles.at(trianglesPath.at(i));
    Triangle triangle2 = m_triangles.at(trianglesPath.at(i + 1));
    std::array<Vector2,2> sharedEdge = triangle1.GetSharedEdge(triangle2);

    const Vector2 from = m_triangles.at(trianglesPath.at(i)).GetCentroid();
    const Vector2 to = m_triangles.at(trianglesPath.at(i + 1)).GetCentroid();
    const float crossProduct1 = CrossProduct(from, to, sharedEdge.at(0));
    const float crossProduct2 = CrossProduct(from, to, sharedEdge.at(1));

    Portal portal;

    if (crossProduct1 >= crossProduct2)
    {
      portal.left = sharedEdge.at(1);
      portal.right = sharedEdge.at(0);
    }
    else
    {
      portal.left = sharedEdge.at(0);
      portal.right = sharedEdge.at(1);
    }

    portals.push_back(portal);
  }

  portals.push_back(Portal{centroidsPath.back(), centroidsPath.back()});

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

  addIfDifferent(centroidsPath.back());

  return path;
}

// TODO: refactor
void Navmesh::Triangulate()
{
  m_triangles.clear();

  std::vector<uint32_t> indices = mapbox::earcut<uint32_t>(m_polygons);
  std::vector<Vector2> trianglesIndices;

  std::vector<size_t> ringOffsets;
  ringOffsets.reserve(m_polygons.size());
  size_t vertexOffset = 0;
  for (const auto &ring : m_polygons)
  {
    ringOffsets.push_back(vertexOffset);
    vertexOffset += ring.size();
  }

  for (const uint32_t index : indices)
  {
    std::array<float, 2> p;
    bool found = false;

    for (size_t ringIndex = 0; ringIndex < m_polygons.size(); ++ringIndex)
    {
      const size_t start = ringOffsets.at(ringIndex);
      const size_t end = start + m_polygons.at(ringIndex).size();
      if (index >= start && index < end)
      {
        p = m_polygons.at(ringIndex).at(static_cast<size_t>(index) - start);
        found = true;
        break;
      }
    }

    if (!found)
    {
      continue;
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
}

size_t Navmesh::GetTriangleIndexFrom(const Vector2 &vector)
{
  for (size_t i = 0; i < m_triangles.size(); ++i)
  {
    if (m_triangles.at(i).Contains(vector))
    {
      return i;
    }
  }

  return -1;
}
