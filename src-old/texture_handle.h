#pragma once

#include <raylib.h>

class TextureHandle
{
public:
  TextureHandle();
  explicit TextureHandle(Texture2D texture);
  TextureHandle(const TextureHandle &) = delete;
  TextureHandle &operator=(const TextureHandle &) = delete;
  TextureHandle(TextureHandle &&other) noexcept;
  TextureHandle &operator=(TextureHandle &&other) noexcept;
  ~TextureHandle();
  const Texture2D &Get() const;

private:
  Texture2D m_texture;
};
