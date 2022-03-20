#ifndef CHARACTER_H
#define CHARACTER_H

#include "entity.h"

namespace rtte
{
    class Character
        : public Entity
    {
    public:
        Character(float x, float y, const std::vector<NavMesh::Polygon> &polygons);
        ~Character();
        void Select();
        void Deselect();
        bool Selected();

    private:
        bool m_selected;
    };
}

#endif
