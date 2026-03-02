#pragma once

#include <limits>
#include <raylib.h>
#include <string>
#include <tuple>
#include <vector>

enum TextureTransformation
{
  None,
  Fill,
};

class Entity
{
public:
  Entity(
      std::tuple<int, int> position,
      std::tuple<int, int> size,
      int layerIndex,
      const std::vector<std::tuple<int, int>> &polygon,
      const std::string &texturePath,
      TextureTransformation textureTransformation);
  ~Entity();
  int LayerIndex();
  int ZIndex();
  void Draw();

private:
  void CreatePolygonTexture(const std::string &texturePath);
  std::tuple<int, int> m_position;
  std::tuple<int, int> m_size;
  int m_layerIndex;
  std::vector<std::tuple<int, int>> m_polygon;
  Texture m_texture;
};
