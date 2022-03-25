#include "serializer.h"

#include "enemy.h"
#include "entity.h"
#include "game.h"
#include "character.h"
#include <Polygon.h>
#include <string>
#include <vector>
#include <yaml-cpp/yaml.h>

namespace rtte
{
    Serializer::Serializer() = default;

    Serializer::~Serializer() = default;

    void Serializer::Deserialize(const std::string &file)
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

        std::vector<Entity *> entities;
        std::vector<Character *> characters;
        std::vector<Enemy *> enemies;

        for (const auto &character : data["characters"])
        {
            float x = character["position"][0].as<float>();
            float y = character["position"][1].as<float>();

            Character *entity = new Character(x, y, polygons);

            entities.emplace_back(entity);
            characters.emplace_back(entity);
        }

        for (const auto &enemy : data["enemies"])
        {
            float x = enemy["position"][0].as<float>();
            float y = enemy["position"][1].as<float>();

            Enemy *entity = new Enemy(x, y, polygons);

            entities.emplace_back(entity);
            enemies.emplace_back(entity);
        }

        GameData gameData = {
            .debug = data["debug"].as<bool>(),
            .missionName = data["name"].as<std::string>(),
            .mapWidth = data["map width"].as<int>(),
            .mapHeight = data["map height"].as<int>(),
            .polygons = polygons,
            .entities = entities,
            .characters = characters,
            .enemies = enemies,
        };

        Game::Get()->SetGameData(gameData);
    };
}