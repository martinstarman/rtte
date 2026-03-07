#include <filesystem>
#include <raylib.h>
#include <string>
#include <toml.hpp>
#include <tuple>
#include <vector>

#include "entity.h"
#include "game.h"

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

  Game *game = new Game();

  try
  {
    std::filesystem::path mapFilePath = argv[1];
    std::filesystem::path mapFileDir = mapFilePath.parent_path();
    auto data = toml::parse(mapFilePath.string(), toml::spec::v(1, 1, 0));

    for (const auto &tomlEntity : toml::find<toml::array>(data, "Entities"))
    {
      auto tomlPosition = toml::find<std::tuple<int, int>>(tomlEntity, "Position");
      auto tomlSelectable = toml::find<bool>(tomlEntity, "Selectable");
      auto tomlSize = toml::find<std::tuple<int, int>>(tomlEntity, "Size");
      auto tomlLayerIndex = toml::find<int>(tomlEntity, "LayerIndex");
      auto tomlPolygon = toml::find<std::vector<std::tuple<int, int>>>(tomlEntity, "Polygon");
      auto tomlTexture = toml::find<toml::value>(tomlEntity, "Texture");
      auto tomlTexturePath = mapFileDir / toml::find<std::string>(tomlTexture, "Path");
      auto tomlTextureTransformation = toml::find<std::string>(tomlTexture, "Transformation");
      auto tomlAnimation = toml::find<toml::value>(tomlTexture, "Animation");
      auto tomlTextureAnimationFrames = toml::find<int>(tomlAnimation, "Frames");
      auto tomlTextureAnimationFramesPerSecond = toml::find<int>(tomlAnimation, "FramesPerSecond");

      game->AddEntity(new Entity(
          tomlPosition,
          tomlSize,
          tomlLayerIndex,
          tomlPolygon,
          tomlSelectable,
          tomlTexturePath.string(),
          tomlTextureTransformation == "fill"
              ? TextureTransformation::Fill
              : TextureTransformation::None,
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

  while (!WindowShouldClose())
  {
    game->Update();
    game->Draw();
  }

  delete game;
  CloseWindow();
  return 0;
}
