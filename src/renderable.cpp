#include "renderable.h"

Renderable::Renderable(Vector3 position)
    : m_position(position)
{
}

Vector3 Renderable::GetPosition() const
{
  return m_position;
}
