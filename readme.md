# RTTE
RTTE is attempt to create 2D isometric real time tactics game engine for games like Commandos, Desperados, Helldorado, Robin Hood: The Legend of Sherwood or Chicago 1930.

## Building
Dependencies are installed automatically by [vcpkg](https://vcpkg.io) from `vcpkg.json` (manifest mode).
```
cmake -B build -DCMAKE_TOOLCHAIN_FILE=<vcpkg-root>/scripts/buildsystems/vcpkg.cmake
cmake --build build
```

## Testing
```
ctest --test-dir build -C Debug --output-on-failure
```
