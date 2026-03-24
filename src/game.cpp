#include "game.h"

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
  HandleCameraOffset();
  bool entitySelected = HandleEntitySelection();
  if (!entitySelected)
  {
    HandleEntityMovement();
  }
  HandleEntityTraces();

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
            { return a->GetZIndex() < b->GetZIndex(); });

  for (int layerIndex = 0; layerIndex <= m_maxLayerIndex; layerIndex++)
  {
    for (auto &entity : m_entities)
    {
      if (entity->GetLayerIndex() == layerIndex)
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

  int layerIndex = entity->GetLayerIndex();

  if (layerIndex > m_maxLayerIndex)
  {
    m_maxLayerIndex = layerIndex;
  }
}

Vector2 Game::GetGameMousePosition()
{
  Vector2 mousePosition = GetMousePosition();

  return {mousePosition.x - m_camera.offset.x,
          mousePosition.y - m_camera.offset.y};
}

void Game::HandleCameraOffset()
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

bool Game::HandleEntitySelection()
{
  int selectedEntityId = -1;

  if (IsMouseButtonPressed(MOUSE_BUTTON_LEFT))
  {
    for (auto &entity : m_entities)
    {
      if (entity->GetLayerIndex() > 0 && !entity->GetSelected())
      {
        Vector2 mousePosition = GetGameMousePosition();
        std::vector<Vector2> polygon = entity->GetPolygon();

        if (CheckCollisionPointPoly(mousePosition, &polygon[0], polygon.size()))
        {
          entity->SetSelected(true);
          selectedEntityId = entity->GetId();
        }
      }
    }

    if (selectedEntityId != -1)
    {
      for (auto &entity : m_entities)
      {
        if (entity->GetId() != selectedEntityId)
        {
          entity->SetSelected(false);
        }
      }
    }
  }

  return selectedEntityId != -1;
}

void Game::HandleEntityMovement()
{
  if (IsMouseButtonPressed(MOUSE_BUTTON_LEFT))
  {
    for (auto &entity : m_entities)
    {
      if (entity->GetSelected())
      {
        Vector2 mousePosition = GetGameMousePosition();
        entity->SetPath({mousePosition});
      }
    }
  }
}

void Game::HandleEntityTraces()
{
  for (const auto leavesTraceEntities : m_entities)
  {
    if (leavesTraceEntities->GetShowsTraces())
    {
      std::vector<Vector2> polygon = leavesTraceEntities->GetPolygon();

      for (const auto movingEntities : m_entities)
      {
        if (movingEntities->IsMoving())
        {
          Vector2 position = movingEntities->GetPosition();

          if (CheckCollisionPointPoly(position, &polygon[0], polygon.size()))
          {
            movingEntities->SetTrace();
          }
        }
      }
    }
  }
}
