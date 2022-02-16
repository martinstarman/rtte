#include "util.h"

#include <fontconfig/fontconfig.h>
#include <string>

namespace rtte::util
{
    std::string GetFontPath()
    {
        FcInit();

        // TODO: Fontconfig error: Cannot load default config file: No such file: (null)
        FcConfig *config = FcInitLoadConfigAndFonts();
        FcPattern *pattern = FcNameParse((const FcChar8 *)"Arial");

        FcConfigSubstitute(config, pattern, FcMatchPattern);
        FcDefaultSubstitute(pattern);

        FcResult result;
        FcPattern *fontMatch = FcFontMatch(config, pattern, &result);

        std::string fontPath;
        FcChar8 *path;

        if (fontMatch && FcPatternGetString(fontMatch, FC_FILE, 0, &path) == FcResultMatch)
        {
            fontPath = std::string(reinterpret_cast<char *>(path));
        }

        FcPatternDestroy(fontMatch);
        FcPatternDestroy(pattern);
        FcConfigDestroy(config);
        FcFini();

        return fontPath;
    }
}