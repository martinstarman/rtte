#include <raylib.h>
#include <vector>

#include "animation.h"
#include "navmesh.h"
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

  std::vector<std::vector<Vector2>> holes = {
      {{120.0, 220.0}, {220.0, 200.0}, {240.0, 300.0}, {140.0, 320.0}},
      {{320.0, 260.0}, {400.0, 340.0}, {320.0, 420.0}, {260.0, 340.0}},
      {{460.0, 180.0}, {560.0, 180.0}, {560.0, 400.0}, {460.0, 400.0}},
      {{620.0, 300.0}, {700.0, 260.0}, {740.0, 340.0}, {680.0, 420.0}, {610.0, 390.0}},
      {{180.0, 440.0}, {300.0, 470.0}, {200.0, 540.0}},
      {{600.0, 450.0}, {630.0, 450.0}, {630.0, 550.0}, {600.0, 550.0}},
      {{630.0, 520.0}, {740.0, 520.0}, {740.0, 550.0}, {630.0, 550.0}},
  };
  Navmesh navmesh(Rectangle{20.0, 160.0, 760.0, 420.0}, holes, 8.0f);
  Vector2 start = {60.0, 360.0};

  while (!WindowShouldClose())
  {
    BeginDrawing();
    ClearBackground(DARKGRAY);

    float dt = GetFrameTime();

    if (IsMouseButtonPressed(MOUSE_BUTTON_LEFT))
    {
      start = GetMousePosition();
    }
    std::vector<Vector2> path = navmesh.FindPath(start, GetMousePosition());

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

    navmesh.Render();
    for (size_t i = 1; i < path.size(); ++i)
    {
      DrawLineEx(path.at(i - 1), path.at(i), 2.0f, RED);
    }
    DrawCircleV(start, 4.0f, GREEN);

    EndDrawing();
  }

  CloseWindow();
  return 0;
}
