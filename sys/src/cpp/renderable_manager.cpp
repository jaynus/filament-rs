//
// Created by jaynus on 4/22/20.
//

#include "renderable_manager.h"
#include <filament/RenderableManager.h>



namespace helpers {
    class BuilderProxy {
    public:
        BuilderProxy(filament::RenderableManager::Builder * ptr): ptr(ptr) {}
        ~BuilderProxy() {delete ptr; }

        filament::RenderableManager::Builder * ptr;
    };

    BuilderProxy *renderable_builder_new(size_t count) {
        return new BuilderProxy(new filament::RenderableManager::Builder(count));
    }

    void renderable_builder_destroy(BuilderProxy *proxy) {
        delete proxy;
    }

    void renderable_builder_geometry(BuilderProxy * proxy,
                                    size_t index, filament::backend::PrimitiveType type,
                                    filament::VertexBuffer* vertices, filament::IndexBuffer* indices
    ) {
        proxy->ptr->geometry(index, type, vertices, indices);
    }

    void renderable_builder_culling(BuilderProxy * proxy, bool value) {
        proxy->ptr->culling(value);
    }
    void renderable_builder_castShadows(BuilderProxy * proxy, bool value) {
        proxy->ptr->castShadows(value);
    }
    void renderable_builder_receiveShadows(BuilderProxy * proxy, bool value) {
        proxy->ptr->receiveShadows(value);
    }
    void renderable_builder_screenSpaceContactShadows(BuilderProxy * proxy, bool value) {
        proxy->ptr->screenSpaceContactShadows(value);
    }
    void renderable_builder_morphing(BuilderProxy * proxy, bool value) {
        proxy->ptr->morphing(value);
    }

    void renderable_builder_material(BuilderProxy * proxy, size_t index,
            filament::MaterialInstance * material) {
        proxy->ptr->material(index, material);
    }

    bool renderable_builder_build(BuilderProxy * proxy, filament::Engine &engine, utils::Entity entity) {
        return proxy->ptr->build(engine, entity) == 0;
    }

}