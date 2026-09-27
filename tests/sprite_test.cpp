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

TEST_F(SpriteTest, TextureHasImageSize)
{
  const char *path = "sample/sprite1.png";

  Image image = LoadImage(path);
  ASSERT_TRUE(IsImageValid(image));

  Sprite sprite(path, Vector3{0.0, 0.0, 0.0});
  const Texture2D &texture = sprite.GetTexture();

  ASSERT_TRUE(IsTextureValid(texture));
  EXPECT_EQ(texture.width, image.width);
  EXPECT_EQ(texture.height, image.height);

  UnloadImage(image);
}

TEST_F(SpriteTest, AnimatedTextureHasSpriteSheetSize)
{
  const char *path = "sample/sprite2.png";

  Image image = LoadImage(path);
  ASSERT_TRUE(IsImageValid(image));

  Sprite sprite(path, Vector3{0.0, 0.0, 0.0}, Animation(2, Vector2{32.0, 32.0}));
  const Texture2D &texture = sprite.GetTexture();

  ASSERT_TRUE(IsTextureValid(texture));
  EXPECT_EQ(texture.width, image.width);
  EXPECT_EQ(texture.height, image.height);

  UnloadImage(image);
}

TEST_F(SpriteTest, TextureHasGivenSize)
{
  const char *path = "sample/sprite1.png";

  Sprite sprite(path, Vector3{0.0, 0.0, 0.0}, Vector2{64.0, 48.0});
  const Texture2D &texture = sprite.GetTexture();

  ASSERT_TRUE(IsTextureValid(texture));
  EXPECT_EQ(texture.width, 64);
  EXPECT_EQ(texture.height, 48);
}

TEST_F(SpriteTest, AnimatedTextureHasGivenSizePerFrame)
{
  const char *path = "sample/sprite2.png";

  Image image = LoadImage(path);
  ASSERT_TRUE(IsImageValid(image));

  Sprite sprite(path, Vector3{0.0, 0.0, 0.0}, Vector2{64.0, 64.0}, Animation(2, Vector2{32.0, 32.0}));
  const Texture2D &texture = sprite.GetTexture();

  ASSERT_TRUE(IsTextureValid(texture));
  EXPECT_EQ(texture.width, 128);
  EXPECT_EQ(texture.height, 64);

  Image result = LoadImageFromTexture(texture);
  ASSERT_TRUE(IsImageValid(result));

  EXPECT_EQ(ColorToInt(GetImageColor(result, 0, 0)), ColorToInt(GetImageColor(image, 0, 0)));
  EXPECT_EQ(ColorToInt(GetImageColor(result, 63, 63)), ColorToInt(GetImageColor(image, 31, 31)));
  EXPECT_EQ(ColorToInt(GetImageColor(result, 64, 0)), ColorToInt(GetImageColor(image, 32, 0)));
  EXPECT_EQ(ColorToInt(GetImageColor(result, 127, 63)), ColorToInt(GetImageColor(image, 63, 31)));

  UnloadImage(result);
  UnloadImage(image);
}

TEST_F(SpriteTest, TextureHasShapeBoundsAndIsFilledInsideShape)
{
  const char *path = "sample/sprite1.png";

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

TEST_F(SpriteTest, AnimatedTextureHasShapeBoundsPerFrameAndIsFilledInsideShape)
{
  const char *path = "sample/sprite2.png";

  Image image = LoadImage(path);
  ASSERT_TRUE(IsImageValid(image));

  std::vector<Vector2> shape = {
      Vector2{0.0, 0.0},
      Vector2{64.0, 0.0},
      Vector2{0.0, 48.0},
  };

  Sprite sprite(path, Vector3{0.0, 0.0, 0.0}, shape, Animation(2, Vector2{32.0, 32.0}));
  const Texture2D &texture = sprite.GetTexture();

  ASSERT_TRUE(IsTextureValid(texture));
  EXPECT_EQ(texture.width, 128);
  EXPECT_EQ(texture.height, 48);

  Image result = LoadImageFromTexture(texture);
  ASSERT_TRUE(IsImageValid(result));

  EXPECT_EQ(ColorToInt(GetImageColor(result, 0, 0)), ColorToInt(GetImageColor(image, 0, 0)));
  EXPECT_EQ(ColorToInt(GetImageColor(result, 63, 47)), ColorToInt(BLANK));
  EXPECT_EQ(ColorToInt(GetImageColor(result, 64, 0)), ColorToInt(GetImageColor(image, 32, 0)));
  EXPECT_EQ(ColorToInt(GetImageColor(result, 127, 47)), ColorToInt(BLANK));

  UnloadImage(result);
  UnloadImage(image);
}
