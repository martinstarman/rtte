#include <gtest/gtest.h>
#include <raylib.h>

#include "sprite.h"

class SpriteTest : public ::testing::Test
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

TEST_F(SpriteTest, TextureHasImageDimensions)
{
  const char *path = "sample/sprite.png";

  Image image = LoadImage(path);
  ASSERT_TRUE(IsImageValid(image));

  Sprite sprite(path, Vector3{0.0, 0.0, 0.0});
  const Texture2D &texture = sprite.GetTexture();

  ASSERT_TRUE(IsTextureValid(texture));
  EXPECT_EQ(texture.width, image.width);
  EXPECT_EQ(texture.height, image.height);

  UnloadImage(image);
}

TEST_F(SpriteTest, TextureHasGivenDimensions)
{
  const char *path = "sample/sprite.png";

  Sprite sprite(path, Vector3{0.0, 0.0, 0.0}, Vector2{64.0, 48.0});
  const Texture2D &texture = sprite.GetTexture();

  ASSERT_TRUE(IsTextureValid(texture));
  EXPECT_EQ(texture.width, 64);
  EXPECT_EQ(texture.height, 48);
}

TEST_F(SpriteTest, TextureHasShapeBoundsAndIsFilledInsideShape)
{
  const char *path = "sample/sprite.png";

  Image image = LoadImage(path);
  ASSERT_TRUE(IsImageValid(image));

  std::vector<Vector2> shape = {
      Vector2{0.0, 0.0},
      Vector2{64.0, 0.0},
      Vector2{0.0, 48.0},
  };

  Sprite sprite(path, Vector3{0.0, 0.0, 0.0}, shape);
  const Texture2D &texture = sprite.GetTexture();

  ASSERT_TRUE(IsTextureValid(texture));
  EXPECT_EQ(texture.width, 64);
  EXPECT_EQ(texture.height, 48);

  Image result = LoadImageFromTexture(texture);
  ASSERT_TRUE(IsImageValid(result));

  Color inside = GetImageColor(result, 0, 0);
  Color expected = GetImageColor(image, 0, 0);
  EXPECT_EQ(ColorToInt(inside), ColorToInt(expected));

  Color outside = GetImageColor(result, result.width - 1, result.height - 1);
  EXPECT_EQ(ColorToInt(outside), ColorToInt(BLANK));

  UnloadImage(result);
  UnloadImage(image);
}
