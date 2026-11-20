#include "animation.h"

Animation::Animation(int frameCount, Vector2 frameSize, int fps)
    : m_currentFrame(0),
      m_frameCount(frameCount),
      m_frameSize(frameSize),
      m_frameDuration(fps > 0 ? 1.0f / (float)fps : 0.0f),
      m_timeElapsed(0.0f)
{
}

Animation::~Animation() = default;

void Animation::Update(float dt)
{
  if (m_frameCount <= 1 || m_frameDuration <= 0.0f)
  {
    return;
  }

  m_timeElapsed += dt;

  while (m_timeElapsed >= m_frameDuration)
  {
    m_timeElapsed -= m_frameDuration;
    m_currentFrame = (m_currentFrame + 1) % m_frameCount;
  }
}

int Animation::GetFrameCount() const
{
  return m_frameCount;
}

int Animation::GetCurrentFrame() const
{
  return m_currentFrame;
}

Vector2 Animation::GetFrameSize() const
{
  return m_frameSize;
}
