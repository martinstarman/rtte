#include "animation.h"

Animation::Animation(size_t frameCount, Vector2 frameSize)
    : m_currentFrame(0),
      m_frameCount(frameCount),
      m_frameSize(frameSize)
{
}

Animation::~Animation() = default;

void Animation::Update()
{
  if (++m_currentFrame >= m_frameCount)
  {
    m_currentFrame = 0;
  }
}

size_t Animation::GetFrameCount()
{
  return m_frameCount;
}

size_t Animation::GetCurrentFrame()
{
  return m_currentFrame;
}

Vector2 Animation::GetFrameSize()
{
  return m_frameSize;
}
