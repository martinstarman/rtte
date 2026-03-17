#include <filesystem>
#include <raylib.h>
#include <string>
#include <toml.hpp>
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
      auto tomlId = toml::find<std::string>(tomlEntity, "Id");
      auto tomlPosition = toml::find<std::vector<float>>(tomlEntity, "Position");
      auto tomlSelectable = toml::find<bool>(tomlEntity, "Selectable");
      auto tomlSize = toml::find<std::vector<float>>(tomlEntity, "Size");
      auto tomlLayerIndex = toml::find<int>(tomlEntity, "LayerIndex");
      auto tomlLeavesTraces = toml::find<bool>(tomlEntity, "LeavesTraces");
      auto tomlPolygon = toml::find<std::vector<std::vector<float>>>(tomlEntity, "Polygon");
      auto tomlTexture = toml::find<toml::value>(tomlEntity, "Texture");
      auto tomlTexturePath = mapFileDir / toml::find<std::string>(tomlTexture, "Path");
      auto tomlTextureTransformation = toml::find<std::string>(tomlTexture, "Transformation");
      auto tomlAnimation = toml::find<toml::value>(tomlTexture, "Animation");
      auto tomlTextureAnimationFrames = toml::find<int>(tomlAnimation, "Frames");
      auto tomlTextureAnimationFramesPerSecond = toml::find<int>(tomlAnimation, "FramesPerSecond");
      auto tomlTrace = toml::find<toml::value>(tomlEntity, "Trace");
      auto tomlTraceTexturePath = toml::find<std::string>(tomlTrace, "TexturePath");

      Vector2 position = {tomlPosition.at(0), tomlPosition.at(1)};
      Vector2 size = {tomlSize.at(0), tomlSize.at(1)};
      std::vector<Vector2> polygon;
      for (int i = 0; i < tomlPolygon.size(); i++)
      {
        Vector2 polygonPoint = {tomlPolygon.at(i).at(0),
                                tomlPolygon.at(i).at(1)};
        polygon.emplace_back(polygonPoint);
      }
      std::string traceTexturePath = tomlTraceTexturePath == ""
                                         ? ""
                                         : (mapFileDir / tomlTraceTexturePath).string();

      game->AddEntity(new Entity(tomlId,
                                 position,
                                 size,
                                 tomlLayerIndex,
                                 polygon,
                                 tomlSelectable,
                                 tomlTexturePath.string(),
                                 tomlTextureTransformation == "fill"
                                     ? TextureTransformation::Fill
                                     : TextureTransformation::None,
                                 tomlTextureAnimationFrames,
                                 tomlTextureAnimationFramesPerSecond,
                                 tomlLeavesTraces,
                                 traceTexturePath));
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
