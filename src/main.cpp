#include <raylib.h>

#include "mission.h"

int main(int argc, char *argv[])
{
  InitWindow(800, 600, "RTTE");
  SetTargetFPS(60);

  Mission mission("sample/sample.toml");

  while (!WindowShouldClose())
  {
    BeginDrawing();
    ClearBackground(DARKGRAY);

    float dt = GetFrameTime();

    mission.Update(dt);
    mission.Render();

    EndDrawing();
  }

  CloseWindow();
  return 0;
}
