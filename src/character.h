#ifndef CHARACTER_H
#define CHARACTER_H

#include "entity.h"
#include <polygon.h>
#include <vector>

namespace rtte
{
    class Character : public Entity
    {
    public:
        Character(float x, float y, const std::vector<NavMesh::Polygon> &polygons);
        ~Character();
    };
}

#endif
