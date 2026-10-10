#pragma once

#include <raylib.h>

class Renderable
{
public:
  Renderable(Vector3 position);
  virtual ~Renderable() = default;
  virtual void Render() const = 0;
  virtual float GetYSortIndex() const = 0;
  Vector3 GetPosition() const;

protected:
  Vector3 m_position;
};
