#include <algorithm>
#include <filesystem>
#include <raylib.h>
#include <string>
#include <toml.hpp>
#include <tuple>
#include <vector>

#include "entity.h"

const float CAMERA_MOVEMENT_SPEED = 5.0;

int main(int argc, char *argv[])
{
  InitWindow(800, 600, "RTTE");
  SetTargetFPS(60);

  if (argc < 2)
  {
    TraceLog(LOG_ERROR, "RTTE: map toml file was not specified!");
    CloseWindow();
    return 0;
  }

  int layers = 0;
  std::vector<Entity *> entities;

  try
  {
    std::filesystem::path mapFilePath = argv[1];
    std::filesystem::path mapFileDir = mapFilePath.parent_path();
    auto data = toml::parse(mapFilePath.string(), toml::spec::v(1, 1, 0));

    for (const auto &entity : toml::find<toml::array>(data, "Entities"))
    {
      auto position = toml::find<std::tuple<int, int>>(entity, "Position");
      auto size = toml::find<std::tuple<int, int>>(entity, "Size");
      auto layerIndex = toml::find<int>(entity, "LayerIndex");
      auto polygon = toml::find<std::vector<std::tuple<int, int>>>(entity, "Polygon");
      auto texturePath = mapFileDir / toml::find<std::string>(entity, "TexturePath");

      if (layers < layerIndex)
      {
        layers = layerIndex;
      }

      entities.emplace_back(new Entity(position, size, layerIndex, polygon, texturePath.string()));
    }
  }
  catch (const toml::exception &error)
  {
    TraceLog(LOG_ERROR, error.what());
    CloseWindow();
    return 0;
  }

  Camera2D camera;
  camera = {0};
  camera.target = {0, 0};
  camera.offset = {0, 0};
  camera.rotation = 0;
  camera.zoom = 1;

  while (!WindowShouldClose())
  {
    if (IsKeyDown(KEY_RIGHT))
    {
      camera.offset.x -= CAMERA_MOVEMENT_SPEED;
    }
    else if (IsKeyDown(KEY_LEFT))
    {
      camera.offset.x += CAMERA_MOVEMENT_SPEED;
    }
    else if (IsKeyDown(KEY_UP))
    {
      camera.offset.y += CAMERA_MOVEMENT_SPEED;
    }
    else if (IsKeyDown(KEY_DOWN))
    {
      camera.offset.y -= CAMERA_MOVEMENT_SPEED;
    }

    BeginDrawing();
    ClearBackground(MAGENTA);
    BeginMode2D(camera);

    for (int layerIndex = 0; layerIndex <= layers; layerIndex++)
    {
      std::vector<Entity *> layerEntities = {};

      std::copy_if(entities.begin(), entities.end(), std::back_inserter(layerEntities),
                   [layerIndex](Entity *entity)
                   { return entity->LayerIndex() == layerIndex; });

      std::sort(layerEntities.begin(), layerEntities.end(),
                [](Entity *a, Entity *b)
                { return a->ZIndex() < b->ZIndex(); });

      for (auto &layerEntity : layerEntities)
      {
        layerEntity->Draw();
      }
    }

    EndMode2D();
    EndDrawing();
  }

  for (auto const &entity : entities)
  {
    delete entity;
  }

  CloseWindow();

  return 0;
}
