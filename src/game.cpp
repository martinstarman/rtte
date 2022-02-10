#include "game.h"

#include "entity.h"
#include <point.h>
#include <polygon.h>
#include <SDL.h>
#include <vector>

namespace rtte
{
    Game *Game::s_instance = nullptr;

    Game *Game::Get()
    {
        if (s_instance == nullptr)
        {
            s_instance = new Game();
        }

        return s_instance;
    }

    Game::Game()
        : m_debug(false),
          m_mouse{0, 0},
          m_windowSize(800, 600),
          m_mapSize(1200, 900),
          m_offset(0.0f, 0.0f)
    {
        m_window = SDL_CreateWindow("RTTE", SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED,
                                    m_windowSize.x, m_windowSize.y, SDL_WINDOW_SHOWN);
        m_renderer = SDL_CreateRenderer(m_window, -1, SDL_RENDERER_ACCELERATED);
        SDL_SetRenderDrawBlendMode(m_renderer, SDL_BLENDMODE_BLEND);
        SDL_SetWindowMouseGrab(m_window, SDL_TRUE);

        NavMesh::Polygon polygon1;
        polygon1.AddPoint(225, 225);
        polygon1.AddPoint(250, 225);
        polygon1.AddPoint(250, 250);
        polygon1.AddPoint(225, 250);
        m_polygons.emplace_back(polygon1);

        NavMesh::Polygon polygon2;
        polygon2.AddPoint(275, 225);
        polygon2.AddPoint(300, 225);
        polygon2.AddPoint(300, 250);
        polygon2.AddPoint(275, 250);
        m_polygons.emplace_back(polygon2);

        NavMesh::Polygon polygon3;
        polygon3.AddPoint(225, 275);
        polygon3.AddPoint(250, 275);
        polygon3.AddPoint(250, 300);
        polygon3.AddPoint(225, 300);
        m_polygons.emplace_back(polygon3);

        NavMesh::Polygon polygon4;
        polygon4.AddPoint(275, 275);
        polygon4.AddPoint(300, 275);
        polygon4.AddPoint(300, 300);
        polygon4.AddPoint(275, 300);
        m_polygons.emplace_back(polygon4);

        NavMesh::Polygon polygon5;
        polygon5.AddPoint(1100, 800);
        polygon5.AddPoint(1150, 850);
        polygon5.AddPoint(1050, 850);
        m_polygons.emplace_back(polygon5);

        m_entity = new Entity(m_polygons);

        int rectWidth = 20;
        m_topRect = SDL_Rect{0, 0, m_windowSize.x, rectWidth};
        m_rightRect = SDL_Rect{m_windowSize.x - rectWidth, 0, rectWidth, m_windowSize.y};
        m_bottomRect = SDL_Rect{0, m_windowSize.y - rectWidth, m_windowSize.x, rectWidth};
        m_leftRect = SDL_Rect{0, 0, rectWidth, m_windowSize.y};
    }

    Game::~Game()
    {
        SDL_DestroyRenderer(m_renderer);
        SDL_DestroyWindow(m_window);
        delete m_entity;
        delete s_instance;
    }

    void Game::Update()
    {
        SDL_PumpEvents();
        const uint8_t *keyState = SDL_GetKeyboardState(nullptr);
        const uint32_t mouseState = SDL_GetMouseState(&m_mouse.x, &m_mouse.y);
        float step = 0.05f;

        if (m_offset.x >= 0 &&
            (keyState[SDL_SCANCODE_LEFT] || SDL_PointInRect(&m_mouse, &m_leftRect)))
        {
            m_offset.x -= step;
        }

        if (m_offset.x <= m_mapSize.x - m_windowSize.x &&
            (keyState[SDL_SCANCODE_RIGHT] || SDL_PointInRect(&m_mouse, &m_rightRect)))
        {
            m_offset.x += step;
        }

        if (m_offset.y >= 0 &&
            (keyState[SDL_SCANCODE_UP] || SDL_PointInRect(&m_mouse, &m_topRect)))
        {
            m_offset.y -= step;
        }

        if (m_offset.y <= m_mapSize.y - m_windowSize.y &&
            (keyState[SDL_SCANCODE_DOWN] || SDL_PointInRect(&m_mouse, &m_bottomRect)))
        {
            m_offset.y += step;
        }

        if ((mouseState & SDL_BUTTON_LMASK) != 0)
        {
            NavMesh::Point pos = ToGamePos(m_mouse);
            m_entity->FindPath(pos.x, pos.y);
        }

        Render();
    }

    void Game::Render()
    {
        SDL_SetRenderDrawColor(m_renderer, 0, 0, 0, 255);
        SDL_RenderClear(m_renderer);

        m_entity->Render();

        if (m_debug)
        {
            SDL_SetRenderDrawColor(m_renderer, 255, 255, 255, 64);

            // polygons
            for (const auto polygon : m_polygons)
            {
                int size = polygon.Size();

                for (int i = 0; i < size; i++)
                {
                    NavMesh::Point p1 = ToRenderPos(polygon[i]);
                    NavMesh::Point p2 = ToRenderPos(polygon[(i + 1) % size]);

                    SDL_RenderDrawLine(m_renderer, p1.x, p1.y, p2.x, p2.y);
                }
            }

            // offset rects
            SDL_RenderDrawRect(m_renderer, &m_topRect);
            SDL_RenderDrawRect(m_renderer, &m_rightRect);
            SDL_RenderDrawRect(m_renderer, &m_bottomRect);
            SDL_RenderDrawRect(m_renderer, &m_leftRect);
        }

        SDL_RenderPresent(m_renderer);
    }

    void Game::SetDebug()
    {
        m_debug = true;
    }

    bool Game::GetDebug()
    {
        return m_debug;
    }

    SDL_Renderer *Game::GetRenderer()
    {
        return m_renderer;
    }

    NavMesh::Point Game::ToRenderPos(NavMesh::Point pos)
    {
        return NavMesh::Point(pos.x - (int)m_offset.x, pos.y - (int)m_offset.y);
    }

    NavMesh::Point Game::ToGamePos(SDL_Point pos)
    {
        return NavMesh::Point(pos.x + (int)m_offset.x, pos.y + (int)m_offset.y);
    }
}
