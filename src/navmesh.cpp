#include "navmesh.h"

Navmesh::Navmesh(Rectangle bounds, const std::vector<std::vector<Vector2>> &holes, float agentRadius)
    : m_bounds({bounds.x + agentRadius,
                bounds.y + agentRadius,
                bounds.width - 2.0f * agentRadius,
                bounds.height - 2.0f * agentRadius}),
      m_holes(holes)
{
  for (const std::vector<Vector2> &hole : m_holes)
  {
    std::vector<Vector2> expandedHole;
    size_t count = hole.size();

    for (size_t i = 0; i < count; ++i)
    {
      const Vector2 &prev = hole.at((i + count - 1) % count);
      const Vector2 &curr = hole.at(i);
      const Vector2 &next = hole.at((i + 1) % count);
      Vector2 n1 = OutwardNormal(prev, curr);
      Vector2 n2 = OutwardNormal(curr, next);
      float scale = agentRadius / (1.0f + Vector2DotProduct(n1, n2));

      expandedHole.push_back(Vector2Add(curr, Vector2Scale(Vector2Add(n1, n2), scale)));
    }

    m_expandedHoles.push_back(expandedHole);
  }

  for (const std::vector<Vector2> &hole : m_expandedHoles)
  {
    for (const Vector2 &v : hole)
    {
      if (IsWalkable(v))
      {
        m_nodes.push_back(v);
      }
    }
  }

  m_edges.resize(m_nodes.size());

  for (size_t i = 0; i < m_nodes.size(); ++i)
  {
    for (size_t j = i + 1; j < m_nodes.size(); ++j)
    {
      if (IsVisible(m_nodes.at(i), m_nodes.at(j)))
      {
        m_edges.at(i).push_back(j);
        m_edges.at(j).push_back(i);
      }
    }
  }
}

std::vector<Vector2> Navmesh::FindPath(Vector2 start, Vector2 target) const
{
  if (!IsWalkable(start) || !IsWalkable(target))
  {
    return {};
  }

  std::vector<Vector2> nodes = m_nodes;
  nodes.push_back(start);
  nodes.push_back(target);
  const int startIndex = nodes.size() - 2;
  const int targetIndex = nodes.size() - 1;

  std::vector<std::vector<int>> edges = m_edges;
  edges.resize(nodes.size());

  if (IsVisible(start, target))
  {
    edges.at(startIndex).push_back(targetIndex);
  }

  for (int i = 0; i < startIndex; ++i)
  {
    if (IsVisible(start, nodes.at(i)))
    {
      edges.at(startIndex).push_back(i);
    }
    if (IsVisible(nodes.at(i), target))
    {
      edges.at(i).push_back(targetIndex);
    }
  }

  std::vector<float> nodeCosts(nodes.size(), std::numeric_limits<float>::infinity());
  std::vector<int> prevNodes(nodes.size(), -1);
  std::vector<bool> closedNodes(nodes.size(), false);

  nodeCosts.at(startIndex) = 0.0f;

  while (true)
  {
    int current = -1;
    float bestEstimate = std::numeric_limits<float>::infinity();

    for (size_t i = 0; i < nodes.size(); ++i)
    {
      float estimate = nodeCosts.at(i) + Vector2Distance(nodes.at(i), target);

      if (!closedNodes.at(i) && estimate < bestEstimate)
      {
        current = i;
        bestEstimate = estimate;
      }
    }

    if (current == -1 || current == targetIndex)
    {
      break;
    }

    closedNodes.at(current) = true;

    for (size_t i = 0; i < edges.at(current).size(); ++i)
    {
      int neighbour = edges.at(current).at(i);
      float candidate = nodeCosts.at(current) + Vector2Distance(nodes.at(current), nodes.at(neighbour));

      if (candidate < nodeCosts.at(neighbour))
      {
        nodeCosts.at(neighbour) = candidate;
        prevNodes.at(neighbour) = current;
      }
    }
  }

  if (prevNodes.at(targetIndex) == -1)
  {
    return {};
  }

  std::vector<Vector2> path;

  for (int index = targetIndex; index != -1; index = prevNodes.at(index))
  {
    path.insert(path.begin(), nodes.at(index));
  }

  return path;
}

void Navmesh::Render() const
{
  for (size_t i = 0; i < m_edges.size(); ++i)
  {
    for (int to : m_edges.at(i))
    {
      DrawLineV(m_nodes.at(i), m_nodes.at(to), Fade(LIGHTGRAY, 0.2f));
    }
  }

  for (size_t i = 0; i < m_holes.size(); ++i)
  {
    const std::vector<Vector2> &expanded = m_expandedHoles.at(i);
    DrawLineStrip(expanded.data(), (int)expanded.size(), Fade(ORANGE, 0.5f));
    DrawLineV(expanded.back(), expanded.front(), Fade(ORANGE, 0.5f));

    const std::vector<Vector2> &hole = m_holes.at(i);
    DrawLineStrip(hole.data(), (int)hole.size(), YELLOW);
    DrawLineV(hole.back(), hole.front(), YELLOW);
  }

  DrawRectangleLinesEx(m_bounds, 1.0f, YELLOW);
}

bool Navmesh::IsWalkable(Vector2 v) const
{
  if (!CheckCollisionPointRec(v, m_bounds))
  {
    return false;
  }

  for (const std::vector<Vector2> &hole : m_expandedHoles)
  {
    bool isInside = true;

    for (size_t i = 0; i < hole.size() && isInside; ++i)
    {
      const Vector2 &a = hole.at(i);
      const Vector2 &b = hole.at((i + 1) % hole.size());

      isInside = Vector2DotProduct(OutwardNormal(a, b), Vector2Subtract(v, a)) < -kEpsilon;
    }

    if (isInside)
    {
      return false;
    }
  }

  return true;
}

bool Navmesh::IsVisible(Vector2 a, Vector2 b) const
{
  for (const std::vector<Vector2> &hole : m_expandedHoles)
  {
    bool isOutside = false;

    for (size_t i = 0; i < hole.size(); ++i)
    {
      const Vector2 &p = hole.at(i);
      const Vector2 &q = hole.at((i + 1) % hole.size());
      Vector2 n = OutwardNormal(p, q);

      if (Vector2DotProduct(n, Vector2Subtract(a, p)) >= -kEpsilon &&
          Vector2DotProduct(n, Vector2Subtract(b, p)) >= -kEpsilon)
      {
        isOutside = true;
      }
    }

    if (isOutside)
    {
      continue;
    }

    Vector2 n = OutwardNormal(a, b);
    bool isOnLeftSide = false;
    bool isOnRightSide = false;

    for (size_t i = 0; i < hole.size(); ++i)
    {
      float side = Vector2DotProduct(n, Vector2Subtract(hole.at(i), a));

      if (side > kEpsilon)
      {
        isOnLeftSide = true;
      }

      if (side < -kEpsilon)
      {
        isOnRightSide = true;
      }
    }

    if (isOnLeftSide && isOnRightSide)
    {
      return false;
    }
  }

  return true;
}

Vector2 Navmesh::OutwardNormal(Vector2 a, Vector2 b)
{
  Vector2 d = Vector2Subtract(b, a);
  return Vector2Normalize({d.y, -d.x});
}
