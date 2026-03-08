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
      const std::string &id,
      std::tuple<int, int> position,
      std::tuple<int, int> size,
      int layerIndex,
      const std::vector<std::tuple<int, int>> &polygon,
      bool selectable,
      const std::string &texturePath,
      TextureTransformation textureTransformation,
      int textureFrames,
      int textureFramesPerSecond);
  ~Entity();
  const std::string &Id();
  int LayerIndex();
  int ZIndex();
  void Update();
  void Draw();
  bool Selectable();
  void Selected(bool selected);
  std::vector<Vector2> Polygon();

private:
  void CreatePolygonTexture(const std::string &texturePath);
  void Animate();
  std::string m_id;
  std::tuple<int, int> m_position;
  bool m_selectable;
  bool m_selected;
  std::tuple<int, int> m_size;
  int m_layerIndex;
  std::vector<std::tuple<int, int>> m_polygon;
  Texture m_texture;
  int m_textureFrames;
  int m_textureFramesPerSecond;
  int m_textureFrame;
  int m_frames;
};
