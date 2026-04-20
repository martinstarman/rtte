#pragma once

#include <algorithm>
#include <raylib.h>
#include <string>
#include <vector>

#include "entity.h"
#include "navmesh.h"

const float CAMERA_MOVEMENT_SPEED = 5.0;

class Game
{
public:
  Game();
  ~Game();
  void Update();
  void Draw();
  void AddEntity(Entity *entity);

private:
  Camera2D m_camera;
  std::vector<Entity *> m_entities;
  int m_maxDrawingLayer;
  Vector2 GetGameMousePosition();
  void HandleCameraOffset();
  bool HandleEntitySelection();
  void HandleEntityMovement();
  void HandleEntityTraces();

  Navmesh *m_navmesh;
};
