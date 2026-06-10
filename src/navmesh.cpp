#include "navmesh.h"

Navmesh::Navmesh(float mapWidth, float mapHeight)
{
  m_polygons.push_back({{
      {0.0, 0.0},
      {0.0, mapHeight},
      {mapWidth, mapHeight},
      {mapWidth, 0.0},
  }});
};

Navmesh::~Navmesh() = default;

void Navmesh::AddHole(const std::vector<std::array<float, 2>> &hole)
{
  m_polygons.push_back(hole);
  Triangulate();
}

// TODO: debug
void Navmesh::Draw() const
{
  for (const auto &t : m_triangles)
  {
    DrawLineV(t.GetA(), t.GetB(), WHITE);
    DrawLineV(t.GetB(), t.GetC(), WHITE);
    DrawLineV(t.GetC(), t.GetA(), WHITE);
  }
};

std::vector<Vector2> Navmesh::GetPath(const Vector2 &start, const Vector2 &target, float entityRadius) const
{
  assert(entityRadius > 0.0 && "Entity radius must be greater than zero.");

  size_t startTriangleIndex = GetTriangleIndexFrom(start);
  size_t targetTriangleIndex = GetTriangleIndexFrom(target);

  if (startTriangleIndex == targetTriangleIndex)
  {
    return {start, target};
  }

  std::vector<CXXGraph::Node<size_t>> nodes;

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

      double weight = static_cast<double>(Vector2Distance(m_triangles.at(i).GetCentroid(),
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
    size_t triangleIndex = static_cast<size_t>(std::stoull(nodeId));
    centroidsPath.push_back(m_triangles.at(triangleIndex).GetCentroid());
  }

  centroidsPath.push_back(target);
  std::vector<size_t> trianglesPath;

  for (size_t i = 1; i + 1 < centroidsPath.size(); ++i)
  {
    size_t triangleIndex = GetTriangleIndexFrom(centroidsPath.at(i));
    trianglesPath.push_back(triangleIndex);
  }

  std::vector<Vector2> path;

  auto pathPushBack = [&](const Vector2 &point)
  {
    if (path.empty() || !Vector2Equals(path.back(), point))
    {
      path.push_back(point);
    }
  };

  std::vector<Portal> portals;
  portals.push_back(Portal{centroidsPath.front(), centroidsPath.front()});

  for (size_t i = 0; i + 1 < trianglesPath.size(); ++i)
  {
    Triangle triangle1 = m_triangles.at(trianglesPath.at(i));
    Triangle triangle2 = m_triangles.at(trianglesPath.at(i + 1));
    std::array<Vector2, 2> sharedEdge = triangle1.GetSharedEdge(triangle2);

    Vector2 from = m_triangles.at(trianglesPath.at(i)).GetCentroid();
    Vector2 to = m_triangles.at(trianglesPath.at(i + 1)).GetCentroid();
    float crossProduct1 = CrossProduct(from, to, sharedEdge.at(0));
    float crossProduct2 = CrossProduct(from, to, sharedEdge.at(1));

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

    if (entityRadius > 0.0f)
    {
      const float edgeX = portal.right.x - portal.left.x;
      const float edgeY = portal.right.y - portal.left.y;
      const float edgeLength = std::sqrt((edgeX * edgeX) + (edgeY * edgeY));

      if (edgeLength <= (2.0f * entityRadius))
      {
        return {};
      }

      const float dirX = edgeX / edgeLength;
      const float dirY = edgeY / edgeLength;
      portal.left.x += dirX * entityRadius;
      portal.left.y += dirY * entityRadius;
      portal.right.x -= dirX * entityRadius;
      portal.right.y -= dirY * entityRadius;
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

  pathPushBack(portalApex);

  for (size_t i = 1; i < portals.size(); ++i)
  {
    Vector2 left = portals.at(i).left;
    Vector2 right = portals.at(i).right;

    if (CrossProduct(portalApex, portalRight, right) <= 0.0f)
    {
      if (Vector2Equals(portalApex, portalRight) || CrossProduct(portalApex, portalLeft, right) > 0.0f)
      {
        portalRight = right;
        rightIndex = i;
      }
      else
      {
        pathPushBack(portalLeft);

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
        pathPushBack(portalRight);

        portalApex = portalRight;
        apexIndex = rightIndex;
        portalLeft = portalApex;
        leftIndex = apexIndex;
        i = apexIndex;
        continue;
      }
    }
  }

  pathPushBack(centroidsPath.back());

  return path;
}

void Navmesh::Triangulate()
{
  m_triangles.clear();

  std::vector<uint32_t> indices = mapbox::earcut<uint32_t>(m_polygons);
  std::vector<Vector2> trianglesIndices;
  std::vector<size_t> polygonOffsets;
  size_t currentPolygonOffset = 0;

  for (const auto &polygon : m_polygons)
  {
    polygonOffsets.push_back(currentPolygonOffset);
    currentPolygonOffset += polygon.size();
  }

  for (const uint32_t index : indices)
  {
    std::array<float, 2> p;
    bool found = false;

    for (size_t i = 0; i < m_polygons.size(); ++i)
    {
      size_t start = polygonOffsets.at(i);
      size_t end = start + m_polygons.at(i).size();
      if (index >= start && index < end)
      {
        p = m_polygons.at(i).at(static_cast<size_t>(index) - start);
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

  for (size_t i = 0; i < trianglesIndices.size() - 2; i += 3)
  {
    Triangle triangle = Triangle(
        trianglesIndices.at(i),
        trianglesIndices.at(i + 1),
        trianglesIndices.at(i + 2));
    m_triangles.push_back(triangle);
  }
}

size_t Navmesh::GetTriangleIndexFrom(const Vector2 &v) const
{
  for (size_t i = 0; i < m_triangles.size(); ++i)
  {
    if (m_triangles.at(i).Contains(v))
    {
      return i;
    }
  }

  return -1;
}
