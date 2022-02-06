# RTEE

RTTE is attempt to create 2D open source game engine for real time tactics games. Games like Commandos Behind Enemy Lines and Beyond the Call of Duty. Every game is set of .yml files that defines base functionality.

## Goals

- keep the code simple as possible
- map editor
- cooperation
- ...

## Building & running

- clone repo
- install cmake
- run ```./vendor/vcpkg/bootstrap-vcpkg.sh```
- run ```./vendor/vcpkg/vcpkg.exe install```
- run ```cmake -B build -S . -DCMAKE_TOOLCHAIN_FILE=./vendor/vcpkg/scripts/buildsystems/vcpkg.cmake```
- run ```cmake --build build```
- run ```./build/debug/rtte.exe```
