#include "footprint.h"

Footprint::Footprint(Vector3 position, float rotation)
    : Renderable(position),
      m_rotation(rotation),
      m_age(0.0f)
{
}

void Footprint::Update(float dt)
{
  m_age += dt;
}

void Footprint::Render() const
{
  Rectangle rect = {m_position.x, m_position.y, kSize.x, kSize.y};
  Vector2 origin = {kSize.x / 2.0f, kSize.y / 2.0f};

  DrawRectanglePro(rect, origin, m_rotation, Fade(LIGHTGRAY, GetOpacity()));
}

float Footprint::GetYSortIndex() const
{
  return -std::numeric_limits<float>::infinity();
}

float Footprint::GetOpacity() const
{
  return std::clamp(1.0f - m_age / kLifetime, 0.0f, 1.0f);
}

bool Footprint::IsExpired() const
{
  return m_age >= kLifetime;
}
