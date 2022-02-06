#ifndef WINDOW_H
#define WINDOW_H

#include <SDL.h>

namespace rtte
{
    class Window
    {
    public:
        Window(int width, int height, bool debug);
        ~Window();
        void Render();

    private:
        SDL_Window *m_window;
        SDL_Renderer *m_renderer;
        int m_width;
        int m_height;
        bool m_debug;
    };
}

#endif
