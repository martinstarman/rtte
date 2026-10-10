#include <gtest/gtest.h>
#include <raylib.h>

#include "animation.h"

TEST(AnimationTest, StartsAtFirstFrame)
{
  Animation animation(3, Vector2{32.0, 32.0}, 2.0f);

  EXPECT_EQ(animation.GetCurrentFrame(), 0);
}

TEST(AnimationTest, StaysOnFrameUntilFrameDurationElapses)
{
  Animation animation(3, Vector2{32.0, 32.0}, 2.0f);

  animation.Update(0.25f);
  EXPECT_EQ(animation.GetCurrentFrame(), 0);

  animation.Update(0.25f);
  EXPECT_EQ(animation.GetCurrentFrame(), 1);
}

TEST(AnimationTest, SkipsFramesOnLongUpdate)
{
  Animation animation(3, Vector2{32.0, 32.0}, 2.0f);

  animation.Update(1.0f);

  EXPECT_EQ(animation.GetCurrentFrame(), 2);
}

TEST(AnimationTest, WrapsToFirstFrame)
{
  Animation animation(3, Vector2{32.0, 32.0}, 2.0f);

  animation.Update(1.5f);

  EXPECT_EQ(animation.GetCurrentFrame(), 0);
}

TEST(AnimationTest, KeepsRemainingTime)
{
  Animation animation(3, Vector2{32.0, 32.0}, 2.0f);

  animation.Update(0.75f);
  EXPECT_EQ(animation.GetCurrentFrame(), 1);

  animation.Update(0.25f);
  EXPECT_EQ(animation.GetCurrentFrame(), 2);
}

TEST(AnimationTest, DoesNotAdvanceWithZeroFps)
{
  Animation animation(3, Vector2{32.0, 32.0}, 0.0f);

  animation.Update(10.0f);

  EXPECT_EQ(animation.GetCurrentFrame(), 0);
}
