#pragma once

#include <algorithm>
#include <cmath>
#include <limits>
#include <raylib.h>
#include <string>
#include <vector>

#include "utils.h"

const float MOVEMENT_SPEED = 2.5;

struct Config
{
  int id;
  Vector2 defaultPosition;
  int drawingLayer;
  std::vector<Vector2> shape;
  bool showsTraces;
  Octant defaultOctant;
};

struct TextureConfig
{
  std::string path;
  int framesInRow;
  int framesPerSecond;
};

struct TraceConfig
{
  std::string path;
  int ticksToLive;
  int tracesPerSecond;
};

struct Trace
{
  Vector2 position;
  float rotation;
  int ticks;
};

class Entity
{
public:
  Entity(
      const Config &config,
      const TextureConfig &textureConfig,
      const TraceConfig &traceConfig);
  ~Entity();
  int GetId();
  int GetDrawingLayer();
  float GetZIndex();
  bool GetSelected();
  std::vector<Vector2> GetShape();
  Vector2 GetPosition();
  bool GetShowsTraces();
  bool IsMoving();
  void SetSelected(bool selected);
  void SetPath(const std::vector<Vector2> &path);
  void SetTrace();
  void Update();
  void Draw();

private:
  Config m_config;
  Vector2 m_position;
  bool m_selected;
  std::vector<Vector2> m_path;
  Octant m_octant;
  TextureConfig m_textureConfig;
  Texture m_texture;
  int m_currentTextureFrame;
  int m_animationTicks;
  TraceConfig m_traceConfig;
  Texture m_traceTexture;
  std::vector<Trace> m_traces;

private:
  void CreatePolygonTexture();
  void HandleAnimation();
  void HandleMovement();
};
