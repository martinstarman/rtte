#include "entity.h"

#include "game.h"
#include "point.h"
#include "polygon.h"
#include <SDL.h>
#include <vector>

namespace rtte
{
    Entity::Entity()
        : m_x(0.0f),
          m_y(0.0f),
          m_pathFinder(),
          m_path({})
    {
        // TODO: move it to the game
        NavMesh::Polygon polygon;
        polygon.AddPoint(25, 25);
        polygon.AddPoint(50, 25);
        polygon.AddPoint(50, 50);
        polygon.AddPoint(25, 50);

        std::vector<NavMesh::Polygon> polygons;
        polygons.emplace_back(polygon);

        m_pathFinder.AddPolygons(polygons, 0);
    }

    Entity::~Entity()
    {
    }

    void Entity::Render()
    {
        // TODO: render entity

        if (Game::Get()->GetDebug())
        {
            if (m_path.size() > 0)
            {
                std::vector<SDL_Point> points;

                for (int i = 0; i < m_path.size(); i++)
                {
                    NavMesh::Point point = m_path.at(i);
                    points.emplace_back(SDL_Point{point.x, point.y});
                }

                SDL_SetRenderDrawColor(Game::Get()->GetRenderer(), 255, 255, 255, 64);
                SDL_RenderDrawLines(Game::Get()->GetRenderer(), &points[0], (int)m_path.size());
            }
        }
    }

    void Entity::FindPath(int x, int y)
    {
        NavMesh::Point start((int)m_x, (int)m_y);
        NavMesh::Point end(x, y);

        m_pathFinder.AddExternalPoints({start, end});

        m_path = m_pathFinder.GetPath(start, end);
    }
}