#include "entity.h"

Entity::Entity(
    std::tuple<int, int> position,
    std::tuple<int, int> size,
    int layerIndex,
    const std::vector<std::tuple<int, int>> &polygon)
    : m_position(position),
      m_size(size),
      m_layerIndex(layerIndex),
      m_polygon(polygon)
{
}

Entity::~Entity() = default;

int Entity::LayerIndex()
{
  return m_layerIndex;
}

int Entity::ZIndex()
{
  return std::get<1>(m_position) + std::get<1>(m_size);
}

void Entity::Draw()
{
  for (int i = 0; i < m_polygon.size(); i++)
  {
    DrawLine(
        std::get<0>(m_polygon.at(i)),
        std::get<1>(m_polygon.at(i)),
        std::get<0>(m_polygon.at((i + 1) % m_polygon.size())),
        std::get<1>(m_polygon.at((i + 1) % m_polygon.size())),
        WHITE);
  }
}
