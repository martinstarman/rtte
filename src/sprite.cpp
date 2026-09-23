#include "sprite.h"

Sprite::Sprite(const std::string &path, Vector2 dimensions, Vector2 position)
    : m_path(path),
      m_position(position),
      m_dimensions(dimensions),
      m_texture(LoadTexture(path.c_str()))
{
}

Sprite::~Sprite()
{
  UnloadTexture(m_texture);
}

void Sprite::Render()
{
  DrawTextureV(m_texture, m_position, WHITE);
}

const Texture2D &Sprite::GetTexture() const
{
  return m_texture;
}
