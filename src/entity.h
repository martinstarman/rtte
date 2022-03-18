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
        Entity(const std::vector<NavMesh::Polygon> &polygons);
        ~Entity();
        void FindPath(int x, int y);
        void RemovePath();
        void Update(float dt);
        void Render();

    private:
        float m_x;
        float m_y;
        NavMesh::PathFinder m_pathFinder;
        std::vector<NavMesh::Point> m_path;
    };
}

#endif
