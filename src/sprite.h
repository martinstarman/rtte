#pragma once

#include <limits>
#include <raylib.h>
#include <string>
#include <vector>

class Sprite
{
public:
  Sprite(const std::string &path, Vector3 position);
  Sprite(const std::string &path, Vector3 position, Vector2 dimensions);
  Sprite(const std::string &path, Vector3 position, const std::vector<Vector2> shape);
  ~Sprite();
  void Render();
  const Texture2D &GetTexture() const;

private:
  std::string m_path;
  Vector3 m_position;
  Vector2 m_dimensions;
  std::vector<Vector2> m_shape;
  Texture2D m_texture;
};
