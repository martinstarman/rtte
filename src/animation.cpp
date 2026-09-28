#include "animation.h"

Animation::Animation(size_t frameCount, Vector2 frameSize, float fps)
    : m_currentFrame(0),
      m_frameCount(frameCount),
      m_frameSize(frameSize),
      m_frameDuration(fps > 0.0f ? 1.0f / fps : 0.0f),
      m_elapsed(0.0f)
{
}

Animation::~Animation() = default;

void Animation::Update(float dt)
{
  if (m_frameCount <= 1 || m_frameDuration <= 0.0f)
  {
    return;
  }

  m_elapsed += dt;

  while (m_elapsed >= m_frameDuration)
  {
    m_elapsed -= m_frameDuration;
    m_currentFrame = (m_currentFrame + 1) % m_frameCount;
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

void Animation::SetFrameSize(Vector2 frameSize)
{
  m_frameSize = frameSize;
}
