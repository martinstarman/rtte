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

  Sprite sprite(path, Vector2{32.0, 32.0}, Vector2{0.0, 0.0});
  const Texture2D &texture = sprite.GetTexture();

  ASSERT_TRUE(IsTextureValid(texture));
  EXPECT_EQ(texture.width, image.width);
  EXPECT_EQ(texture.height, image.height);

  UnloadImage(image);
}
