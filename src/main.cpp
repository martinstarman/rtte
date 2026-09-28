#include <raylib.h>
#include <vector>

#include "animation.h"
#include "sprite.h"

int main(int argc, char *argv[])
{
  InitWindow(800, 600, "RTTE");
  SetTargetFPS(60);

  std::vector<Vector2> shape = {
      {0.0, 0.0},
      {32.0, 0.0},
      {64.0, 32.0},
      {32.0, 32.0},
  };
  Animation animation = {2, {32.0, 32.0}, 1.0f};

  Sprite sprite1("sample/sprite1.png", Vector3{10.0, 10.0, 0.0});
  Sprite sprite2("sample/sprite1.png", Vector3{60.0, 10.0, 0.0}, Vector2{64.0, 64.0});
  Sprite sprite3("sample/sprite1.png", Vector3{150.0, 10.0, 0.0}, shape);
  Sprite sprite4("sample/sprite2.png", Vector3{10.0, 60.0, 0.0}, animation);
  Sprite sprite5("sample/sprite2.png", Vector3{60.0, 84.0, 0.0}, Vector2{64.0, 64.0}, animation);
  Sprite sprite6("sample/sprite2.png", Vector3{150.0, 60.0, 0.0}, shape, animation);

  while (!WindowShouldClose())
  {
    BeginDrawing();
    ClearBackground(WHITE);

    float dt = GetFrameTime();

    sprite1.Update(dt);
    sprite2.Update(dt);
    sprite3.Update(dt);
    sprite4.Update(dt);
    sprite5.Update(dt);
    sprite6.Update(dt);

    sprite1.Render();
    sprite2.Render();
    sprite3.Render();
    sprite4.Render();
    sprite5.Render();
    sprite6.Render();

    EndDrawing();
  }

  CloseWindow();
  return 0;
}
