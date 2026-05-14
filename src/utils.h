#pragma once

#define _USE_MATH_DEFINES

#include <cmath>
#include <raylib.h>
#include <string>
#include <unordered_map>

float const EPSILON = 0.001f;

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
bool Vector2Equals(const Vector2 &lhs, const Vector2 &rhs);
float Vector2Distance(const Vector2 &lhs, const Vector2 &rhs);
float CrossProduct(const Vector2 &a, const Vector2 &b, const Vector2 &c);
