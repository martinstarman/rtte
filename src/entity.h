#pragma once

#include <raylib.h>
#include <tuple>
#include <vector>

class Entity
{
public:
  Entity(
      std::tuple<int, int> position,
      std::tuple<int, int> size,
      int layerIndex,
      const std::vector<std::tuple<int, int>> &polygon);
  ~Entity();
  int LayerIndex();
  int ZIndex();
  void Draw();

private:
  std::tuple<int, int> m_position;
  std::tuple<int, int> m_size;
  int m_layerIndex;
  std::vector<std::tuple<int, int>> m_polygon;
};
