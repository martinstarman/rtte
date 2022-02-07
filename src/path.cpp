#include "path.h"

#include <Point.h>
#include <Polygon.h>
#include <SDL.h>
#include <vector>

namespace rtte
{
    Path::Path()
        : m_pathFinder()
    {
        NavMesh::Polygon polygon;
        polygon.AddPoint(25, 25);
        polygon.AddPoint(50, 25);
        polygon.AddPoint(50, 50);
        polygon.AddPoint(25, 50);

        std::vector<NavMesh::Polygon> polygons;
        polygons.emplace_back(polygon);

        m_pathFinder.AddPolygons(polygons, 0);
    }

    Path::~Path()
    {
    }

    void Path::Find(int x1, int y1, int x2, int y2)
    {
        NavMesh::Point start(x1, y1);
        NavMesh::Point end(x2, y2);

        m_pathFinder.AddExternalPoints({start, end});

        std::vector<NavMesh::Point> path = m_pathFinder.GetPath(start, end);

        for (const auto point : path)
        {
            SDL_Log("%ix%i", point.x, point.y);
        }
    }
}
