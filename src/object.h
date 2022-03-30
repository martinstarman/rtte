#ifndef OBJECT_H
#define OBJECT_H

#include "entity.h"
#include <string>
#include "texture.h"

namespace rtte
{
    class Object : public Entity
    {
    public:
        Object(float x, float y, const std::string &image);
        ~Object();
        void Render() override;
        int GetYIndex() override;

    private:
        Texture *m_texture;
    };
}

#endif
