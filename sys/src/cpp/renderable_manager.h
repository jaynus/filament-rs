//
// Created by jaynus on 4/22/20.
//

#ifndef FILAMENT_RS_RENDERABLE_MANAGER_H
#define FILAMENT_RS_RENDERABLE_MANAGER_H

#include <utils/Entity.h>
#include <filament/Engine.h>
#include <backend/DriverEnums.h>

namespace filament {
    class VertexBuffer;
    class IndexBuffer;
    class MaterialInstance;
}

namespace helpers {
    class BuilderProxy;

    BuilderProxy *renderable_builder_new(size_t count);
    void renderable_builder_destroy(BuilderProxy *proxy);

    void renderable_builder_geometry(BuilderProxy * build,
            size_t index, filament::backend::PrimitiveType type,
            filament::VertexBuffer* vertices, filament::IndexBuffer* indices);

    void renderable_builder_culling(BuilderProxy * build, bool culling);

    void renderable_builder_castShadows(BuilderProxy * build, bool);
    void renderable_builder_receiveShadows(BuilderProxy * build, bool);
    void renderable_builder_screenSpaceContactShadows(BuilderProxy * build, bool);

    void renderable_builder_morphing(BuilderProxy * build, bool);

    void renderable_builder_material(BuilderProxy * build, size_t index, filament::MaterialInstance * material);

    bool renderable_builder_build(BuilderProxy * proxy, filament::Engine &engine, utils::Entity entity);
}

#endif //FILAMENT_RS_RENDERABLE_MANAGER_H
