#include "entity.h"

#include "game.h"
#include "point.h"
#include "polygon.h"
#include <SDL.h>
#include "util.h"
#include <vector>

namespace rtte
{
    Entity::Entity(float x, float y)
        : m_x(x),
          m_y(y),
          m_selected(false),
          m_pathFinder(),
          m_path({})
    {
    }

    Entity::~Entity()
    {
    }

    void Entity::Update(float dt)
    {
        Move(dt);
    }

    void Entity::Move(float dt)
    {
        if (m_path.size() > 0)
        {
            SDL_Point next = m_path.at(0);
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
            SDL_SetRenderDrawColor(Game::Get()->GetRenderer(), 255, 255, 255, 255);

            // position
            SDL_Point pos = Game::Get()->ToRenderPos((int)m_x, (int)m_y);

            SDL_RenderDrawLine(Game::Get()->GetRenderer(), pos.x, pos.y - 10, pos.x, pos.y + 10);
            SDL_RenderDrawLine(Game::Get()->GetRenderer(), pos.x - 10, pos.y, pos.x + 10, pos.y);

            // rect
            if (m_selected)
            {
                SDL_Rect rect = {pos.x - 10, pos.y - 10, 20, 20}; // TODO: width, height
                SDL_RenderDrawRect(Game::Get()->GetRenderer(), &rect);
            }

            // path
            int size = (int)m_path.size();

            if (size > 0)
            {
                SDL_Point next = m_path.at(0);
                next = Game::Get()->ToRenderPos(next.x, next.y);
                SDL_RenderDrawLine(Game::Get()->GetRenderer(), pos.x, pos.y, next.x, next.y);

                for (int i = 0; i < size - 1; i++)
                {
                    SDL_Point p1 = m_path.at(i);
                    SDL_Point p2 = m_path.at(i + 1);
                    p1 = Game::Get()->ToRenderPos(p1.x, p1.y);
                    p2 = Game::Get()->ToRenderPos(p2.x, p2.y);

                    SDL_RenderDrawLine(Game::Get()->GetRenderer(), p1.x, p1.y, p2.x, p2.y);
                }
            }
        }
    }

    void Entity::FindPath(int x, int y)
    {
        // TODO: call only when polygons changed
        // TODO: inflation
        m_pathFinder.AddPolygons(Game::Get()->GetGameData().polygons, 0);

        NavMesh::Point start((int)m_x, (int)m_y);
        NavMesh::Point end(x, y);
        m_pathFinder.AddExternalPoints({start, end});
        std::vector<NavMesh::Point> path = m_pathFinder.GetPath(start, end);
        m_path.clear();

        // first position equals entity position
        for (int i = 1; i < path.size(); i++)
        {
            NavMesh::Point point = path.at(i);
            m_path.emplace_back(SDL_Point{point.x, point.y});
        }
    }

    void Entity::RemovePath()
    {
        m_path.clear();
    }

    SDL_Rect Entity::GetRect()
    {
        return SDL_Rect{(int)m_x - 10, (int)m_y - 10, 20, 20}; // TODO: width, height
    }

    void Entity::Select()
    {
        m_selected = true;
    }

    void Entity::Deselect()
    {
        m_selected = false;
    }

    bool Entity::Selected()
    {
        return m_selected;
    }

    float Entity::GetX()
    {
        return m_x;
    }

    float Entity::GetY()
    {
        return m_y;
    }
}