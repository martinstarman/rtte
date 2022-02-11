#include "serializer.h"

#include "game.h"
#include <string>
#include <yaml-cpp/yaml.h>

namespace rtte
{
    Serializer::Serializer() = default;

    Serializer::~Serializer() = default;

    void Serializer::Deserialize(const std::string file)
    {
        YAML::Node data = YAML::LoadFile(file);

        GameData gameData = {
            .debug = data["debug"].as<bool>(),
            .missionName = data["name"].as<std::string>(),
            .mapWidth = data["map width"].as<int>(),
            .mapHeight = data["map height"].as<int>()
        };

        Game::Get()->SetGameData(gameData);
    };
}