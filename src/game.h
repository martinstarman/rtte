#pragma once

#include <algorithm>
#include <array>
#include <raylib.h>
#include <string>
#include <vector>

#include "entity.h"
#include "navmesh.h"

const float CAMERA_MOVEMENT_SPEED = 5.0;

class Game
{
public:
  Game(float mapWidth, float mapHeight);
  ~Game();
  void Update();
  void Draw();
  void AddEntity(Entity *entity);

private:
  bool m_debug;
  float m_mapWidth;
  float m_mapHeight;
  Camera2D m_camera;
  std::vector<Entity *> m_entities;
  Navmesh *m_navmesh;
  int m_maxDrawingLayer;
  Vector2 GetGameMousePosition() const;
  void HandleCameraOffset();
  bool HandleEntitySelection();
  void HandleEntityMovement();
  void HandleEntityTraces();
  void HandleDebug();
};
