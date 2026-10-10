#include <cmath>
#include <gtest/gtest.h>
#include <raylib.h>
#include <raymath.h>

#include "enemy.h"

TEST(EnemyTest, StandsStillWithEmptyPath)
{
  Enemy enemy(Vector3{10.0, 20.0, 0.0}, {}, 90.0f);

  enemy.Update(1.0f);

  EXPECT_FLOAT_EQ(enemy.GetPosition().x, 10.0f);
  EXPECT_FLOAT_EQ(enemy.GetPosition().y, 20.0f);
  EXPECT_FLOAT_EQ(enemy.GetDirection(), 90.0f);
}

TEST(EnemyTest, FacesMovementDirection)
{
  Enemy enemy(Vector3{0.0, 0.0, 0.0}, {{100.0, 0.0}, {100.0, 100.0}}, 90.0f);

  enemy.Update(50.0f / Enemy::kSpeed);

  EXPECT_FLOAT_EQ(enemy.GetPosition().x, 50.0f);
  EXPECT_FLOAT_EQ(enemy.GetDirection(), 0.0f);

  enemy.Update(100.0f / Enemy::kSpeed);

  EXPECT_FLOAT_EQ(enemy.GetPosition().x, 100.0f);
  EXPECT_FLOAT_EQ(enemy.GetPosition().y, 50.0f);
  EXPECT_FLOAT_EQ(enemy.GetDirection(), 90.0f);
}

TEST(EnemyTest, LoopsPath)
{
  Enemy enemy(Vector3{0.0, 0.0, 0.0}, {{30.0, 0.0}, {0.0, 0.0}}, 0.0f);

  enemy.Update(70.0f / Enemy::kSpeed);

  EXPECT_FLOAT_EQ(enemy.GetPosition().x, 10.0f);
  EXPECT_FLOAT_EQ(enemy.GetDirection(), 0.0f);
}

TEST(EnemyTest, StopsAtSingleWaypoint)
{
  Enemy enemy(Vector3{0.0, 0.0, 0.0}, {{30.0, 0.0}}, 90.0f);

  enemy.Update(10.0f);

  EXPECT_FLOAT_EQ(enemy.GetPosition().x, 30.0f);
  EXPECT_FLOAT_EQ(enemy.GetDirection(), 0.0f);
}

TEST(EnemyTest, LookSweepsAroundDirection)
{
  Enemy enemy(Vector3{0.0, 0.0, 0.0}, {}, 90.0f);

  EXPECT_FLOAT_EQ(enemy.GetLookAngle(), 90.0f);

  enemy.Update(PI / 2.0f / Enemy::kSweepSpeed);

  EXPECT_NEAR(enemy.GetLookAngle(), 90.0f + Enemy::kSweepAngle, 1e-3f);
}

TEST(EnemyTest, ViewReachesFullDistanceWithoutObstacles)
{
  Enemy enemy(Vector3{0.0, 0.0, 0.0}, {}, 0.0f);

  std::vector<Vector2> polygon = enemy.GetViewPolygon({});

  ASSERT_GT(polygon.size(), 2);
  for (size_t i = 1; i < polygon.size(); ++i)
  {
    EXPECT_NEAR(Vector2Length(polygon.at(i)), Enemy::kViewDistance, 1e-2f);
  }
}

TEST(EnemyTest, ViewIsBlockedByVisionObstacle)
{
  Enemy enemy(Vector3{0.0, 0.0, 0.0}, {}, 0.0f);
  std::vector<Obstacle> obstacles = {
      Obstacle({{50.0, -500.0}, {60.0, -500.0}, {60.0, 500.0}, {50.0, 500.0}}, true, true)};

  std::vector<Vector2> polygon = enemy.GetViewPolygon(obstacles);

  for (const Vector2 &point : polygon)
  {
    EXPECT_LE(point.x, 50.01f);
  }
}

TEST(EnemyTest, ViewIgnoresMovementOnlyObstacle)
{
  Enemy enemy(Vector3{0.0, 0.0, 0.0}, {}, 0.0f);
  std::vector<Obstacle> obstacles = {
      Obstacle({{50.0, -500.0}, {60.0, -500.0}, {60.0, 500.0}, {50.0, 500.0}}, true, false)};

  std::vector<Vector2> polygon = enemy.GetViewPolygon(obstacles);

  for (size_t i = 1; i < polygon.size(); ++i)
  {
    EXPECT_NEAR(Vector2Length(polygon.at(i)), Enemy::kViewDistance, 1e-2f);
  }
}

TEST(EnemyTest, FacingFollowsInitialDirection)
{
  Enemy enemy(Vector3{0.0, 0.0, 0.0}, {}, 180.0f);

  EXPECT_EQ(enemy.GetFacing(), 4);
  EXPECT_FALSE(enemy.IsMoving());
}

TEST(EnemyTest, FacingFollowsMovement)
{
  Enemy enemy(Vector3{0.0, 0.0, 0.0}, {{0.0, -100.0}}, 0.0f);

  enemy.Update(0.1f);

  EXPECT_EQ(enemy.GetFacing(), 6);
  EXPECT_TRUE(enemy.IsMoving());
}

TEST(EnemyTest, StopsMovingAtSingleWaypoint)
{
  Enemy enemy(Vector3{0.0, 0.0, 0.0}, {{30.0, 0.0}}, 90.0f);

  enemy.Update(10.0f);
  enemy.Update(0.1f);

  EXPECT_FALSE(enemy.IsMoving());
  EXPECT_EQ(enemy.GetFacing(), 0);
}
