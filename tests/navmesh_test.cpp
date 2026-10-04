#include <cmath>
#include <gtest/gtest.h>
#include <raylib.h>
#include <raymath.h>
#include <vector>

#include "navmesh.h"

namespace
{
  const Rectangle kBounds = {0.0f, 0.0f, 100.0f, 100.0f};
  const std::vector<Vector2> kSquare = {{40.0f, 40.0f}, {60.0f, 40.0f}, {60.0f, 60.0f}, {40.0f, 60.0f}};

  float Length(const std::vector<Vector2> &path)
  {
    float length = 0.0f;
    for (size_t i = 1; i < path.size(); ++i)
    {
      length += Vector2Distance(path.at(i - 1), path.at(i));
    }
    return length;
  }
}

TEST(NavmeshTest, GoesStraightWithoutHoles)
{
  Navmesh navmesh(kBounds, {}, 0.0f);

  std::vector<Vector2> path = navmesh.FindPath({10.0f, 10.0f}, {90.0f, 90.0f});

  ASSERT_EQ(path.size(), 2);
  EXPECT_FLOAT_EQ(path.at(0).x, 10.0f);
  EXPECT_FLOAT_EQ(path.at(1).y, 90.0f);
}

TEST(NavmeshTest, GoesAroundHole)
{
  Navmesh navmesh(kBounds, {kSquare}, 0.0f);

  std::vector<Vector2> path = navmesh.FindPath({10.0f, 50.0f}, {90.0f, 50.0f});

  ASSERT_EQ(path.size(), 4);
  EXPECT_NEAR(Length(path), 2.0f * std::sqrt(30.0f * 30.0f + 10.0f * 10.0f) + 20.0f, 1e-3f);
}

TEST(NavmeshTest, KeepsAgentRadiusFromHole)
{
  Navmesh navmesh(kBounds, {kSquare}, 5.0f);

  std::vector<Vector2> path = navmesh.FindPath({10.0f, 50.0f}, {90.0f, 50.0f});

  ASSERT_EQ(path.size(), 4);
  EXPECT_NEAR(Length(path), 2.0f * std::sqrt(25.0f * 25.0f + 15.0f * 15.0f) + 30.0f, 1e-3f);
}

TEST(NavmeshTest, DoesNotCutThroughHoleDiagonal)
{
  Navmesh navmesh(kBounds, {kSquare}, 0.0f);

  std::vector<Vector2> path = navmesh.FindPath({40.0f, 40.0f}, {60.0f, 60.0f});

  ASSERT_EQ(path.size(), 3);
  EXPECT_NEAR(Length(path), 40.0f, 1e-3f);
}

TEST(NavmeshTest, GoesAlongHoleEdge)
{
  Navmesh navmesh(kBounds, {kSquare}, 0.0f);

  std::vector<Vector2> path = navmesh.FindPath({10.0f, 40.0f}, {90.0f, 40.0f});

  ASSERT_EQ(path.size(), 2);
}

TEST(NavmeshTest, GoesAroundTouchingHoles)
{
  std::vector<Vector2> bar = {{20.0f, 20.0f}, {40.0f, 20.0f}, {40.0f, 80.0f}, {20.0f, 80.0f}};
  std::vector<Vector2> foot = {{40.0f, 60.0f}, {80.0f, 60.0f}, {80.0f, 80.0f}, {40.0f, 80.0f}};
  Navmesh navmesh(kBounds, {bar, foot}, 1.0f);

  std::vector<Vector2> path = navmesh.FindPath({60.0f, 40.0f}, {60.0f, 90.0f});

  ASSERT_EQ(path.size(), 4);
  EXPECT_NEAR(Length(path), std::sqrt(21.0f * 21.0f + 19.0f * 19.0f) + 22.0f + std::sqrt(21.0f * 21.0f + 9.0f * 9.0f), 1e-3f);
}

TEST(NavmeshTest, PassesBetweenHoles)
{
  std::vector<Vector2> top = {{40.0f, 0.0f}, {60.0f, 0.0f}, {60.0f, 45.0f}, {40.0f, 45.0f}};
  std::vector<Vector2> bottom = {{40.0f, 55.0f}, {60.0f, 55.0f}, {60.0f, 100.0f}, {40.0f, 100.0f}};
  Navmesh navmesh(kBounds, {top, bottom}, 2.0f);

  std::vector<Vector2> path = navmesh.FindPath({10.0f, 50.0f}, {90.0f, 50.0f});

  EXPECT_NEAR(Length(path), 80.0f, 1e-3f);
}

TEST(NavmeshTest, ReturnsEmptyPathWhenGapIsTooNarrow)
{
  std::vector<Vector2> top = {{40.0f, 0.0f}, {60.0f, 0.0f}, {60.0f, 45.0f}, {40.0f, 45.0f}};
  std::vector<Vector2> bottom = {{40.0f, 55.0f}, {60.0f, 55.0f}, {60.0f, 100.0f}, {40.0f, 100.0f}};
  Navmesh navmesh(kBounds, {top, bottom}, 6.0f);

  EXPECT_TRUE(navmesh.FindPath({10.0f, 50.0f}, {90.0f, 50.0f}).empty());
}

TEST(NavmeshTest, ReturnsEmptyPathWhenHoleSplitsMap)
{
  std::vector<Vector2> wall = {{40.0f, -10.0f}, {60.0f, -10.0f}, {60.0f, 110.0f}, {40.0f, 110.0f}};
  Navmesh navmesh(kBounds, {wall}, 0.0f);

  EXPECT_TRUE(navmesh.FindPath({10.0f, 50.0f}, {90.0f, 50.0f}).empty());
}

TEST(NavmeshTest, ReturnsEmptyPathWhenStartIsInsideHole)
{
  Navmesh navmesh(kBounds, {kSquare}, 0.0f);

  EXPECT_TRUE(navmesh.FindPath({50.0f, 50.0f}, {90.0f, 50.0f}).empty());
}

TEST(NavmeshTest, ReturnsEmptyPathWhenTargetIsOutsideBounds)
{
  Navmesh navmesh(kBounds, {kSquare}, 0.0f);

  EXPECT_TRUE(navmesh.FindPath({10.0f, 50.0f}, {150.0f, 50.0f}).empty());
}
