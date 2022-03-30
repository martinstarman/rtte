#include "object.h"

#include "entity.h"
#include "game.h"
#include <SDL.h>
#include <string>
#include "texture.h"

namespace rtte
{
    // TODO: this should not be selectable, movable
    //       we can override these methods
    Object::Object(float x, float y, const std::string &image)
        : Entity(x, y)
    {
        m_texture = new Texture(image);
    }

    Object::~Object()
    {
        delete m_texture;
    }

    void Object::Render()
    {
        SDL_Point pos = Game::Get()->ToRenderPos((int)m_x, (int)m_y);
        SDL_Rect rect{pos.x, pos.y, m_texture->GetWidth(), m_texture->GetHeight()};
        SDL_RenderCopy(Game::Get()->GetRenderer(), m_texture->GetSDLTexture(), nullptr, &rect);

        if (Game::Get()->GetDebug())
        {
            SDL_RenderDrawRect(Game::Get()->GetRenderer(), &rect);
        }

        Entity::Render();
    }

    int Object::GetYIndex()
    {
        return (int)m_y + m_texture->GetHeight();
    }
}