#include "game.h"

Game::Game() : m_maxDrawingLayer(0)
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
  HandleEntityAbilities();

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

  for (int drawingLayer = 0; drawingLayer <= m_maxDrawingLayer; drawingLayer++)
  {
    for (auto &entity : m_entities)
    {
      if (entity->GetDrawingLayer() == drawingLayer)
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

  int drawingLayer = entity->GetDrawingLayer();

  if (drawingLayer > m_maxDrawingLayer)
  {
    m_maxDrawingLayer = drawingLayer;
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
      if (entity->GetDrawingLayer() > 0 && !entity->GetSelected())
      {
        Vector2 mousePosition = GetGameMousePosition();
        std::vector<Vector2> shape = entity->GetShape();

        if (CheckCollisionPointPoly(mousePosition, &shape[0], shape.size()))
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
      std::vector<Vector2> shape = leavesTraceEntities->GetShape();

      for (const auto movingEntities : m_entities)
      {
        if (movingEntities->IsMoving())
        {
          Vector2 position = movingEntities->GetPosition();

          if (CheckCollisionPointPoly(position, &shape[0], shape.size()))
          {
            movingEntities->SetTrace();
          }
        }
      }
    }
  }
}

void Game::HandleEntityAbilities()
{
  for (const auto &needsAbilityEntity : m_entities)
  {
    TraceLog(LOG_INFO, "1");
    std::string needsAbility = needsAbilityEntity->GetNeedsAbility();
    
    if (needsAbility != "")
    {
      TraceLog(LOG_INFO, "2");
      std::vector<Vector2> shape = needsAbilityEntity->GetShape();
      
      for (const auto &abilityEntity : m_entities)
      {
        Vector2 position = abilityEntity->GetPosition();
        std::string ability = abilityEntity->GetAbility();
        
        if (CheckCollisionPointPoly(position, &shape[0], shape.size()) && needsAbility != ability)
        {
          TraceLog(LOG_INFO, "4");
          abilityEntity->SetPath({});
        }
      }
    }
  }
}
