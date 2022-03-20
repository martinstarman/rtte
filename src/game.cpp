#include "game.h"

#include "entity.h"
#include <point.h>
#include <polygon.h>
#include <SDL.h>
#include <SDL_ttf.h>
#include "util.h"
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
        : m_mouse{0, 0},
          m_windowSize(800, 600),
          m_offset(0.0f, 0.0f),
          m_running(true)
    {
        m_window = SDL_CreateWindow("RTTE", SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED,
                                    m_windowSize.x, m_windowSize.y, SDL_WINDOW_SHOWN);
        m_renderer = SDL_CreateRenderer(m_window, -1, SDL_RENDERER_ACCELERATED);
        SDL_SetRenderDrawBlendMode(m_renderer, SDL_BLENDMODE_BLEND);
        SDL_SetWindowMouseGrab(m_window, SDL_TRUE);

        m_font = TTF_OpenFont(util::GetFontPath().c_str(), 14);

        int rectWidth = 20;
        m_topRect = SDL_Rect{0, 0, m_windowSize.x, rectWidth};
        m_rightRect = SDL_Rect{m_windowSize.x - rectWidth, 0, rectWidth, m_windowSize.y};
        m_bottomRect = SDL_Rect{0, m_windowSize.y - rectWidth, m_windowSize.x, rectWidth};
        m_leftRect = SDL_Rect{0, 0, rectWidth, m_windowSize.y};

        // defaults
        m_gameData = {
            .debug = false,
            .missionName = "",
            .mapWidth = m_windowSize.x,
            .mapHeight = m_windowSize.y,
            .polygons = {},
        };
    }

    void Game::SetGameData(GameData gameData)
    {
        m_gameData = gameData;
    }

    Game::~Game()
    {
        SDL_DestroyRenderer(m_renderer);
        SDL_DestroyWindow(m_window);
        TTF_CloseFont(m_font);

        for (auto entity : m_gameData.entities)
        {
            delete entity;
        }

        delete s_instance;
    }

    void Game::Update(float dt)
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

        if (m_offset.x <= m_gameData.mapWidth - m_windowSize.x &&
            (keyState[SDL_SCANCODE_RIGHT] || SDL_PointInRect(&m_mouse, &m_rightRect)))
        {
            m_offset.x += step;
        }

        if (m_offset.y >= 0 &&
            (keyState[SDL_SCANCODE_UP] || SDL_PointInRect(&m_mouse, &m_topRect)))
        {
            m_offset.y -= step;
        }

        if (m_offset.y <= m_gameData.mapHeight - m_windowSize.y &&
            (keyState[SDL_SCANCODE_DOWN] || SDL_PointInRect(&m_mouse, &m_bottomRect)))
        {
            m_offset.y += step;
        }

        if (keyState[SDL_SCANCODE_ESCAPE])
        {
            m_running = false;
        }

        if ((mouseState & SDL_BUTTON_LMASK) != 0)
        {
            NavMesh::Point pos = ToGamePos(m_mouse);
            // TODO: selected character
            m_gameData.entities.at(0)->FindPath(pos.x, pos.y);
        }

        if ((mouseState & SDL_BUTTON_RMASK) != 0)
        {
            // TODO: selected character
            m_gameData.entities.at(0)->RemovePath();
        }

        for (const auto entity : m_gameData.entities)
        {
            entity->Update(dt);
        }
    }

    void Game::Render(float dt)
    {
        SDL_SetRenderDrawColor(m_renderer, 0, 0, 0, 255);
        SDL_RenderClear(m_renderer);

        m_gameData.entities.at(0)->Render();

        // debug
        if (GetDebug())
        {
            SDL_SetRenderDrawColor(m_renderer, 255, 255, 255, 64);

            // polygons
            for (const auto polygon : m_gameData.polygons)
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

            //
            std::string fps = "fps: " + std::to_string(1 / dt);
            SDL_Surface *surface = TTF_RenderText_Solid(m_font, fps.c_str(), {255, 255, 255, 128});
            SDL_Texture *texture = SDL_CreateTextureFromSurface(m_renderer, surface);
            int w, h;
            SDL_QueryTexture(texture, nullptr, nullptr, &w, &h);
            SDL_Rect dest{30, 30, w, h};
            SDL_RenderCopy(m_renderer, texture, nullptr, &dest);
            SDL_DestroyTexture(texture);
            SDL_FreeSurface(surface);
        }

        SDL_RenderPresent(m_renderer);
    }

    bool Game::GetDebug()
    {
        return m_gameData.debug;
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

    bool Game::GetRunning()
    {
        return m_running;
    };
}
