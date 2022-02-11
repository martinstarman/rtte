#include "serializer.h"

#include "game.h"
#include <Polygon.h>
#include <string>
#include <vector>
#include <yaml-cpp/yaml.h>

namespace rtte
{
    Serializer::Serializer() = default;

    Serializer::~Serializer() = default;

    void Serializer::Deserialize(const std::string file)
    {
        YAML::Node data = YAML::LoadFile(file);

        std::vector<NavMesh::Polygon> polygons;

        for (const auto &points : data["polygons"])
        {
            NavMesh::Polygon polygon;

            for (const auto &point : points)
            {
                int x = point[0].as<int>();
                int y = point[1].as<int>();

                polygon.AddPoint(x, y);
            }

            polygons.emplace_back(polygon);
        }

        GameData gameData = {
            .debug = data["debug"].as<bool>(),
            .missionName = data["name"].as<std::string>(),
            .mapWidth = data["map width"].as<int>(),
            .mapHeight = data["map height"].as<int>(),
            .polygons = polygons};

        Game::Get()->SetGameData(gameData);
    };
}