#ifndef TEXTURE_H
#define TEXTURE_H

#include <SDL.h>
#include <string>

namespace rtte
{
    class Texture
    {
    public:
        Texture(const std::string &image);
        ~Texture();
        int GetWidth();
        int GetHeight();
        SDL_Texture *GetSDLTexture();

    private:
        SDL_Texture *m_texture;
        int m_width;
        int m_height;
    };
}

#endif
