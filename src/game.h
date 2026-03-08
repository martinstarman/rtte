#pragma once

#include <algorithm>
#include <raylib.h>
#include <string>
#include <vector>

#include "entity.h"

class Game
{
public:
  Game();
  ~Game();
  void Update();
  void Draw();
  void AddEntity(Entity *entity);

private:
  void ProcessCameraMovement();
  void ProcessEntitySelection();
  Camera2D m_camera;
  std::vector<Entity *> m_entities;
  int m_maxLayerIndex;
};
