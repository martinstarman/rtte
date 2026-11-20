#include "directional_sprite.h"

DirectionalSprite::DirectionalSprite(float fps)
    : m_texture(),
      m_animation(kWalkFrames, kFrameSize, fps),
      m_facing(kSouth)
{
}

DirectionalSprite::DirectionalSprite(const std::string &path, float fps)
    : m_texture(LoadTexture(path.c_str())),
      m_animation(kWalkFrames, kFrameSize, fps),
      m_facing(kSouth)
{
}

void DirectionalSprite::Update(float dt)
{
  m_animation.Update(dt);
}

void DirectionalSprite::Render(Vector2 feet, bool moving) const
{
  size_t frame = moving ? m_animation.GetCurrentFrame() : kIdleFrame;
  Rectangle source = {frame * kFrameSize.x, m_facing * kFrameSize.y, kFrameSize.x, kFrameSize.y};
  Vector2 topLeft = {std::floor(feet.x - kFeet.x), std::floor(feet.y - kFeet.y)};

  DrawTextureRec(m_texture.Get(), source, topLeft, WHITE);
}

void DirectionalSprite::SetAngle(float degrees)
{
  int sector = (int)std::lround(degrees / (360.0f / kDirections));
  m_facing = (size_t)(((sector % (int)kDirections) + (int)kDirections) % (int)kDirections);
}

size_t DirectionalSprite::GetFacing() const
{
  return m_facing;
}

bool DirectionalSprite::IsLoaded() const
{
  return m_texture.Get().id != 0;
}
