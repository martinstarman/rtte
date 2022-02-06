#include "window.h"

#include <SDL.h>

namespace rtte
{
    Window::Window(int width, int height, bool debug)
        : m_width(width),
          m_height(height),
          m_debug(debug)
    {
        m_window = SDL_CreateWindow("RTTE", SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED,
                                    width, height, SDL_WINDOW_SHOWN);
        m_renderer = SDL_CreateRenderer(m_window, -1, SDL_RENDERER_ACCELERATED);

        if (debug)
        {
            SDL_Log("Window size: %ix%i", 800, 600);
        }
    }

    Window::~Window()
    {
        SDL_DestroyRenderer(m_renderer);
        SDL_DestroyWindow(m_window);
    }

    void Window::Render()
    {
        SDL_SetRenderDrawColor(m_renderer, 255, 105, 180, 255);
        SDL_RenderClear(m_renderer);
        SDL_RenderPresent(m_renderer);
    }
}
