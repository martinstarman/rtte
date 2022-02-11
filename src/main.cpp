#include <docopt.h>
#include "game.h"
#include <SDL.h>
#include "serializer.h"

static const char USAGE[] =
    R"(RTTE.

    Usage:
      rtte <mission-file-path>
      rtte (-h | --help)
      rtte (-v | --version)

    Options:
      -h --help     Show this screen.
      -v --version  Show version.
)";

int main(int argc, char *argv[])
{
    std::map<std::string, docopt::value> args = docopt::docopt(
        USAGE,
        {argv + 1, argv + argc},
        true,   // show help
        "0.0.1" // version string
    );

    if (SDL_Init(SDL_INIT_VIDEO) != 0)
    {
        SDL_Log("Unable to initialize SDL: %s", SDL_GetError());
        return 1;
    }

    rtte::Game *game = rtte::Game::Get();
    rtte::Serializer serializer;
    serializer.Deserialize(args["<mission-file-path>"].asString());

    bool isRunning = true;
    SDL_Event event;

    while (isRunning)
    {
        while (SDL_PollEvent(&event))
        {
            if (event.type == SDL_QUIT)
            {
                isRunning = false;
            }
        }

        game->Update();
    }

    SDL_Quit();

    return 0;
}
