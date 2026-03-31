#pragma once

#include <algorithm>
#include <cmath>
#include <limits>
#include <raylib.h>
#include <string>
#include <vector>

#include "utils.h"

const float MOVEMENT_SPEED = 2.5;
const int TRACE_VISIBILITY_TICKS_COUNT = 100;

struct EntityConfig
{
  int id;
  Vector2 defaultPosition;
  int drawingLayer;
  std::vector<Vector2> shape;
  bool showsTraces;
  Octant defaultOctant;
  std::string ability;
  std::string needsAbility;
};

struct EntityTextureConfig
{
  std::string path;
  int framesInRow;
  int framesPerSecond;
};

struct TraceTextureConfig
{
  std::string path;
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
      const EntityConfig &entityConfig,
      const EntityTextureConfig &entityTextureConfig,
      const TraceTextureConfig &traceTextureConfig);
  ~Entity();
  int GetId();
  int GetDrawingLayer();
  float GetZIndex();
  bool GetSelected();
  std::vector<Vector2> GetShape();
  Vector2 GetPosition();
  bool GetShowsTraces();
  const std::string &GetAbility();
  const std::string &GetNeedsAbility();
  bool IsMoving();
  void SetSelected(bool selected);
  void SetPath(const std::vector<Vector2> &path);
  void SetTrace();
  void Update();
  void Draw();

private:
  EntityConfig m_entityConfig;
  Vector2 m_position;
  bool m_selected;
  std::vector<Vector2> m_path;
  Octant m_octant;

private:
  EntityTextureConfig m_entityTextureConfig;
  Texture m_texture;
  int m_currentTextureFrame;
  int m_animationTicks;

private:
  TraceTextureConfig m_traceTextureConfig;
  Texture m_traceTexture;
  std::vector<Trace> m_traces;

private:
  void CreatePolygonTexture();
  void HandleAnimation();
  void HandleMovement();
  void RemoveOldTraces();
};
