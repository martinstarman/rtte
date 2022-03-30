#include "enemy.h"

#include "entity.h"
#include "game.h"
#include <SDL.h>
#include <vector>

namespace rtte
{
    Enemy::Enemy(float x, float y)
        : Entity(x, y),
          m_cone()
    {
    }

    Enemy::~Enemy()
    {
    }

    void Enemy::Update(float dt)
    {
        m_cone.AddPolygons(Game::Get()->GetGameData().polygons);

        m_vision = m_cone.GetVision(NavMesh::Point((int)m_x, (int)m_y), 100, 120, 210);
        m_vision.insert(m_vision.begin(), NavMesh::PointF(m_x, m_y));
        m_vision.emplace_back(NavMesh::PointF(m_x, m_y));

        Entity::Update(dt);
    }

    void Enemy::Render()
    {
        int size = (int)m_vision.size();

        for (int i = 0; i < size; i++)
        {
            NavMesh::PointF p1 = m_vision.at(i);
            NavMesh::PointF p2 = m_vision.at((i + 1) % size);
            SDL_Point p3 = Game::Get()->ToRenderPos((int)p1.x, (int)p1.y);
            SDL_Point p4 = Game::Get()->ToRenderPos((int)p2.x, (int)p2.y);

            SDL_RenderDrawLine(Game::Get()->GetRenderer(), p3.x, p3.y, p4.x, p4.y);
        }

        Entity::Render();
    }

    int Enemy::GetYIndex()
    {
        return (int)m_y; // TODO: height
    }
}