#ifndef ENEMY_H
#define ENEMY_H

#include <cone_of_vision.h>
#include "entity.h"
#include <pointf.h>
#include <vector>

namespace rtte
{
    class Enemy : public Entity
    {
    public:
        Enemy(float x, float y);
        ~Enemy();
        void Update(float dt) override;
        void Render() override;
        int GetYIndex() override;

    private:
        NavMesh::ConeOfVision m_cone;
        std::vector<NavMesh::PointF> m_vision;
    };
}

#endif
