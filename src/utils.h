#pragma once

#define _USE_MATH_DEFINES

#include <cmath>
#include <raylib.h>
#include <string>
#include <unordered_map>

enum Octant
{
  East,
  NorthEast,
  North,
  NorthWest,
  West,
  SouthWest,
  South,
  SouthEast,
};

const std::unordered_map<std::string, Octant> OCTANT_MAP = {
    {"East", Octant::East},
    {"NorthEast", Octant::NorthEast},
    {"North", Octant::North},
    {"NorthWest", Octant::NorthWest},
    {"West", Octant::West},
    {"SouthWest", Octant::SouthWest},
    {"South", Octant::South},
    {"SouthEast", Octant::SouthEast},
};

float GetAngleBetween(Vector2 a, Vector2 b);
Octant GetOctantFrom(float angle);
Octant GetOctantFrom(const std::string &string);
