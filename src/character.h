#pragma once

#include <raylib.h>
#include <raymath.h>
#include <vector>

class Character
{
public:
  Character(Vector2 position, float speed);
  void Update(float dt);
  void Render() const;
  void SetPath(const std::vector<Vector2> &path);
  Vector2 GetPosition() const;
  bool IsMoving() const;
  bool Contains(Vector2 point) const;
  bool IsSelected() const;
  void SetSelected(bool selected);

private:
  static constexpr float kRadius = 8.0f;

  Vector2 m_position;
  float m_speed;
  std::vector<Vector2> m_path;
  size_t m_pathIndex;
  bool m_selected;
};
