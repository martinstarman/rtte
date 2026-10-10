#pragma once

#include <algorithm>
#include <limits>
#include <raylib.h>

#include "renderable.h"

class Footprint : public Renderable
{
public:
  static constexpr float kLifetime = 10.0f;

  Footprint(Vector3 position, float rotation);
  void Update(float dt);
  void Render() const override;
  float GetYSortIndex() const override;
  float GetOpacity() const;
  bool IsExpired() const;

private:
  static constexpr Vector2 kSize = {3.0f, 6.0f};

  float m_rotation;
  float m_age;
};
