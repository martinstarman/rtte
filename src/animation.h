#pragma once

#include <raylib.h>

class Animation
{
public:
  Animation(size_t frameCount, Vector2 frameSize);
  ~Animation();
  void Update();
  size_t GetFrameCount();
  size_t GetCurrentFrame();
  Vector2 GetFrameSize();

private:
  size_t m_currentFrame;
  size_t m_frameCount;
  Vector2 m_frameSize;
};
