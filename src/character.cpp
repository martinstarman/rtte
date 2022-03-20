#include "character.h"

#include "entity.h"
#include <polygon.h>
#include <vector>

namespace rtte
{
    Character::Character(float x, float y, const std::vector<NavMesh::Polygon> &polygons)
        : Entity(x, y, polygons),
          m_selected(false)
    {
    }

    Character::~Character()
    {
    }

    void Character::Select()
    {
        m_selected = true;
    }

    void Character::Deselect()
    {
        m_selected = false;
    }

    bool Character::Selected()
    {
        return m_selected;
    }
}