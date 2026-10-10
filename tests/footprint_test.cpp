#include <gtest/gtest.h>
#include <raylib.h>

#include "footprint.h"

TEST(FootprintTest, StartsFullyOpaque)
{
  Footprint footprint(Vector3{0.0, 0.0, 0.0}, 0.0f);

  EXPECT_FLOAT_EQ(footprint.GetOpacity(), 1.0f);
  EXPECT_FALSE(footprint.IsExpired());
}

TEST(FootprintTest, FadesOverTime)
{
  Footprint footprint(Vector3{0.0, 0.0, 0.0}, 0.0f);

  footprint.Update(Footprint::kLifetime / 2.0f);

  EXPECT_FLOAT_EQ(footprint.GetOpacity(), 0.5f);
  EXPECT_FALSE(footprint.IsExpired());
}

TEST(FootprintTest, ExpiresAfterLifetime)
{
  Footprint footprint(Vector3{0.0, 0.0, 0.0}, 0.0f);

  footprint.Update(Footprint::kLifetime);

  EXPECT_FLOAT_EQ(footprint.GetOpacity(), 0.0f);
  EXPECT_TRUE(footprint.IsExpired());
}

TEST(FootprintTest, RendersBelowEverythingInLayer)
{
  Footprint footprint(Vector3{0.0, 100.0, 0.0}, 0.0f);

  EXPECT_LT(footprint.GetYSortIndex(), -1e30f);
}
