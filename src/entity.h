#pragma once

#include <algorithm>
#include <cmath>
#include <limits>
#include <raylib.h>
#include <string>
#include "utils.h"
#include <vector>

const float MOVEMENT_SPEED = 2.5;

struct Trace
{
  Vector2 position;
  float rotation;
  int ticks;
};

class Entity
{
public:
  Entity(int id,
         Vector2 position,
         int layerIndex,
         const std::vector<Vector2> &polygon,
         const std::string &texturePath,
         int textureFramesInRow,
         int textureFramesPerSecond,
         bool showsTraces,
         const std::string &traceTexturePath,
         int traceTicksToLive,
         int traceTracesPerSecond);
  ~Entity();
  int GetId();
  int GetLayerIndex();
  float GetZIndex();
  bool GetSelected();
  std::vector<Vector2> GetPolygon();
  Vector2 GetPosition();
  bool GetShowsTraces();
  bool IsMoving();
  void SetSelected(bool selected);
  void SetPath(const std::vector<Vector2> &path);
  void SetTrace();
  void Update();
  void Draw();

private:
  int m_id;
  Vector2 m_position;
  bool m_selected;
  int m_layerIndex;
  std::vector<Vector2> m_polygon;
  Texture m_texture;
  int m_textureFramesInRow;
  int m_textureFramesPerSecond;
  int m_currentTextureFrame;
  int m_animationTicks;
  std::vector<Vector2> m_path;
  bool m_showsTraces;
  Texture m_traceTexture;
  std::vector<Trace> m_traces;
  int m_traceTicksToLive;
  int m_traceTracesPerSecond;
  float m_textureRectangleY;

private:
  void CreatePolygonTexture(const std::string &texturePath);
  void HandleAnimation();
  void HandleMovement();
};
