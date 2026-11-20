#pragma once

#include <raylib.h>

class Drawable
{
public:
  Drawable(Vector3 position, Vector2 size);
  virtual ~Drawable() = default;
  virtual void Render() const = 0;
  float GetYSortIndex() const;
  Vector3 GetPosition() const;
  Vector2 GetSize() const;

protected:
  Vector3 m_position;
  Vector2 m_size;
};
