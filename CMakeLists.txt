cmake_minimum_required(VERSION 4.3)

set (CMAKE_CXX_STANDARD 17)

project(rtte)

add_executable(rtte)

target_sources(rtte
  PRIVATE
    src/entity.cpp
    src/game.cpp
    src/main.cpp
    src/utils.cpp
)

find_package(raylib CONFIG REQUIRED)
target_link_libraries(rtte PRIVATE raylib)

find_package(toml11 CONFIG REQUIRED)
target_link_libraries(rtte PRIVATE toml11::toml11)
