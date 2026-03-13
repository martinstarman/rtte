#pragma once

#include <cmath>
#include <limits>
#include <raylib.h>
#include <string>
#include <vector>

const float MOVEMENT_SPEED = 2.5;

enum TextureTransformation
{
  None,
  Fill,
};

class Entity
{
public:
  Entity(const std::string &id,
         Vector2 position,
         Vector2 size,
         int layerIndex,
         const std::vector<Vector2> &polygon,
         bool selectable,
         const std::string &texturePath,
         TextureTransformation textureTransformation,
         int textureFrames,
         int textureFramesPerSecond);
  ~Entity();
  const std::string &GetId();
  int GetLayerIndex();
  float GetZIndex();
  bool GetSelectable();
  bool GetSelected();
  std::vector<Vector2> GetPolygon();
  void SetSelected(bool selected);
  void SetPath(const std::vector<Vector2> &path);
  void Update();
  void Draw();

private:
  std::string m_id;
  Vector2 m_position;
  bool m_selectable;
  bool m_selected;
  Vector2 m_size;
  int m_layerIndex;
  std::vector<Vector2> m_polygon;
  Texture m_texture;
  int m_textureFrames;
  int m_textureFramesPerSecond;
  int m_textureFrame;
  int m_frames;
  std::vector<Vector2> m_path;
  void CreatePolygonTexture(const std::string &texturePath);
  void HandleAnimation();
  void HandleMovement();
};
