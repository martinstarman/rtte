#include <filesystem>
#include <fstream>
#include <gtest/gtest.h>

#include "mission.h"

TEST(MissionTest, ParsesMapSize)
{
  Mission mission("sample/sample.toml");
  Vector2 size = mission.GetMapSize();

  EXPECT_FLOAT_EQ(size.x, 1000.0f);
  EXPECT_FLOAT_EQ(size.y, 800.0f);
}

TEST(MissionTest, MapSizeDefaultsWhenMissing)
{
  std::filesystem::path path = std::filesystem::temp_directory_path() / "rtte_mission_empty.toml";
  std::ofstream(path) << "";

  Mission mission(path.string());
  Vector2 size = mission.GetMapSize();

  EXPECT_FLOAT_EQ(size.x, 800.0f);
  EXPECT_FLOAT_EQ(size.y, 600.0f);

  std::filesystem::remove(path);
}
