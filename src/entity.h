#ifndef ENTITY_H
#define ENTITY_H

#include <path_finder.h>
#include <SDL.h>
#include <vector>

namespace rtte
{
    class Entity
    {
    public:
        Entity(float x, float y);
        ~Entity();
        void FindPath(int x, int y);
        void RemovePath();
        virtual void Update(float dt);
        virtual void Render();
        SDL_Rect GetRect();
        void Select();
        void Deselect();
        bool Selected();
        float GetX();
        float GetY();
        virtual int GetYIndex() = 0;

    private:
        void Move(float dt);

    private:
        bool m_selected;
        NavMesh::PathFinder m_pathFinder;
        std::vector<SDL_Point> m_path;

    protected:
        float m_x;
        float m_y;
    };
}

#endif
