#pragma once

#include <raylib.h>

class Animation
{
public:
  Animation(size_t frameCount, Vector2 frameSize, float fps);
  ~Animation();
  void Update(float dt);
  size_t GetFrameCount() const;
  size_t GetCurrentFrame() const;
  Vector2 GetFrameSize() const;
  void SetFrameSize(Vector2 frameSize);

private:
  size_t m_currentFrame;
  size_t m_frameCount;
  Vector2 m_frameSize;
  float m_frameDuration;
  float m_elapsed;
};
