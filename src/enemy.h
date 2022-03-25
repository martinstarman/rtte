#ifndef ENEMY_H
#define ENEMY_H

#include <cone_of_vision.h>
#include "entity.h"
#include <pointf.h>
#include <polygon.h>
#include <vector>

namespace rtte
{
    class Enemy : public Entity
    {
    public:
        Enemy(float x, float y, const std::vector<NavMesh::Polygon> &polygons);
        ~Enemy();
        void Update(float dt) override;
        void Render() override;

    private:
        NavMesh::ConeOfVision m_cone;
        std::vector<NavMesh::PointF> m_vision;
    };
}

#endif
