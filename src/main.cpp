#include <docopt.h>
#include "game.h"
#include <SDL.h>
#include <SDL_ttf.h>
#include "serializer.h"
#include <string>
#include "util.h"

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

    std::string font = rtte::util::GetFontPath();

    if (font == "")
    {
        SDL_Log("Unable to load any font");
        return 1;
    }

    if (SDL_Init(SDL_INIT_VIDEO) != 0)
    {
        SDL_Log("Unable to initialize SDL: %s", SDL_GetError());
        return 1;
    }

    if (TTF_Init() == -1)
    {
        SDL_Log("Unable to initialize SDL_ttf: %s", TTF_GetError());
        return 1;
    }

    rtte::Game *game = rtte::Game::Get();
    rtte::Serializer serializer;
    serializer.Deserialize(args["<mission-file-path>"].asString());

    bool isRunning = true;
    int time = 0;
    int lastTime = 0;
    float dt = 0;
    SDL_Event event;

    while (isRunning)
    {
        time = SDL_GetTicks();

        while (SDL_PollEvent(&event))
        {
            if (event.type == SDL_QUIT)
            {
                isRunning = false;
            }
        }

        dt += (time - lastTime) / 1000.0f;

        if (dt > 1.0f / 60.0f)
        {
            game->Update(dt);
            lastTime = time;
            dt = 0;
        }
    }

    TTF_Quit();
    SDL_Quit();

    return 0;
}
