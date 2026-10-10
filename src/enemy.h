#pragma once

#include <algorithm>
#include <cmath>
#include <raylib.h>
#include <raymath.h>
#include <vector>

#include "obstacle.h"
#include "renderable.h"

class Enemy : public Renderable
{
public:
  static constexpr float kSpeed = 30.0f;
  static constexpr float kViewDistance = 200.0f;
  static constexpr float kViewAngle = 60.0f;
  static constexpr float kSweepAngle = 30.0f;
  static constexpr float kSweepSpeed = 2.0f;

  Enemy(Vector3 position, const std::vector<Vector2> &path, float direction);
  void Update(float dt);
  void Render() const override;
  float GetYSortIndex() const override;
  bool Contains(Vector2 point) const;
  float GetDirection() const;
  float GetLookAngle() const;
  std::vector<Vector2> GetViewPolygon(const std::vector<Obstacle> &obstacles) const;

private:
  static constexpr float kRadius = 8.0f;
  static constexpr int kViewRays = 64;

  Vector2 GetPosition2D() const;
  float CastRay(Vector2 origin, Vector2 end, const std::vector<Obstacle> &obstacles) const;

  std::vector<Vector2> m_path;
  size_t m_pathIndex;
  float m_direction;
  float m_time;
};
