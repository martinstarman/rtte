#pragma once

#include <algorithm>
#include <raylib.h>
#include <string>

#include "animation.h"
#include "drawable.h"

class Sprite : public Drawable
{
public:
  Sprite(const std::string &path, Vector3 position, Vector2 size);
  Sprite(const std::string &path, Vector3 position, Vector2 size, Animation animation);
  ~Sprite();
  void Update(float dt);
  void Render() const override;

private:
  std::string m_path;
  Texture m_texture;
  Animation m_animation;
};
