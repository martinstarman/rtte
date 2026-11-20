#pragma once

#include <cmath>
#include <raylib.h>
#include <string>

#include "animation.h"
#include "texture_handle.h"

class DirectionalSprite
{
public:
  static constexpr size_t kDirections = 8;

  DirectionalSprite(float fps);
  DirectionalSprite(const std::string &path, float fps);
  void Update(float dt);
  void Render(Vector2 feet, bool moving) const;
  void SetAngle(float degrees);
  size_t GetFacing() const;
  bool IsLoaded() const;

private:
  static constexpr Vector2 kFrameSize = {24.0f, 32.0f};
  static constexpr Vector2 kFeet = {12.0f, 29.0f};
  static constexpr size_t kWalkFrames = 4;
  static constexpr size_t kIdleFrame = 4;
  static constexpr size_t kSouth = 2;

  TextureHandle m_texture;
  Animation m_animation;
  size_t m_facing;
};
