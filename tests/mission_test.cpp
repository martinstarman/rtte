#include <filesystem>
#include <fstream>
#include <gtest/gtest.h>

#include "mission.h"

class MissionTest : public ::testing::Test
{
protected:
  static void SetUpTestSuite()
  {
    SetConfigFlags(FLAG_WINDOW_HIDDEN);
    InitWindow(1, 1, "RTTE tests");
  }

  static void TearDownTestSuite()
  {
    CloseWindow();
  }
};

TEST_F(MissionTest, ParsesMapSize)
{
  Mission mission("sample/sample.toml");
  Vector2 size = mission.GetMapSize();

  EXPECT_FLOAT_EQ(size.x, 1000.0f);
  EXPECT_FLOAT_EQ(size.y, 800.0f);
}

TEST_F(MissionTest, MapSizeDefaultsWhenMissing)
{
  std::filesystem::path path = std::filesystem::temp_directory_path() / "rtte_mission_empty.toml";
  std::ofstream(path) << "";

  Mission mission(path.string());
  Vector2 size = mission.GetMapSize();

  EXPECT_FLOAT_EQ(size.x, 800.0f);
  EXPECT_FLOAT_EQ(size.y, 600.0f);

  std::filesystem::remove(path);
}

TEST_F(MissionTest, ParsesCharacters)
{
  Mission mission("sample/sample.toml");
  const std::vector<Character> &characters = mission.GetCharacters();

  ASSERT_EQ(characters.size(), 2);
  EXPECT_FLOAT_EQ(characters.at(0).GetPosition().x, 50.0f);
  EXPECT_FLOAT_EQ(characters.at(0).GetPosition().y, 50.0f);
  EXPECT_FLOAT_EQ(characters.at(1).GetPosition().x, 80.0f);
  EXPECT_FLOAT_EQ(characters.at(1).GetPosition().y, 50.0f);
}

TEST_F(MissionTest, SkipsCharacterWithoutPosition)
{
  std::filesystem::path path = std::filesystem::temp_directory_path() / "rtte_mission_characters.toml";
  std::ofstream(path) << "[[Character]]\n\n[[Character]]\nPosition = [10.0, 20.0, 0.0]\n";

  Mission mission(path.string());
  const std::vector<Character> &characters = mission.GetCharacters();

  ASSERT_EQ(characters.size(), 1);
  EXPECT_FLOAT_EQ(characters.at(0).GetPosition().x, 10.0f);
  EXPECT_FLOAT_EQ(characters.at(0).GetPosition().y, 20.0f);

  std::filesystem::remove(path);
}

TEST_F(MissionTest, NoCharactersWhenMissing)
{
  std::filesystem::path path = std::filesystem::temp_directory_path() / "rtte_mission_no_characters.toml";
  std::ofstream(path) << "";

  Mission mission(path.string());

  EXPECT_TRUE(mission.GetCharacters().empty());

  std::filesystem::remove(path);
}

TEST_F(MissionTest, ParsesSprites)
{
  Mission mission("sample/sample.toml");
  const std::vector<Sprite> &sprites = mission.GetSprites();

  ASSERT_EQ(sprites.size(), 6);
  EXPECT_EQ(sprites.at(0).GetTexture().width, 32);
  EXPECT_EQ(sprites.at(1).GetTexture().width, 64);
  EXPECT_EQ(sprites.at(2).GetTexture().width, 64);
  EXPECT_EQ(sprites.at(3).GetTexture().width, 64);
  EXPECT_EQ(sprites.at(4).GetTexture().width, 128);
  EXPECT_EQ(sprites.at(5).GetTexture().width, 128);

  for (const Sprite &sprite : sprites)
  {
    EXPECT_TRUE(IsTextureValid(sprite.GetTexture()));
  }
}

TEST_F(MissionTest, SkipsSpriteWithoutPathOrPosition)
{
  std::filesystem::path path = std::filesystem::temp_directory_path() / "rtte_mission_sprites.toml";
  std::ofstream(path) << "[[Sprite]]\nPosition = [0.0, 0.0, 0.0]\n\n[[Sprite]]\nPath = \"sprite1.png\"\n";

  Mission mission(path.string());

  EXPECT_TRUE(mission.GetSprites().empty());

  std::filesystem::remove(path);
}

TEST_F(MissionTest, ParsesObstacleBlocking)
{
  std::filesystem::path path = std::filesystem::temp_directory_path() / "rtte_mission_obstacles.toml";
  std::ofstream(path)
      << "[[Obstacle]]\nPoints = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0]]\n\n"
      << "[[Obstacle]]\nPoints = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0]]\nBlocksVision = true\n\n"
      << "[[Obstacle]]\nPoints = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0]]\nBlocksMovement = false\nBlocksVision = true\n";

  Mission mission(path.string());
  const std::vector<Obstacle> &obstacles = mission.GetObstacles();

  ASSERT_EQ(obstacles.size(), 3);
  EXPECT_TRUE(obstacles.at(0).BlocksMovement());
  EXPECT_FALSE(obstacles.at(0).BlocksVision());
  EXPECT_TRUE(obstacles.at(1).BlocksMovement());
  EXPECT_TRUE(obstacles.at(1).BlocksVision());
  EXPECT_FALSE(obstacles.at(2).BlocksMovement());
  EXPECT_TRUE(obstacles.at(2).BlocksVision());

  std::filesystem::remove(path);
}

TEST_F(MissionTest, ParsesSurfaces)
{
  std::filesystem::path path = std::filesystem::temp_directory_path() / "rtte_mission_surfaces.toml";
  std::ofstream(path)
      << "[[Surface]]\nPoints = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0]]\nFootprints = true\n\n"
      << "[[Surface]]\nPoints = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0]]\n\n"
      << "[[Surface]]\nPoints = [[0.0, 0.0], [10.0, 0.0]]\nFootprints = true\n";

  Mission mission(path.string());
  const std::vector<Surface> &surfaces = mission.GetSurfaces();

  ASSERT_EQ(surfaces.size(), 2);
  EXPECT_TRUE(surfaces.at(0).HasFootprints());
  EXPECT_FALSE(surfaces.at(1).HasFootprints());

  std::filesystem::remove(path);
}
