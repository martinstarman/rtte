#ifndef GAME_H
#define GAME_H

#include "entity.h"
#include <point.h>
#include <pointf.h>
#include <polygon.h>
#include <SDL.h>
#include <string>
#include <vector>

namespace rtte
{
    // data to (de)serialization
    struct GameData
    {
        bool debug;
        std::string missionName;
        int mapWidth;
        int mapHeight;
        std::vector<NavMesh::Polygon> polygons;
    };

    class Game
    {
    public:
        static Game *Get();
        void SetGameData(GameData gameData);
        ~Game();
        void Update();
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
        Entity *m_entity;                         // game data
        SDL_Point m_mouse;
        NavMesh::Point m_windowSize;
        NavMesh::PointF m_offset;
        SDL_Rect m_topRect;
        SDL_Rect m_rightRect;
        SDL_Rect m_bottomRect;
        SDL_Rect m_leftRect;
        GameData m_gameData;
    };
}

#endif
