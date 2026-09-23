#include <raylib.h>

#include "sprite.h"

int main(int argc, char *argv[])
{
  InitWindow(800, 600, "RTTE");
  SetTargetFPS(60);

  Sprite sprite("sample/sprite.png", Vector2{32.0, 32.0}, Vector2{10.0, 10.0});

  while (!WindowShouldClose())
  {
    BeginDrawing();
    ClearBackground(WHITE);
    sprite.Render();
    EndDrawing();
  }

  CloseWindow();
  return 0;
}
