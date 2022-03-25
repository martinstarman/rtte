#include "game.h"

#include <algorithm>
#include "enemy.h"
#include "character.h"
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
          m_leftMouseButtonDown{-1, -1},
          m_rightMouseButtonDown{-1, -1},
          m_windowSize{800, 600},
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
            .entities = {},
        };
    }

    void Game::SetGameData(const GameData &gameData)
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
        HandleKeyboardState();
        HandleMouseState();

        for (const auto entity : m_gameData.entities)
        {
            entity->Update(dt);
        }
    }

    void Game::Render(float dt)
    {
        SDL_SetRenderDrawColor(m_renderer, 0, 0, 0, 255);
        SDL_RenderClear(m_renderer);

        // entities
        for (const auto entity : m_gameData.entities)
        {
            entity->Render();
        }

        // selection rect
        if (m_leftMouseButtonDown.x != -1 && m_leftMouseButtonDown.y != -1)
        {
            int minx = std::min(m_mouse.x, m_leftMouseButtonDown.x);
            int miny = std::min(m_mouse.y, m_leftMouseButtonDown.y);
            int maxx = std::max(m_mouse.x, m_leftMouseButtonDown.x);
            int maxy = std::max(m_mouse.y, m_leftMouseButtonDown.y);

            SDL_Rect rect = {minx, miny, maxx - minx, maxy - miny};
            SDL_RenderDrawRect(m_renderer, &rect);
        }

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
                    NavMesh::Point p1 = polygon[i];
                    NavMesh::Point p2 = polygon[(i + 1) % size];
                    SDL_Point p3 = ToRenderPos(p1.x, p1.y);
                    SDL_Point p4 = ToRenderPos(p2.x, p2.y);

                    SDL_RenderDrawLine(m_renderer, p3.x, p3.y, p4.x, p4.y);
                }
            }

            // offset rects
            SDL_RenderDrawRect(m_renderer, &m_topRect);
            SDL_RenderDrawRect(m_renderer, &m_rightRect);
            SDL_RenderDrawRect(m_renderer, &m_bottomRect);
            SDL_RenderDrawRect(m_renderer, &m_leftRect);

            // fps
            // TODO: refactor
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

    SDL_Point Game::ToRenderPos(int x, int y)
    {
        return SDL_Point{x - (int)m_offset.x, y - (int)m_offset.y};
    }

    SDL_Point Game::ToGamePos(SDL_Point pos)
    {
        return SDL_Point{pos.x + (int)m_offset.x, pos.y + (int)m_offset.y};
    }

    bool Game::GetRunning()
    {
        return m_running;
    };

    void Game::HandleKeyboardState()
    {
        const uint8_t *state = SDL_GetKeyboardState(nullptr);

        // map movement
        if (state[SDL_SCANCODE_LEFT] || SDL_PointInRect(&m_mouse, &m_leftRect))
        {
            if (m_offset.x > 0)
            {
                m_offset.x -= 0.05f;
            }
        }

        if (state[SDL_SCANCODE_RIGHT] || SDL_PointInRect(&m_mouse, &m_rightRect))
        {
            if (m_offset.x <= m_gameData.mapWidth - m_windowSize.x)
            {
                m_offset.x += 0.05f;
            }
        }

        if (state[SDL_SCANCODE_UP] || SDL_PointInRect(&m_mouse, &m_topRect))
        {
            if (m_offset.y >= 0)
            {
                m_offset.y -= 0.05f;
            }
        }

        if (state[SDL_SCANCODE_DOWN] || SDL_PointInRect(&m_mouse, &m_bottomRect))
        {
            if (m_offset.y <= m_gameData.mapHeight - m_windowSize.y)
            {
                m_offset.y += 0.05f;
            }
        }

        // esc
        if (state[SDL_SCANCODE_ESCAPE])
        {
            m_running = false;
        }
    }

    void Game::HandleMouseState()
    {
        const uint32_t mouseState = SDL_GetMouseState(&m_mouse.x, &m_mouse.y);

        // left mouse button
        if ((mouseState & SDL_BUTTON_LMASK) != 0) // down
        {
            if (m_leftMouseButtonDown.x == -1 && m_leftMouseButtonDown.y == -1)
            {
                m_leftMouseButtonDown.x = m_mouse.x;
                m_leftMouseButtonDown.y = m_mouse.y;
            }
        }
        else // up
        {
            if (m_mouse.x == m_leftMouseButtonDown.x && m_mouse.y == m_leftMouseButtonDown.y)
            {
                HandleLeftMouseButtonClick();
            }
            else if (m_leftMouseButtonDown.x != -1 && m_leftMouseButtonDown.y != -1)
            {
                HandleLeftMouseButtonArea();
            }

            m_leftMouseButtonDown.x = -1;
            m_leftMouseButtonDown.y = -1;
        }

        // right mouse button
        if ((mouseState & SDL_BUTTON_RMASK) != 0) // down
        {
            if (m_rightMouseButtonDown.x == -1 && m_rightMouseButtonDown.y == -1)
            {
                m_rightMouseButtonDown.x = m_mouse.x;
                m_rightMouseButtonDown.y = m_mouse.y;
            }
        }
        else // up
        {
            if (m_mouse.x == m_rightMouseButtonDown.x && m_mouse.y == m_rightMouseButtonDown.y)
            {
                HandleRightMouseButtonClick();
            }

            m_rightMouseButtonDown.x = -1;
            m_rightMouseButtonDown.y = -1;
        }
    }

    void Game::HandleLeftMouseButtonClick()
    {
        SDL_Point mouse = ToGamePos(m_mouse);

        // characters
        int selectedCharacterIndex = -1;

        for (int i = 0; i < m_gameData.characters.size(); i++)
        {
            Character *character = m_gameData.characters.at(i);
            SDL_Rect entityRect = character->GetRect();

            if (SDL_PointInRect(&mouse, &entityRect))
            {
                selectedCharacterIndex = i;
            }
        }

        if (selectedCharacterIndex == -1)
        {
            for (const auto character : m_gameData.characters)
            {
                if (character->Selected())
                {
                    character->FindPath(mouse.x, mouse.y);
                }
            }
        }
        else
        {
            for (int i = 0; i < m_gameData.characters.size(); i++)
            {
                Entity *character = m_gameData.characters.at(i);

                if (i == selectedCharacterIndex)
                {
                    character->Select();
                }
                else
                {
                    character->Deselect();
                }
            }
        }

        // enemies
        int selectedEnemyIndex = -1;

        for (int i = 0; i < m_gameData.enemies.size(); i++)
        {
            Enemy *enemy = m_gameData.enemies.at(i);
            SDL_Rect entityRect = enemy->GetRect();

            if (SDL_PointInRect(&mouse, &entityRect))
            {
                selectedEnemyIndex = i;
            }
        }

        if (selectedEnemyIndex > -1)
        {
            for (int i = 0; i < m_gameData.enemies.size(); i++)
            {
                Enemy *enemy = m_gameData.enemies.at(i);

                if (i == selectedEnemyIndex)
                {
                    enemy->Select();
                }
                else
                {
                    enemy->Deselect();
                }
            }
        }
    }

    void Game::HandleLeftMouseButtonArea()
    {
        std::vector<int> selectedCharacterIndex;
        SDL_Point mouse = ToGamePos(m_mouse);
        SDL_Point leftMouseButtonDown = ToGamePos(m_leftMouseButtonDown);

        int minx = std::min(mouse.x, leftMouseButtonDown.x);
        int miny = std::min(mouse.y, leftMouseButtonDown.y);
        int maxx = std::max(mouse.x, leftMouseButtonDown.x);
        int maxy = std::max(mouse.y, leftMouseButtonDown.y);

        SDL_Rect rect = {minx, miny, maxx - minx, maxy - miny};

        for (int i = 0; i < m_gameData.characters.size(); i++)
        {
            Character *character = m_gameData.characters.at(i);
            SDL_Point pos{(int)character->GetX(), (int)character->GetY()};

            if (SDL_PointInRect(&pos, &rect))
            {
                selectedCharacterIndex.emplace_back(i);
            }
        }

        // do not deselect entities when no character is in selection area
        if (selectedCharacterIndex.size() > 0)
        {
            for (int i = 0; i < m_gameData.characters.size(); i++)
            {
                Character *character = m_gameData.characters.at(i);

                if (std::find(selectedCharacterIndex.begin(), selectedCharacterIndex.end(), i) ==
                    selectedCharacterIndex.end())
                {
                    character->Deselect();
                }
                else
                {
                    character->Select();
                }
            }
        }
    }

    void Game::HandleRightMouseButtonClick()
    {
        for (const auto entity : m_gameData.entities)
        {
            if (entity->Selected())
            {
                entity->Deselect();
            }
        }
    }
}
