#include <raylib.h>
#include <vector>

#include "sprite.h"

int main(int argc, char *argv[])
{
  InitWindow(800, 600, "RTTE");
  SetTargetFPS(60);

  Sprite sprite1("sample/sprite.png", Vector3{10.0, 10.0, 0.0});
  Sprite sprite2("sample/sprite.png", Vector3{60.0, 10.0, 0.0}, Vector2{64.0, 64.0});

  std::vector<Vector2> shape = {
      {0.0, 0.0},
      {32.0, 0.0},
      {64.0, 32.0},
      {32.0, 32.0},
  };

  Sprite sprite3("sample/sprite.png", Vector3{150.0, 10.0, 0.0}, shape);

  while (!WindowShouldClose())
  {
    BeginDrawing();
    ClearBackground(WHITE);

    sprite1.Render();
    sprite2.Render();
    sprite3.Render();

    EndDrawing();
  }

  CloseWindow();
  return 0;
}
