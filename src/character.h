#pragma once

#include <raylib.h>
#include <cmath>
#include <raymath.h>
#include <string>
#include <utility>
#include <vector>

#include "directional_sprite.h"
#include "footprint.h"
#include "renderable.h"

class Character : public Renderable
{
public:
  static constexpr float kSpeed = 150.0f;
  static constexpr float kStepLength = 12.0f;

  Character(Vector3 position);
  Character(Vector3 position, const std::string &spritePath);
  void Update(float dt);
  void Render() const override;
  void SetPath(const std::vector<Vector2> &path);
  float GetYSortIndex() const override;
  bool IsMoving() const;
  bool Contains(Vector2 point) const;
  bool IsSelected() const;
  void SetSelected(bool selected);
  std::vector<Footprint> GetFootprints();
  size_t GetFacing() const;

private:
  Vector2 GetPosition2D() const;
  void Walk(Vector2 from, Vector2 to);

  static constexpr float kRadius = 8.0f;
  static constexpr float kFootOffset = 3.0f;
  static constexpr float kAnimationFps = 2.0f * kSpeed / kStepLength;

  std::vector<Vector2> m_path;
  size_t m_pathIndex;
  bool m_selected;
  float m_stepDistance;
  bool m_leftFoot;
  std::vector<Footprint> m_footprints;
  DirectionalSprite m_sprite;
};
