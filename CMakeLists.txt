project(rtte CXX)

cmake_minimum_required(VERSION 3.17)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(SDL2 CONFIG REQUIRED)
find_package(docopt CONFIG REQUIRED)
find_package(yaml-cpp CONFIG REQUIRED)
find_package(Fontconfig REQUIRED)
find_package(sdl2-ttf CONFIG REQUIRED)

include_directories(
    vendor/navmesh/source
)

add_executable(
    ${PROJECT_NAME}
    # 
    src/util.cpp
    src/entity.cpp
    src/character.cpp
    src/enemy.cpp
    src/serializer.cpp
    src/game.cpp
    src/main.cpp
    # navmesh (it would be nice to port it to vcpkg)
    vendor/navmesh/source/path_finder.cpp
    vendor/navmesh/source/point.cpp
    vendor/navmesh/source/polygon.cpp
    vendor/navmesh/source/cone_of_vision.cpp
    vendor/navmesh/source/segment.cpp
)

target_link_libraries(
    ${PROJECT_NAME}
    PRIVATE
    SDL2::SDL2
    SDL2::SDL2main
    docopt
    yaml-cpp
    Fontconfig::Fontconfig
    SDL2::SDL2_ttf
)