#include "character.h"

#include "entity.h"

namespace rtte
{
    Character::Character(float x, float y)
        : Entity(x, y)
    {
    }

    Character::~Character()
    {
    }

    int Character::GetYIndex()
    {
        return (int)m_y; // TODO: height
    }
}