#include <gtest/gtest.h>

#include "directional_sprite.h"

TEST(DirectionalSpriteTest, FacesSouthByDefault)
{
  DirectionalSprite sprite(10.0f);

  EXPECT_EQ(sprite.GetFacing(), 2);
  EXPECT_FALSE(sprite.IsLoaded());
}

TEST(DirectionalSpriteTest, PicksNearestDirection)
{
  DirectionalSprite sprite(10.0f);

  for (size_t i = 0; i < DirectionalSprite::kDirections; ++i)
  {
    sprite.SetAngle(i * 45.0f);
    EXPECT_EQ(sprite.GetFacing(), i);
  }

  sprite.SetAngle(20.0f);
  EXPECT_EQ(sprite.GetFacing(), 0);

  sprite.SetAngle(25.0f);
  EXPECT_EQ(sprite.GetFacing(), 1);
}

TEST(DirectionalSpriteTest, WrapsAngles)
{
  DirectionalSprite sprite(10.0f);

  sprite.SetAngle(-90.0f);
  EXPECT_EQ(sprite.GetFacing(), 6);

  sprite.SetAngle(-180.0f);
  EXPECT_EQ(sprite.GetFacing(), 4);

  sprite.SetAngle(350.0f);
  EXPECT_EQ(sprite.GetFacing(), 0);

  sprite.SetAngle(720.0f + 90.0f);
  EXPECT_EQ(sprite.GetFacing(), 2);
}
