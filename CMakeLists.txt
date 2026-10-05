cmake_minimum_required(VERSION 4.3)

project(rtte CXX)

set(CMAKE_CXX_STANDARD 17)

find_package(raylib CONFIG REQUIRED)
find_package(toml11 CONFIG REQUIRED)
find_package(GTest CONFIG REQUIRED)

add_library(rtte_core STATIC src/animation.cpp src/sprite.cpp src/navmesh.cpp src/mission.cpp)
target_include_directories(rtte_core PUBLIC src)
target_link_libraries(rtte_core PUBLIC raylib toml11::toml11)

add_executable(rtte src/main.cpp)
target_link_libraries(rtte PRIVATE rtte_core)

enable_testing()
include(GoogleTest)

add_executable(rtte_tests tests/animation_test.cpp tests/sprite_test.cpp tests/navmesh_test.cpp tests/mission_test.cpp)
target_link_libraries(rtte_tests PRIVATE rtte_core GTest::gtest_main)
gtest_discover_tests(rtte_tests WORKING_DIRECTORY ${CMAKE_SOURCE_DIR} DISCOVERY_MODE PRE_TEST)
