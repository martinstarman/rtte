cmake_minimum_required(VERSION 4.3)

project(rtte CXX)

set(CMAKE_CXX_STANDARD 17)

find_package(raylib CONFIG REQUIRED)
find_package(toml11 CONFIG REQUIRED)

add_executable(rtte
  src/animation.cpp
  src/drawable.cpp
  src/main.cpp
  src/mission.cpp
  src/sprite.cpp
)

target_include_directories(rtte PRIVATE src)
target_link_libraries(rtte PRIVATE raylib toml11::toml11)
