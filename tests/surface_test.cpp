#include <gtest/gtest.h>
#include <raylib.h>

#include "surface.h"

TEST(SurfaceTest, ContainsPointInside)
{
  Surface surface({{0.0, 0.0}, {100.0, 0.0}, {100.0, 100.0}, {0.0, 100.0}}, true);

  EXPECT_TRUE(surface.Contains(Vector2{50.0, 50.0}));
  EXPECT_FALSE(surface.Contains(Vector2{150.0, 50.0}));
}

TEST(SurfaceTest, KeepsFootprintsFlag)
{
  Surface snow({{0.0, 0.0}, {10.0, 0.0}, {10.0, 10.0}}, true);
  Surface grass({{0.0, 0.0}, {10.0, 0.0}, {10.0, 10.0}}, false);

  EXPECT_TRUE(snow.HasFootprints());
  EXPECT_FALSE(grass.HasFootprints());
}
