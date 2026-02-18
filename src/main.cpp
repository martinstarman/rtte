#include <algorithm>
#include <raylib.h>
#include <string>
#include <toml.hpp>
#include <tuple>
#include <vector>

#include "entity.h"

int main(void)
{
  InitWindow(800, 600, "RTTE");

  auto data = toml::parse("../example/example.toml", toml::spec::v(1, 1, 0));
  int layers = 0;
  std::vector<Entity> entities;

  try
  {
    for (const auto &entity : toml::find<toml::array>(data, "Entities"))
    {
      auto position = toml::find<std::tuple<int, int>>(entity, "Position");
      auto size = toml::find<std::tuple<int, int>>(entity, "Size");
      auto layerIndex = toml::find<int>(entity, "LayerIndex");
      auto polygon = toml::find<std::vector<std::tuple<int, int>>>(entity, "Polygon");

      if (layers < layerIndex)
      {
        layers = layerIndex;
      }

      entities.emplace_back(position, size, layerIndex, polygon);
    }
  }
  catch (const toml::exception &error)
  {
    TraceLog(LOG_ERROR, error.what());
  }

  while (!WindowShouldClose())
  {
    BeginDrawing();
    ClearBackground(MAGENTA);

    for (int layerIndex = 0; layerIndex <= layers; layerIndex++)
    {
      std::vector<Entity> layerEntities;

      std::copy_if(entities.begin(), entities.end(), std::back_inserter(layerEntities),
                   [layerIndex](Entity entity)
                   { return entity.LayerIndex() == layerIndex; });

      std::sort(layerEntities.begin(), layerEntities.end(),
                [](Entity &a, Entity &b)
                { return a.ZIndex() < b.ZIndex(); });

      for (auto &layerEntity : layerEntities)
      {
        layerEntity.Draw();
      }
    }

    EndDrawing();
  }

  CloseWindow();

  return 0;
}
