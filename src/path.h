#ifndef PATH_H
#define PATH_H

#include <path_finder.h>

namespace rtte
{
    class Path
    {
    public:
        Path();
        ~Path();
        void Find(int x1, int y1, int x2, int y2);

    private:
        NavMesh::PathFinder m_pathFinder;
    };
}

#endif
