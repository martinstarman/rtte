#include <gtest/gtest.h>
#include <raylib.h>

#include "character.h"

TEST(CharacterTest, StartsAtPositionWithoutMoving)
{
  Character character(Vector3{10.0, 20.0, 0.0});
  Vector3 position = character.GetPosition();

  EXPECT_FLOAT_EQ(position.x, 10.0f);
  EXPECT_FLOAT_EQ(position.y, 20.0f);
  EXPECT_FALSE(character.IsMoving());
}

TEST(CharacterTest, MovesTowardsNextPoint)
{
  Character character(Vector3{0.0, 0.0, 0.0});

  character.SetPath({{0.0, 0.0}, {100.0, 0.0}});
  character.Update(50.0f / Character::kSpeed);
  Vector3 position = character.GetPosition();

  EXPECT_FLOAT_EQ(position.x, 50.0f);
  EXPECT_FLOAT_EQ(position.y, 0.0f);
  EXPECT_TRUE(character.IsMoving());
}

TEST(CharacterTest, CarriesOverDistanceAcrossPoints)
{
  Character character(Vector3{0.0, 0.0, 0.0});

  character.SetPath({{0.0, 0.0}, {50.0, 0.0}, {50.0, 100.0}});
  character.Update(100.0f / Character::kSpeed);
  Vector3 position = character.GetPosition();

  EXPECT_FLOAT_EQ(position.x, 50.0f);
  EXPECT_FLOAT_EQ(position.y, 50.0f);
  EXPECT_TRUE(character.IsMoving());
}

TEST(CharacterTest, StopsAtPathEnd)
{
  Character character(Vector3{0.0, 0.0, 0.0});

  character.SetPath({{0.0, 0.0}, {30.0, 40.0}});
  character.Update(10.0f);
  Vector3 position = character.GetPosition();

  EXPECT_FLOAT_EQ(position.x, 30.0f);
  EXPECT_FLOAT_EQ(position.y, 40.0f);
  EXPECT_FALSE(character.IsMoving());
}

TEST(CharacterTest, EmptyPathStopsMovement)
{
  Character character(Vector3{0.0, 0.0, 0.0});

  character.SetPath({{0.0, 0.0}, {100.0, 0.0}});
  character.Update(50.0f / Character::kSpeed);
  character.SetPath({});
  character.Update(50.0f / Character::kSpeed);
  Vector3 position = character.GetPosition();

  EXPECT_FLOAT_EQ(position.x, 50.0f);
  EXPECT_FALSE(character.IsMoving());
}

TEST(CharacterTest, ContainsPointWithinRadius)
{
  Character character(Vector3{100.0, 100.0, 0.0});

  EXPECT_TRUE(character.Contains(Vector2{100.0, 100.0}));
  EXPECT_TRUE(character.Contains(Vector2{105.0, 100.0}));
  EXPECT_FALSE(character.Contains(Vector2{120.0, 100.0}));
}

TEST(CharacterTest, SelectionToggles)
{
  Character character(Vector3{0.0, 0.0, 0.0});

  EXPECT_FALSE(character.IsSelected());

  character.SetSelected(true);
  EXPECT_TRUE(character.IsSelected());

  character.SetSelected(false);
  EXPECT_FALSE(character.IsSelected());
}

TEST(CharacterTest, KeepsZWhileMoving)
{
  Character character(Vector3{0.0, 0.0, 2.0});
  character.SetPath({{100.0, 0.0}});

  character.Update(0.5f);

  EXPECT_FLOAT_EQ(character.GetPosition().z, 2.0f);
}

TEST(CharacterTest, YSortIndexIsBelowCenter)
{
  Character character(Vector3{0.0, 50.0, 0.0});

  EXPECT_GT(character.GetYSortIndex(), 50.0f);
}

TEST(CharacterTest, LeavesFootprintEveryStep)
{
  Character character(Vector3{0.0, 0.0, 1.0});

  character.SetPath({{0.0, 0.0}, {100.0, 0.0}});
  character.Update(5.0f * Character::kStepLength / Character::kSpeed);
  std::vector<Footprint> footprints = character.GetFootprints();

  ASSERT_EQ(footprints.size(), 5);
  EXPECT_FLOAT_EQ(footprints.at(0).GetPosition().x, Character::kStepLength);
  EXPECT_FLOAT_EQ(footprints.at(4).GetPosition().x, 5.0f * Character::kStepLength);
  EXPECT_FLOAT_EQ(footprints.at(0).GetPosition().z, 1.0f);
}

TEST(CharacterTest, AlternatesFeet)
{
  Character character(Vector3{0.0, 0.0, 0.0});

  character.SetPath({{0.0, 0.0}, {100.0, 0.0}});
  character.Update(2.0f * Character::kStepLength / Character::kSpeed);
  std::vector<Footprint> footprints = character.GetFootprints();

  ASSERT_EQ(footprints.size(), 2);
  EXPECT_LT(footprints.at(0).GetPosition().y * footprints.at(1).GetPosition().y, 0.0f);
}

TEST(CharacterTest, GetFootprintsClears)
{
  Character character(Vector3{0.0, 0.0, 0.0});

  character.SetPath({{0.0, 0.0}, {100.0, 0.0}});
  character.Update(2.0f * Character::kStepLength / Character::kSpeed);
  character.GetFootprints();

  EXPECT_TRUE(character.GetFootprints().empty());
}

TEST(CharacterTest, NoFootprintsWhenStanding)
{
  Character character(Vector3{0.0, 0.0, 0.0});

  character.Update(1.0f);

  EXPECT_TRUE(character.GetFootprints().empty());
}
