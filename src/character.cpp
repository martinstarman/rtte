#include "character.h"

Character::Character(Vector3 position)
    : Renderable(position),
      m_path({}),
      m_pathIndex(0),
      m_selected(false),
      m_stepDistance(0.0f),
      m_leftFoot(true),
      m_footprints({}),
      m_sprite(kAnimationFps)
{
}

Character::Character(Vector3 position, const std::string &spritePath)
    : Character(position)
{
  m_sprite = DirectionalSprite(spritePath, kAnimationFps);
}

void Character::Update(float dt)
{
  if (IsMoving())
  {
    m_sprite.Update(dt);
  }

  float distance = kSpeed * dt;

  while (IsMoving() && distance > 0.0f)
  {
    Vector2 target = m_path.at(m_pathIndex);
    Vector2 position = GetPosition2D();
    float remaining = Vector2Distance(position, target);

    if (remaining <= distance)
    {
      Walk(position, target);
      m_position.x = target.x;
      m_position.y = target.y;
      distance -= remaining;
      ++m_pathIndex;
    }
    else
    {
      Vector2 moved = Vector2MoveTowards(position, target, distance);
      Walk(position, moved);
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

  if (!m_sprite.IsLoaded())
  {
    DrawCircleV(position, kRadius, GREEN);
  }
  else
  {
    DrawEllipse((int)position.x, (int)position.y, kRadius, kRadius / 2.0f, Fade(BLACK, 0.3f));
  }

  if (m_selected)
  {
    DrawCircleLinesV(position, kRadius + 3.0f, YELLOW);
  }

  if (m_sprite.IsLoaded())
  {
    m_sprite.Render(position, IsMoving());
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

size_t Character::GetFacing() const
{
  return m_sprite.GetFacing();
}

std::vector<Footprint> Character::GetFootprints()
{
  std::vector<Footprint> footprints;
  std::swap(footprints, m_footprints);

  return footprints;
}

void Character::Walk(Vector2 from, Vector2 to)
{
  float distance = Vector2Distance(from, to);

  if (distance <= 0.0f)
  {
    return;
  }

  Vector2 direction = Vector2Normalize(Vector2Subtract(to, from));
  float angle = std::atan2(direction.y, direction.x) * RAD2DEG;
  m_sprite.SetAngle(angle);
  Vector2 normal = {-direction.y, direction.x};
  float rotation = angle + 90.0f;

  while (m_stepDistance + distance >= kStepLength)
  {
    float advance = kStepLength - m_stepDistance;
    from = Vector2Add(from, Vector2Scale(direction, advance));
    distance -= advance;
    m_stepDistance = 0.0f;

    float side = m_leftFoot ? -kFootOffset : kFootOffset;
    Vector2 foot = Vector2Add(from, Vector2Scale(normal, side));
    m_footprints.emplace_back(Vector3{foot.x, foot.y, m_position.z}, rotation);
    m_leftFoot = !m_leftFoot;
  }

  m_stepDistance += distance;
}
