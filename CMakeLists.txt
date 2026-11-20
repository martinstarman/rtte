cmake_minimum_required(VERSION 4.3)

project(rtte CXX)

set(CMAKE_CXX_STANDARD 17)

find_package(raylib CONFIG REQUIRED)
find_package(toml11 CONFIG REQUIRED)

add_executable(rtte src/main.cpp src/animation.cpp src/renderable.cpp src/sprite.cpp src/texture_handle.cpp src/directional_sprite.cpp src/navmesh.cpp src/sector.cpp src/footprint.cpp src/character.cpp src/enemy.cpp src/mission.cpp)
target_include_directories(rtte PRIVATE src)
target_link_libraries(rtte PRIVATE raylib toml11::toml11)
