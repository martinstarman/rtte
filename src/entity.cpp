#include "entity.h"

#include "game.h"
#include "point.h"
#include "polygon.h"
#include <SDL.h>
#include "util.h"
#include <vector>

namespace rtte
{
    Entity::Entity(float x, float y, const std::vector<NavMesh::Polygon> &polygons)
        : m_x(x),
          m_y(y),
          m_pathFinder(),
          m_path({})
    {
        m_pathFinder.AddPolygons(polygons, 0);
    }

    Entity::~Entity()
    {
    }

    void Entity::Update(float dt)
    {
        // movement
        if (m_path.size() > 0)
        {
            NavMesh::Point next = m_path.at(0);
            float dx = next.x - m_x;
            float dy = next.y - m_y;
            float dist = util::Distance(m_x, m_y, (float)next.x, (float)next.y);

            m_x += dx * dt / dist;
            m_y += dy * dt / dist;

            dist = util::Distance(m_x, m_y, (float)next.x, (float)next.y);

            if (dist <= 1.0f)
            {
                m_path.erase(m_path.begin());
                m_x = (float)next.x;
                m_y = (float)next.y;
            }
        }
    }

    void Entity::Render()
    {
        // TODO: render entity

        // debug
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

            if (size > 0)
            {
                NavMesh::Point next = Game::Get()->ToRenderPos(m_path.at(0));
                SDL_RenderDrawLine(Game::Get()->GetRenderer(), pos.x, pos.y, next.x, next.y);

                for (int i = 0; i < size - 1; i++)
                {
                    NavMesh::Point p1 = Game::Get()->ToRenderPos(m_path.at(i));
                    NavMesh::Point p2 = Game::Get()->ToRenderPos(m_path.at(i + 1));

                    SDL_RenderDrawLine(Game::Get()->GetRenderer(), p1.x, p1.y, p2.x, p2.y);
                }
            }
        }
    }

    void Entity::FindPath(int x, int y)
    {
        NavMesh::Point start((int)m_x, (int)m_y);
        NavMesh::Point end(x, y);
        m_pathFinder.AddExternalPoints({start, end});
        m_path = m_pathFinder.GetPath(start, end);

        if (m_path.size() > 0)
        {
            m_path.erase(m_path.begin()); // first position equals entity position
        }
    }

    void Entity::RemovePath()
    {
        m_path.clear();
    }
}