#include "texture_handle.h"

TextureHandle::TextureHandle()
    : m_texture({})
{
}

TextureHandle::TextureHandle(Texture2D texture)
    : m_texture(texture)
{
}

TextureHandle::TextureHandle(TextureHandle &&other) noexcept
    : m_texture(other.m_texture)
{
  other.m_texture = {};
}

TextureHandle &TextureHandle::operator=(TextureHandle &&other) noexcept
{
  if (this != &other)
  {
    if (m_texture.id > 0)
    {
      UnloadTexture(m_texture);
    }

    m_texture = other.m_texture;
    other.m_texture = {};
  }

  return *this;
}

TextureHandle::~TextureHandle()
{
  if (m_texture.id > 0)
  {
    UnloadTexture(m_texture);
  }
}

const Texture2D &TextureHandle::Get() const
{
  return m_texture;
}
