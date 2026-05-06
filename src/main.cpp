#include <filesystem>
#include <raylib.h>
#include <string>
#include <toml.hpp>
#include <vector>

#include "entity.h"
#include "game.h"
#include "utils.h"

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
      auto tomlDefaultOctant = toml::find_or<std::string>(tomlEntity, "DefaultOctant", "East");
      auto tomlDefaultPosition = toml::find<std::vector<float>>(tomlEntity, "DefaultPosition");
      auto tomlDrawingLayer = toml::find<int>(tomlEntity, "DrawingLayer");
      auto tomlShowsTraces = toml::find_or<bool>(tomlEntity, "ShowsTraces", false);
      auto tomlName = toml::find_or<std::string>(tomlEntity, "Name", "");
      auto tomlShape = toml::find<std::vector<std::vector<float>>>(tomlEntity, "Shape");
      auto tomlEntityTexture = toml::find<toml::value>(tomlEntity, "EntityTexture");
      auto tomlTexturePath = mapFileDir / toml::find<std::string>(tomlEntityTexture, "Path");
      auto tomlTextureFramesInRow = toml::find<int>(tomlEntityTexture, "FramesInRow");
      auto tomlTextureFramesPerSecond = toml::find<int>(tomlEntityTexture, "FramesPerSecond");

      std::string traceTexturePath = "";
      int traceTextureTracesPerSecond = 0;

      if (tomlEntity.contains("TraceTexture"))
      {
        auto tomlTraceTexture = toml::find<toml::value>(tomlEntity, "TraceTexture");
        traceTexturePath = (mapFileDir / toml::find<std::string>(tomlTraceTexture, "Path")).string();
        traceTextureTracesPerSecond = toml::find<int>(tomlTraceTexture, "TracesPerSecond");
      }

      Vector2 defaultPosition = {tomlDefaultPosition.at(0),
                                 tomlDefaultPosition.at(1)};
      std::vector<Vector2> shape;
      for (int i = 0; i < tomlShape.size(); i++)
      {
        Vector2 shapePoint = {tomlShape.at(i).at(0),
                              tomlShape.at(i).at(1)};
        shape.emplace_back(shapePoint);
      }

      EntityConfig config = {
          id,
          defaultPosition,
          tomlDrawingLayer,
          shape,
          tomlShowsTraces,
          GetOctantFrom(tomlDefaultOctant),
          tomlName,
      };

      EntityTextureConfig textureConfig = {
          tomlTexturePath.string(),
          tomlTextureFramesInRow,
          tomlTextureFramesPerSecond,
      };

      TraceTextureConfig traceConfig = {
          traceTexturePath,
          traceTextureTracesPerSecond,
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
