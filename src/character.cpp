#include "character.h"

#include "entity.h"
#include <polygon.h>
#include <vector>

namespace rtte
{
    Character::Character(float x, float y, const std::vector<NavMesh::Polygon> &polygons)
        : Entity(x, y, polygons)
    {
    }

    Character::~Character()
    {
    }
}