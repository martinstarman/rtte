#ifndef CHARACTER_H
#define CHARACTER_H

#include "entity.h"

namespace rtte
{
    class Character : public Entity
    {
    public:
        Character(float x, float y);
        ~Character();
        int GetYIndex() override;
    };
}

#endif
