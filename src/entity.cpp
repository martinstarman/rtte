#include "entity.h"

#include "game.h"
#include "point.h"
#include "polygon.h"
#include <SDL.h>
#include <vector>

namespace rtte
{
    Entity::Entity(const std::vector<NavMesh::Polygon> &polygons)
        : m_x(20.0f),
          m_y(20.0f),
          m_pathFinder(),
          m_path({})
    {
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
            SDL_SetRenderDrawColor(Game::Get()->GetRenderer(), 255, 255, 255, 64);

            // position
            NavMesh::Point pos((int)m_x, (int)m_y);
            pos = Game::Get()->ToRenderPos(pos);

            SDL_RenderDrawLine(Game::Get()->GetRenderer(), pos.x, pos.y - 10, pos.x, pos.y + 10);
            SDL_RenderDrawLine(Game::Get()->GetRenderer(), pos.x - 10, pos.y, pos.x + 10, pos.y);

            // path
            int size = (int)m_path.size();

            for (int i = 0; i < size - 1; i++)
            {
                NavMesh::Point p1 = Game::Get()->ToRenderPos(m_path.at(i));
                NavMesh::Point p2 = Game::Get()->ToRenderPos(m_path.at(i + 1));

                SDL_RenderDrawLine(Game::Get()->GetRenderer(), p1.x, p1.y, p2.x, p2.y);
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