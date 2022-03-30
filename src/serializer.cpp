#include "serializer.h"

#include "enemy.h"
#include "entity.h"
#include <filesystem>
#include "game.h"
#include "character.h"
#include "object.h"
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

        std::filesystem::path path = file;
        std::string dir = path.parent_path().string();

        std::vector<NavMesh::Polygon> polygons;
        std::vector<Entity *> entities;
        std::vector<Character *> characters;
        std::vector<Enemy *> enemies;
        std::vector<Object *> objects;

        for (const auto &character : data["characters"])
        {
            float x = character["position"][0].as<float>();
            float y = character["position"][1].as<float>();

            Character *entity = new Character(x, y);

            entities.emplace_back(entity);
            characters.emplace_back(entity);
        }

        for (const auto &enemy : data["enemies"])
        {
            float x = enemy["position"][0].as<float>();
            float y = enemy["position"][1].as<float>();

            Enemy *entity = new Enemy(x, y);

            entities.emplace_back(entity);
            enemies.emplace_back(entity);
        }

        for (const auto &object : data["objects"])
        {
            float x = object["position"][0].as<float>();
            float y = object["position"][1].as<float>();
            // TODO: separator?
            std::string image = dir + "/" + object["image"].as<std::string>();

            NavMesh::Polygon polygon;

            for (const auto &point : object["polygon"])
            {
                int px = point[0].as<int>();
                int py = point[1].as<int>();

                polygon.AddPoint(x + px, y + py);
            }

            if (polygon.Size() > 0)
            {
                polygons.emplace_back(polygon);
            }

            Object *entity = new Object(x, y, image);

            entities.emplace_back(entity);
            objects.emplace_back(entity);
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
            .objects = objects,
        };

        Game::Get()->SetGameData(gameData);
    };
}