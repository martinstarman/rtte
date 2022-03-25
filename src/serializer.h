#ifndef SERIALIZER_H
#define SERIALIZER_H

#include <string>

namespace rtte
{
    class Serializer
    {
    public:
        Serializer();
        ~Serializer();
        void Deserialize(const std::string &file);
    };
}

#endif
