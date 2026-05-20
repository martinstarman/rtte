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

  Game *game;

  try
  {
    std::filesystem::path mapFilePath = argv[1];
    std::filesystem::path mapFileDir = mapFilePath.parent_path();
    auto tomlData = toml::parse(mapFilePath.string(), toml::spec::v(1, 1, 0));
    int entityId = 0;

    auto tomlMap = toml::find<toml::value>(tomlData, "Map");
    auto tomlMapWidth = toml::find_or<float>(tomlMap, "Width", 800.0);
    auto tomlMapHeight = toml::find_or<float>(tomlMap, "Height", 600.0);

    game = new Game(tomlMapWidth, tomlMapHeight);

    for (const auto &tomlEntity : toml::find<toml::array>(tomlData, "Entities"))
    {
      auto tomlEntityDefaultOctant = toml::find_or<std::string>(tomlEntity, "DefaultOctant", "East");
      auto tomlEntityDefaultPosition = toml::find_or<std::vector<float>>(tomlEntity, "DefaultPosition", {0.0, 0.0});
      auto tomlEntityShowsTraces = toml::find_or<bool>(tomlEntity, "ShowsTraces", false);

      auto tomlEntityTexture = toml::find<toml::value>(tomlEntity, "EntityTexture");
      auto tomlEntityTextureDrawingLayer = toml::find_or<int>(tomlEntityTexture, "DrawingLayer", 0);
      auto tomlEntityTexturePath = toml::find_or<std::string>(tomlEntityTexture, "Path", "");
      auto tomlEntityTextureFramesInRow = toml::find_or<int>(tomlEntityTexture, "FramesInRow", 1);
      auto tomlEntityTextureFramesPerSecond = toml::find_or<int>(tomlEntityTexture, "FramesPerSecond", 1);
      auto tomlEntityTextureFill = toml::find_or<bool>(tomlEntityTexture, "Fill", false);

      auto tomlEntityShape = toml::find<toml::value>(tomlEntity, "Shape");
      auto tomlEntityShapePoints = toml::find_or<std::vector<std::vector<float>>>(tomlEntityShape, "Points", {});
      auto tomlEntityShapeBlocksMovement = toml::find_or<bool>(tomlEntityShape, "BlocksMovement", false);

      std::string entityTraceTexturePath = "";
      int entityTraceTracesPerSecond = 0;

      if (tomlEntity.contains("Trace"))
      {
        auto tomlEntityTrace = toml::find<toml::value>(tomlEntity, "Trace");
        entityTraceTexturePath = (mapFileDir / toml::find_or<std::string>(tomlEntityTrace, "TexturePath", "")).string();
        entityTraceTracesPerSecond = toml::find_or<int>(tomlEntityTrace, "TracesPerSecond", 0);
      }

      Vector2 entityDefaultPosition = {tomlEntityDefaultPosition.at(0),
                                       tomlEntityDefaultPosition.at(1)};
      std::vector<Vector2> entityShape;
      for (size_t i = 0; i < tomlEntityShapePoints.size(); ++i)
      {
        Vector2 v = {tomlEntityShapePoints.at(i).at(0),
                     tomlEntityShapePoints.at(i).at(1)};
        entityShape.emplace_back(v);
      }

      std::string entityTexturePath = tomlEntityTexturePath == ""
                                          ? ""
                                          : (mapFileDir / tomlEntityTexturePath).string();

      EntityConfig entityConfig = {
          entityId,
          entityDefaultPosition,
          tomlEntityShowsTraces,
          GetOctantFrom(tomlEntityDefaultOctant),
      };

      EntityTextureConfig entityTextureConfig = {
          entityTexturePath,
          tomlEntityTextureDrawingLayer,
          tomlEntityTextureFramesInRow,
          tomlEntityTextureFramesPerSecond,
          tomlEntityTextureFill,
      };

      EntityTraceConfig entityTraceConfig = {
          entityTraceTexturePath,
          entityTraceTracesPerSecond,
      };

      EntityShapeConfig entityShapeConfig = {
          entityShape,
          tomlEntityShapeBlocksMovement,
      };

      EntityMovementConfig entityMovementConfig = {
          tomlEntity.contains("Movement"),
      };

      game->AddEntity(new Entity(
          entityConfig,
          entityTextureConfig,
          entityTraceConfig,
          entityShapeConfig,
          entityMovementConfig));

      entityId += 1;
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
