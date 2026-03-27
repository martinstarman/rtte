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
    int id = 0;

    for (const auto &tomlEntity : toml::find<toml::array>(data, "Entities"))
    {
      auto tomlDefaultPosition = toml::find<std::vector<float>>(tomlEntity, "DefaultPosition");
      auto tomlLayerIndex = toml::find<int>(tomlEntity, "LayerIndex");
      auto tomlShowsTraces = toml::find_or<bool>(tomlEntity, "ShowsTraces", false);
      auto tomlPolygon = toml::find<std::vector<std::vector<float>>>(tomlEntity, "Polygon");
      auto tomlTexture = toml::find<toml::value>(tomlEntity, "Texture");
      auto tomlTexturePath = mapFileDir / toml::find<std::string>(tomlTexture, "Path");
      auto tomlTextureFramesInRow = toml::find<int>(tomlTexture, "FramesInRow");
      auto tomlTextureFramesPerSecond = toml::find<int>(tomlTexture, "FramesPerSecond");

      std::string traceTexturePath = "";
      int traceTicksToLive = 0;
      int traceTracesPerSecond = 0;

      if (tomlEntity.contains("Trace"))
      {
        auto tomlTrace = toml::find<toml::value>(tomlEntity, "Trace");
        traceTexturePath = (mapFileDir / toml::find<std::string>(tomlTrace, "TexturePath")).string();
        traceTicksToLive = toml::find<int>(tomlTrace, "TicksToLive");
        traceTracesPerSecond = toml::find<int>(tomlTrace, "TracesPerSecond");
      }

      Vector2 defaultPosition = {tomlDefaultPosition.at(0),
                                 tomlDefaultPosition.at(1)};
      std::vector<Vector2> polygon;
      for (int i = 0; i < tomlPolygon.size(); i++)
      {
        Vector2 polygonPoint = {tomlPolygon.at(i).at(0),
                                tomlPolygon.at(i).at(1)};
        polygon.emplace_back(polygonPoint);
      }

      Config config = {
          id,
          defaultPosition,
          tomlLayerIndex,
          polygon,
          tomlShowsTraces,
      };

      TextureConfig textureConfig = {
          tomlTexturePath.string(),
          tomlTextureFramesInRow,
          tomlTextureFramesPerSecond,
      };

      TraceConfig traceConfig = {
          traceTexturePath,
          traceTicksToLive,
          traceTracesPerSecond,
      };

      game->AddEntity(new Entity(config,
                                 textureConfig,
                                 traceConfig));

      id += 1;
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
