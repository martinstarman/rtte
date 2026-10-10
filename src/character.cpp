#include "character.h"

Character::Character(Vector2 position, float speed)
    : m_position(position),
      m_speed(speed),
      m_path({}),
      m_pathIndex(0),
      m_selected(false)
{
}

void Character::Update(float dt)
{
  float distance = m_speed * dt;

  while (IsMoving() && distance > 0.0f)
  {
    Vector2 target = m_path.at(m_pathIndex);
    float remaining = Vector2Distance(m_position, target);

    if (remaining <= distance)
    {
      m_position = target;
      distance -= remaining;
      ++m_pathIndex;
    }
    else
    {
      m_position = Vector2MoveTowards(m_position, target, distance);
      distance = 0.0f;
    }
  }
}

void Character::Render() const
{
  if (IsMoving())
  {
    DrawLineEx(m_position, m_path.at(m_pathIndex), 2.0f, RED);

    for (size_t i = m_pathIndex + 1; i < m_path.size(); ++i)
    {
      DrawLineEx(m_path.at(i - 1), m_path.at(i), 2.0f, RED);
    }
  }

  DrawCircleV(m_position, kRadius, GREEN);

  if (m_selected)
  {
    DrawCircleLinesV(m_position, kRadius + 3.0f, YELLOW);
  }
}

void Character::SetPath(const std::vector<Vector2> &path)
{
  m_path = path;
  m_pathIndex = 0;
}

Vector2 Character::GetPosition() const
{
  return m_position;
}

bool Character::IsMoving() const
{
  return m_pathIndex < m_path.size();
}

bool Character::Contains(Vector2 point) const
{
  return CheckCollisionPointCircle(point, m_position, kRadius);
}

bool Character::IsSelected() const
{
  return m_selected;
}

void Character::SetSelected(bool selected)
{
  m_selected = selected;
}
