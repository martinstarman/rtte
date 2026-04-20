#include "navmesh.h"

Navmesh::Navmesh() {};

Navmesh::~Navmesh() {};

void Navmesh::Build()
{
  // The number type to use for tessellation
  using Coord = float;

  // The index type. Defaults to uint32_t, but you can also pass uint16_t if you know that your
  // data won't have more than 65536 vertices.
  using N = uint32_t;

  // Create array
  using Point = std::array<Coord, 2>;
  std::vector<std::vector<Point>> polygon;

  // Fill polygon structure with actual data. Any winding order works.
  // The first polyline defines the main polygon.
  polygon.push_back({{100, 0}, {100, 100}, {0, 100}, {0, 0}});
  // Following polylines define holes.
  polygon.push_back({{75, 25}, {75, 75}, {25, 75}, {25, 25}});

  // Run tessellation
  // Returns array of indices that refer to the vertices of the input polygon.
  // e.g: the index 6 would refer to {25, 75} in this example.
  // Three subsequent indices form a triangle. Output triangles are clockwise.
  std::vector<N> indices = mapbox::earcut<N>(polygon);

  TraceLog(LOG_INFO, std::to_string(polygon.size()).c_str());
  TraceLog(LOG_INFO, std::to_string(polygon.at(0).size()).c_str());
  TraceLog(LOG_INFO, std::to_string(polygon.at(1).size()).c_str());
  TraceLog(LOG_INFO, std::to_string(indices.size()).c_str());
  
  for (int i = 0; i < indices.size(); i++) {
    
    TraceLog(LOG_INFO, std::to_string(indices.at(i)).c_str());
  }
  for (int i = 0; i < indices.size(); i++)
  {
    auto index = indices.at(i);
    Point p;

    if (index < 4)
    {
      p = polygon.at(0).at(index);
    }
    else
    {
      p = polygon.at(1).at(index - polygon.at(0).size());
    }

    m_triangles.emplace_back(Vector2{p.at(0), p.at(1)});
  }
};

void Navmesh::Draw()
{
  for (int i = 0; i < m_triangles.size(); i += 3)
  {
    Vector2 a = m_triangles[i];
    Vector2 b = m_triangles[i + 1];
    Vector2 c = m_triangles[i + 2];

    DrawLineV(a, b, WHITE);
    DrawLineV(b, c, WHITE);
    DrawLineV(c, a, WHITE);
  }
};
