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

    for (const auto &tomlEntity : toml::find<toml::array>(data, "Entities"))
    {
      TextureTransformation textureTransformation = TextureTransformation::None;

      auto tomlPosition = toml::find<std::tuple<int, int>>(tomlEntity, "Position");
      auto tomlSize = toml::find<std::tuple<int, int>>(tomlEntity, "Size");
      auto tomlLayerIndex = toml::find<int>(tomlEntity, "LayerIndex");
      auto tomlPolygon = toml::find<std::vector<std::tuple<int, int>>>(tomlEntity, "Polygon");
      auto tomlTexture = toml::find<toml::value>(tomlEntity, "Texture");
      auto tomlTexturePath = mapFileDir / toml::find<std::string>(tomlTexture, "Path");
      auto tomlTextureTransformation = toml::find<std::string>(tomlTexture, "Transformation");
      auto tomlAnimation = toml::find<toml::value>(tomlTexture, "Animation");
      auto tomlTextureAnimationFrames = toml::find<int>(tomlAnimation, "Frames");
      auto tomlTextureAnimationFramesPerSecond = toml::find<int>(tomlAnimation, "FramesPerSecond");

      if (layers < tomlLayerIndex)
      {
        layers = tomlLayerIndex;
      }

      if (tomlTextureTransformation == "fill")
      {
        textureTransformation = TextureTransformation::Fill;
      }

      entities.emplace_back(new Entity(
          tomlPosition,
          tomlSize,
          tomlLayerIndex,
          tomlPolygon,
          tomlTexturePath.string(),
          textureTransformation,
          tomlTextureAnimationFrames,
          tomlTextureAnimationFramesPerSecond));
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
    if (IsKeyDown(KEY_LEFT))
    {
      camera.offset.x += CAMERA_MOVEMENT_SPEED;
    }
    if (IsKeyDown(KEY_UP))
    {
      camera.offset.y += CAMERA_MOVEMENT_SPEED;
    }
    if (IsKeyDown(KEY_DOWN))
    {
      camera.offset.y -= CAMERA_MOVEMENT_SPEED;
    }

    for (auto &entity : entities)
    {
      entity->Update();
    }

    BeginDrawing();
    ClearBackground(MAGENTA);
    BeginMode2D(camera);

    std::sort(entities.begin(), entities.end(),
              [](Entity *a, Entity *b)
              { return a->ZIndex() < b->ZIndex(); });

    for (int layerIndex = 0; layerIndex <= layers; layerIndex++)
    {
      for (auto &entity : entities)
      {
        if (entity->LayerIndex() == layerIndex)
        {
          entity->Draw();
        }
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
