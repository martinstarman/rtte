#ifndef ENTITY_H
#define ENTITY_H

#include <path_finder.h>
#include <polygon.h>
#include <SDL.h>
#include <vector>

namespace rtte
{
    class Entity
    {
    public:
        Entity(float x, float y, const std::vector<NavMesh::Polygon> &polygons);
        ~Entity();
        void FindPath(int x, int y);
        void RemovePath();
        void Update(float dt);
        void Render();
        SDL_Rect GetRect();
        void Select();
        void Deselect();
        bool Selected();
        float GetX();
        float GetY();

    private:
        void Move(float dt);

    private:
        float m_x;
        float m_y;
        bool m_selected;
        NavMesh::PathFinder m_pathFinder;
        std::vector<SDL_Point> m_path;
    };
}

#endif
