cmake_minimum_required(VERSION 4.3)

set (CMAKE_CXX_STANDARD 17)

project(rtte)

add_executable(rtte)

target_sources(rtte
  PRIVATE
    src/entity.cpp
    src/game.cpp
    src/main.cpp
    src/navmesh.cpp
    src/triangle.cpp
    src/utils.cpp
)

find_package(raylib CONFIG REQUIRED)
target_link_libraries(rtte PRIVATE raylib)

find_package(toml11 CONFIG REQUIRED)
target_link_libraries(rtte PRIVATE toml11::toml11)

find_path(EARCUT_HPP_INCLUDE_DIRS "mapbox/earcut.hpp")
target_include_directories(rtte PRIVATE ${EARCUT_HPP_INCLUDE_DIRS})

find_path(CXXGRAPH_INCLUDE_DIR CXXGraph/CXXGraph.hpp)
target_include_directories(rtte PRIVATE ${CXXGRAPH_INCLUDE_DIR})
