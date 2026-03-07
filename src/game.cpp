#include "game.h"

const float CAMERA_MOVEMENT_SPEED = 5.0;

Game::Game() : m_maxLayerIndex(0)
{
  m_camera = {0};
  m_camera.target = {0, 0};
  m_camera.offset = {0, 0};
  m_camera.rotation = 0;
  m_camera.zoom = 1;
}

Game::~Game()
{
  for (auto const &entity : m_entities)
  {
    delete entity;
  }
}

void Game::Update()
{
  ProcessCameraMovement();
  ProcessEntitySelection();

  for (auto &entity : m_entities)
  {
    entity->Update();
  }
}

void Game::Draw()
{
  BeginDrawing();
  ClearBackground(MAGENTA);
  BeginMode2D(m_camera);

  std::sort(m_entities.begin(), m_entities.end(),
            [](Entity *a, Entity *b)
            { return a->ZIndex() < b->ZIndex(); });

  for (int layerIndex = 0; layerIndex <= m_maxLayerIndex; layerIndex++)
  {
    for (auto &entity : m_entities)
    {
      if (entity->LayerIndex() == layerIndex)
      {
        entity->Draw();
      }
    }
  }

  EndMode2D();
  EndDrawing();
}

void Game::AddEntity(Entity *entity)
{
  m_entities.emplace_back(entity);
  int layerIndex = entity->LayerIndex();

  if (layerIndex > m_maxLayerIndex)
  {
    m_maxLayerIndex = layerIndex;
  }
}

void Game::ProcessCameraMovement()
{
  if (IsKeyDown(KEY_RIGHT))
  {
    m_camera.offset.x -= CAMERA_MOVEMENT_SPEED;
  }
  if (IsKeyDown(KEY_LEFT))
  {
    m_camera.offset.x += CAMERA_MOVEMENT_SPEED;
  }
  if (IsKeyDown(KEY_UP))
  {
    m_camera.offset.y += CAMERA_MOVEMENT_SPEED;
  }
  if (IsKeyDown(KEY_DOWN))
  {
    m_camera.offset.y -= CAMERA_MOVEMENT_SPEED;
  }
}

void Game::ProcessEntitySelection()
{
  if (IsMouseButtonPressed(MOUSE_BUTTON_LEFT))
  {
    for (auto &entity : m_entities)
    {
      if (entity->Selectable())
      {
        Vector2 mousePos = GetMousePosition();
        Vector2 point = {
            mousePos.x - m_camera.offset.x,
            mousePos.y - m_camera.offset.y,
        };
        std::vector<Vector2> points = entity->Polygon();

        if (CheckCollisionPointPoly(point, &points[0], points.size()))
        {
          entity->Selected(true);
        }
      }
    }
  }
}
