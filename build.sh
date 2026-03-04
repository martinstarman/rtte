g++                          \
  -o build/rtte.exe          \
  src/main.cpp               \
  src/entity.cpp             \
  -I external/toml11/include \
  -I include                 \
  -L lib                     \
  -lraylib                   \
  -lgdi32                    \
  -lwinmm
