#include <gtest/gtest.h>
#include <raylib.h>

#include "character.h"

TEST(CharacterTest, StartsAtPositionWithoutMoving)
{
  Character character(Vector2{10.0, 20.0}, 100.0f);
  Vector2 position = character.GetPosition();

  EXPECT_FLOAT_EQ(position.x, 10.0f);
  EXPECT_FLOAT_EQ(position.y, 20.0f);
  EXPECT_FALSE(character.IsMoving());
}

TEST(CharacterTest, MovesTowardsNextPoint)
{
  Character character(Vector2{0.0, 0.0}, 100.0f);

  character.SetPath({{0.0, 0.0}, {100.0, 0.0}});
  character.Update(0.5f);
  Vector2 position = character.GetPosition();

  EXPECT_FLOAT_EQ(position.x, 50.0f);
  EXPECT_FLOAT_EQ(position.y, 0.0f);
  EXPECT_TRUE(character.IsMoving());
}

TEST(CharacterTest, CarriesOverDistanceAcrossPoints)
{
  Character character(Vector2{0.0, 0.0}, 100.0f);

  character.SetPath({{0.0, 0.0}, {50.0, 0.0}, {50.0, 100.0}});
  character.Update(1.0f);
  Vector2 position = character.GetPosition();

  EXPECT_FLOAT_EQ(position.x, 50.0f);
  EXPECT_FLOAT_EQ(position.y, 50.0f);
  EXPECT_TRUE(character.IsMoving());
}

TEST(CharacterTest, StopsAtPathEnd)
{
  Character character(Vector2{0.0, 0.0}, 100.0f);

  character.SetPath({{0.0, 0.0}, {30.0, 40.0}});
  character.Update(10.0f);
  Vector2 position = character.GetPosition();

  EXPECT_FLOAT_EQ(position.x, 30.0f);
  EXPECT_FLOAT_EQ(position.y, 40.0f);
  EXPECT_FALSE(character.IsMoving());
}

TEST(CharacterTest, EmptyPathStopsMovement)
{
  Character character(Vector2{0.0, 0.0}, 100.0f);

  character.SetPath({{0.0, 0.0}, {100.0, 0.0}});
  character.Update(0.5f);
  character.SetPath({});
  character.Update(0.5f);
  Vector2 position = character.GetPosition();

  EXPECT_FLOAT_EQ(position.x, 50.0f);
  EXPECT_FALSE(character.IsMoving());
}

TEST(CharacterTest, ContainsPointWithinRadius)
{
  Character character(Vector2{100.0, 100.0}, 100.0f);

  EXPECT_TRUE(character.Contains(Vector2{100.0, 100.0}));
  EXPECT_TRUE(character.Contains(Vector2{105.0, 100.0}));
  EXPECT_FALSE(character.Contains(Vector2{120.0, 100.0}));
}

TEST(CharacterTest, SelectionToggles)
{
  Character character(Vector2{0.0, 0.0}, 100.0f);

  EXPECT_FALSE(character.IsSelected());

  character.SetSelected(true);
  EXPECT_TRUE(character.IsSelected());

  character.SetSelected(false);
  EXPECT_FALSE(character.IsSelected());
}
