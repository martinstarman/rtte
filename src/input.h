#pragma once

#include <raylib.h>

class Input
{
public:
  Input();
  ~Input();
  static void ProcessCameraMovement(Camera2D *camera);
};
