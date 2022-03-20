#ifndef GAME_H
#define GAME_H

#include "entity.h"
#include <point.h>
#include <pointf.h>
#include <polygon.h>
#include <SDL.h>
#include <string>
#include <SDL.h>
#include <SDL_ttf.h>
#include <vector>

namespace rtte
{
    // data to (de)serialization
    struct GameData
    {
        bool debug;
        std::string missionName; // TODO: not used
        int mapWidth;
        int mapHeight;
        std::vector<NavMesh::Polygon> polygons;
        std::vector<Entity *> entities;
    };

    class Game
    {
    public:
        static Game *Get();
        void SetGameData(GameData gameData);
        ~Game();
        void Update(float dt);
        void Render(float dt);
        bool GetDebug();
        SDL_Renderer *GetRenderer();
        NavMesh::Point ToRenderPos(NavMesh::Point pos);
        bool GetRunning();

    private:
        Game();
        NavMesh::Point ToGamePos(SDL_Point pos);

    private:
        static Game *s_instance;
        SDL_Window *m_window;
        SDL_Renderer *m_renderer;
        SDL_Point m_mouse;
        NavMesh::Point m_windowSize;
        NavMesh::PointF m_offset;
        TTF_Font *m_font;
        bool m_running;
        SDL_Rect m_topRect;
        SDL_Rect m_rightRect;
        SDL_Rect m_bottomRect;
        SDL_Rect m_leftRect;
        GameData m_gameData;
    };
}

#endif
