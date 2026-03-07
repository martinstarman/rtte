g++                          \
  -o build/rtte.exe          \
  src/entity.cpp             \
  src/game.cpp               \
  src/main.cpp               \
  -I external/toml11/include \
  -I include                 \
  -L lib                     \
  -lraylib                   \
  -lgdi32                    \
  -lwinmm
