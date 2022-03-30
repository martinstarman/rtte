#include "texture.h"

#include "game.h"
#include <SDL.h>
#include <SDL_image.h>
#include <string>

namespace rtte
{
    Texture::Texture(const std::string &image)
        : m_texture(nullptr),
          m_width(0),
          m_height(0)
    {
        SDL_Surface *surface = IMG_Load(image.c_str());
        m_texture = SDL_CreateTextureFromSurface(Game::Get()->GetRenderer(), surface);
        SDL_QueryTexture(m_texture, nullptr, nullptr, &m_width, &m_height);
        SDL_FreeSurface(surface);
    }

    Texture::~Texture()
    {
        SDL_DestroyTexture(m_texture);
    }

    int Texture::GetWidth()
    {
        return m_width;
    }

    int Texture::GetHeight()
    {
        return m_height;
    }

    SDL_Texture *Texture::GetSDLTexture()
    {
        return m_texture;
    }
}