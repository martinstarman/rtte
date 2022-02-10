#ifndef GAME_H
#define GAME_H

#include "entity.h"
#include <point.h>
#include <pointf.h>
#include <polygon.h>
#include <SDL.h>
#include <vector>

namespace rtte
{
    class Game
    {
    public:
        static Game *Get();
        ~Game();
        void Update();
        void SetDebug();
        bool GetDebug();
        SDL_Renderer *GetRenderer();
        NavMesh::Point ToRenderPos(NavMesh::Point pos);

    private:
        Game();
        void Render();
        NavMesh::Point ToGamePos(SDL_Point pos);
        static Game *s_instance;
        SDL_Window *m_window;
        SDL_Renderer *m_renderer;
        std::vector<NavMesh::Polygon> m_polygons;
        Entity *m_entity;
        bool m_debug;
        SDL_Point m_mouse;
        NavMesh::Point m_windowSize;
        NavMesh::Point m_mapSize;
        NavMesh::PointF m_offset;
        SDL_Rect m_topRect;
        SDL_Rect m_rightRect;
        SDL_Rect m_bottomRect;
        SDL_Rect m_leftRect;
    };
}

#endif
