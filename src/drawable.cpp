#include "drawable.h"

Drawable::Drawable(Vector3 position, Vector2 size)
    : m_position(position),
      m_size(size)
{
}

float Drawable::GetYSortIndex() const
{
  return m_position.y + m_size.y;
}

Vector3 Drawable::GetPosition() const
{
  return m_position;
}

Vector2 Drawable::GetSize() const
{
  return m_size;
}
