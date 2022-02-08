#ifndef GAME_H
#define GAME_H

#include "entity.h"
#include <SDL.h>

namespace rtte
{
    class Game
    {
    public:
        static Game *Get();
        ~Game();
        void Render();
        void SetDebug();
        bool GetDebug();
        SDL_Renderer *GetRenderer();
        void HandleClick(int x, int y);

    private:
        Game();
        static Game *s_instance;
        SDL_Window *m_window;
        SDL_Renderer *m_renderer;
        Entity *m_entity;
        bool m_debug;
    };
}

#endif
