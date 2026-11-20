#pragma once

#include <raylib.h>

class Animation
{
public:
  Animation(int frameCount, Vector2 frameSize, int fps);
  ~Animation();
  void Update(float dt);
  int GetFrameCount() const;
  int GetCurrentFrame() const;
  Vector2 GetFrameSize() const;

private:
  int m_currentFrame;
  int m_frameCount;
  Vector2 m_frameSize;
  float m_frameDuration;
  float m_timeElapsed;
};
