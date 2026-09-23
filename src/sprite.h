#pragma once

#include <raylib.h>
#include <string>

class Sprite
{
public:
  Sprite(const std::string &path, Vector2 dimensions, Vector2 position);
  ~Sprite();
  void Render();
  const Texture2D &GetTexture() const;

private:
  std::string m_path;
  Vector2 m_position;
  Vector2 m_dimensions;
  Texture2D m_texture;
};
