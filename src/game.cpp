#include "game.h"

#include "entity.h"
#include <SDL.h>

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
          m_entity(new Entity())
    {
        m_window = SDL_CreateWindow("RTTE", SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED,
                                    800, 600, SDL_WINDOW_SHOWN);
        m_renderer = SDL_CreateRenderer(m_window, -1, SDL_RENDERER_ACCELERATED);
        SDL_SetRenderDrawBlendMode(m_renderer, SDL_BLENDMODE_BLEND);
    }

    Game::~Game()
    {
        SDL_DestroyRenderer(m_renderer);
        SDL_DestroyWindow(m_window);
        delete m_entity;
        delete s_instance;
    }

    void Game::Render()
    {
        SDL_SetRenderDrawColor(m_renderer, 0, 0, 0, 255);
        SDL_RenderClear(m_renderer);
        m_entity->Render();
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

    void Game::HandleClick(int x, int y)
    {
        m_entity->FindPath(x, y);
    }
}
