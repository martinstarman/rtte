#pragma once

#include <limits>
#include <raylib.h>
#include <string>
#include <vector>

#include "animation.h"

class Sprite
{
public:
  Sprite(const std::string &path, Vector3 position);
  Sprite(const std::string &path, Vector3 position, Animation animation);
  Sprite(const std::string &path, Vector3 position, Vector2 size);
  Sprite(const std::string &path, Vector3 position, Vector2 size, Animation animation);
  Sprite(const std::string &path, Vector3 position, const std::vector<Vector2> shape);
  Sprite(const std::string &path, Vector3 position, const std::vector<Vector2> shape, Animation animation);
  ~Sprite();
  void Update();
  void Render();
  const Texture2D &GetTexture() const;

private:
  std::string m_path;
  Vector3 m_position;
  Vector2 m_size;
  std::vector<Vector2> m_shape;
  Texture2D m_texture;
  Animation m_animation;
};
