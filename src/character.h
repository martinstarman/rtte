#pragma once

#include <raylib.h>
#include <cmath>
#include <raymath.h>
#include <utility>
#include <vector>

#include "footprint.h"
#include "renderable.h"

class Character : public Renderable
{
public:
  static constexpr float kSpeed = 150.0f;
  static constexpr float kStepLength = 12.0f;

  Character(Vector3 position);
  void Update(float dt);
  void Render() const override;
  void SetPath(const std::vector<Vector2> &path);
  float GetYSortIndex() const override;
  bool IsMoving() const;
  bool Contains(Vector2 point) const;
  bool IsSelected() const;
  void SetSelected(bool selected);
  std::vector<Footprint> GetFootprints();

private:
  Vector2 GetPosition2D() const;
  void Walk(Vector2 from, Vector2 to);

  static constexpr float kRadius = 8.0f;
  static constexpr float kFootOffset = 3.0f;

  std::vector<Vector2> m_path;
  size_t m_pathIndex;
  bool m_selected;
  float m_stepDistance;
  bool m_leftFoot;
  std::vector<Footprint> m_footprints;
};
