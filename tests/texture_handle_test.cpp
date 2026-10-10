#include <gtest/gtest.h>
#include <raylib.h>
#include <utility>

#include "texture_handle.h"

class TextureHandleTest : public ::testing::Test
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

TEST_F(TextureHandleTest, DefaultIsEmpty)
{
  TextureHandle handle;

  EXPECT_EQ(handle.Get().id, 0);
}

TEST_F(TextureHandleTest, HoldsTexture)
{
  TextureHandle handle(LoadTexture("sample/sprite1.png"));

  EXPECT_TRUE(IsTextureValid(handle.Get()));
}

TEST_F(TextureHandleTest, MoveConstructorTransfersOwnership)
{
  TextureHandle source(LoadTexture("sample/sprite1.png"));
  unsigned int id = source.Get().id;

  TextureHandle target(std::move(source));

  EXPECT_EQ(target.Get().id, id);
  EXPECT_EQ(source.Get().id, 0);
}

TEST_F(TextureHandleTest, MoveAssignmentTransfersOwnership)
{
  TextureHandle source(LoadTexture("sample/sprite1.png"));
  unsigned int id = source.Get().id;
  TextureHandle target;

  target = std::move(source);

  EXPECT_EQ(target.Get().id, id);
  EXPECT_EQ(source.Get().id, 0);
}
