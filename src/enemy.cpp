#include "enemy.h"

Enemy::Enemy(Vector3 position, const std::vector<Vector2> &path, float direction)
    : Renderable(position),
      m_path(path),
      m_pathIndex(0),
      m_direction(direction),
      m_time(0.0f),
      m_moving(false),
      m_sprite(kAnimationFps)
{
  m_sprite.SetAngle(m_direction);
}

Enemy::Enemy(Vector3 position, const std::vector<Vector2> &path, float direction, const std::string &spritePath)
    : Enemy(position, path, direction)
{
  m_sprite = DirectionalSprite(spritePath, kAnimationFps);
  m_sprite.SetAngle(m_direction);
}

void Enemy::Update(float dt)
{
  m_time += dt;

  float distance = kSpeed * dt;
  size_t reached = 0;
  m_moving = false;

  while (!m_path.empty() && distance > 0.0f && reached <= m_path.size())
  {
    Vector2 target = m_path.at(m_pathIndex);
    Vector2 position = GetPosition2D();
    float remaining = Vector2Distance(position, target);

    if (remaining > 0.0f)
    {
      m_direction = std::atan2(target.y - position.y, target.x - position.x) * RAD2DEG;
      m_sprite.SetAngle(m_direction);
      m_moving = true;
      reached = 0;
    }

    if (remaining <= distance)
    {
      m_position.x = target.x;
      m_position.y = target.y;
      distance -= remaining;
      m_pathIndex = (m_pathIndex + 1) % m_path.size();
      ++reached;
    }
    else
    {
      Vector2 moved = Vector2MoveTowards(position, target, distance);
      m_position.x = moved.x;
      m_position.y = moved.y;
      distance = 0.0f;
    }
  }

  if (m_moving)
  {
    m_sprite.Update(dt);
  }
}

void Enemy::Render() const
{
  Vector2 position = GetPosition2D();

  if (!m_sprite.IsLoaded())
  {
    float look = GetLookAngle() * DEG2RAD;
    Vector2 end = {position.x + std::cos(look) * kRadius * 2.0f, position.y + std::sin(look) * kRadius * 2.0f};

    DrawCircleV(position, kRadius, RED);
    DrawLineEx(position, end, 2.0f, RED);
    return;
  }

  DrawEllipse((int)position.x, (int)position.y, kRadius, kRadius / 2.0f, Fade(BLACK, 0.3f));
  m_sprite.Render(position, m_moving);
}

float Enemy::GetYSortIndex() const
{
  return m_position.y + kRadius;
}

bool Enemy::Contains(Vector2 point) const
{
  return CheckCollisionPointCircle(point, GetPosition2D(), kRadius);
}

float Enemy::GetDirection() const
{
  return m_direction;
}

size_t Enemy::GetFacing() const
{
  return m_sprite.GetFacing();
}

bool Enemy::IsMoving() const
{
  return m_moving;
}

float Enemy::GetLookAngle() const
{
  return m_direction + kSweepAngle * std::sin(m_time * kSweepSpeed);
}

std::vector<Vector2> Enemy::GetViewPolygon(const std::vector<Obstacle> &obstacles) const
{
  Vector2 origin = GetPosition2D();
  float start = GetLookAngle() - kViewAngle / 2.0f;
  std::vector<Vector2> polygon = {origin};

  for (int i = 0; i <= kViewRays; ++i)
  {
    float angle = (start + kViewAngle * i / kViewRays) * DEG2RAD;
    Vector2 end = {origin.x + std::cos(angle) * kViewDistance, origin.y + std::sin(angle) * kViewDistance};
    float length = CastRay(origin, end, obstacles);

    polygon.push_back(Vector2Lerp(origin, end, length / kViewDistance));
  }

  return polygon;
}

Vector2 Enemy::GetPosition2D() const
{
  return {m_position.x, m_position.y};
}

float Enemy::CastRay(Vector2 origin, Vector2 end, const std::vector<Obstacle> &obstacles) const
{
  float nearest = kViewDistance;

  for (const Obstacle &obstacle : obstacles)
  {
    if (!obstacle.BlocksVision())
    {
      continue;
    }

    const std::vector<Vector2> &points = obstacle.GetPoints();

    for (size_t i = 0; i < points.size(); ++i)
    {
      Vector2 hit;

      if (CheckCollisionLines(origin, end, points.at(i), points.at((i + 1) % points.size()), &hit))
      {
        nearest = std::min(nearest, Vector2Distance(origin, hit));
      }
    }
  }

  return nearest;
}
