#ifndef ENTITY_H
#define ENTITY_H

#include <path_finder.h>
#include <point.h>
#include <vector>

namespace rtte
{
    class Entity
    {
    public:
        Entity();
        ~Entity();
        void Render();
        void FindPath(int x, int y);

    private:
        float m_x;
        float m_y;
        NavMesh::PathFinder m_pathFinder;
        std::vector<NavMesh::Point> m_path;
    };
}

#endif
