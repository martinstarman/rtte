#include "character.h"

Character::Character(Vector3 position)
    : Renderable(position),
      m_path({}),
      m_pathIndex(0),
      m_selected(false)
{
}

void Character::Update(float dt)
{
  float distance = kSpeed * dt;

  while (IsMoving() && distance > 0.0f)
  {
    Vector2 target = m_path.at(m_pathIndex);
    Vector2 position = GetPosition2D();
    float remaining = Vector2Distance(position, target);

    if (remaining <= distance)
    {
      m_position.x = target.x;
      m_position.y = target.y;
      distance -= remaining;
      ++m_pathIndex;
    }
    else
    {
      Vector2 moved = Vector2MoveTowards(position, target, distance);
      m_position.x = moved.x;
      m_position.y = moved.y;
      distance = 0.0f;
    }
  }
}

void Character::Render() const
{
  Vector2 position = GetPosition2D();

  if (IsMoving())
  {
    DrawLineEx(position, m_path.at(m_pathIndex), 2.0f, RED);

    for (size_t i = m_pathIndex + 1; i < m_path.size(); ++i)
    {
      DrawLineEx(m_path.at(i - 1), m_path.at(i), 2.0f, RED);
    }
  }

  DrawCircleV(position, kRadius, GREEN);

  if (m_selected)
  {
    DrawCircleLinesV(position, kRadius + 3.0f, YELLOW);
  }
}

void Character::SetPath(const std::vector<Vector2> &path)
{
  m_path = path;
  m_pathIndex = 0;
}

float Character::GetYSortIndex() const
{
  return m_position.y + kRadius;
}

bool Character::IsMoving() const
{
  return m_pathIndex < m_path.size();
}

bool Character::Contains(Vector2 point) const
{
  return CheckCollisionPointCircle(point, GetPosition2D(), kRadius);
}

bool Character::IsSelected() const
{
  return m_selected;
}

void Character::SetSelected(bool selected)
{
  m_selected = selected;
}

Vector2 Character::GetPosition2D() const
{
  return {m_position.x, m_position.y};
}
