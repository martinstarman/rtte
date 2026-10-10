#pragma once

#include <raylib.h>
#include <raymath.h>
#include <vector>

#include "renderable.h"

class Character : public Renderable
{
public:
  static constexpr float kSpeed = 150.0f;

  Character(Vector3 position);
  void Update(float dt);
  void Render() const override;
  void SetPath(const std::vector<Vector2> &path);
  float GetYSortIndex() const override;
  bool IsMoving() const;
  bool Contains(Vector2 point) const;
  bool IsSelected() const;
  void SetSelected(bool selected);

private:
  Vector2 GetPosition2D() const;

  static constexpr float kRadius = 8.0f;

  std::vector<Vector2> m_path;
  size_t m_pathIndex;
  bool m_selected;
};
