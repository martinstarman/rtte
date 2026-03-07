#include "input.h"

const float CAMERA_MOVEMENT_SPEED = 5.0;

Input::Input() = default;

Input::~Input() = default;

void Input::ProcessCameraMovement(Camera2D *camera)
{
  if (IsKeyDown(KEY_RIGHT))
  {
    camera->offset.x -= CAMERA_MOVEMENT_SPEED;
  }
  if (IsKeyDown(KEY_LEFT))
  {
    camera->offset.x += CAMERA_MOVEMENT_SPEED;
  }
  if (IsKeyDown(KEY_UP))
  {
    camera->offset.y += CAMERA_MOVEMENT_SPEED;
  }
  if (IsKeyDown(KEY_DOWN))
  {
    camera->offset.y -= CAMERA_MOVEMENT_SPEED;
  }
}
