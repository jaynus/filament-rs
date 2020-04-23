//
// Created by jaynus on 4/22/20.
//

#include "materials.h"
#include <filament/Material.h>
#include <filament/Texture.h>
#include <filament/TextureSampler.h>
#include <filament/MaterialInstance.h>

namespace helpers {
    filament::MaterialInstance * material_getDefaultInstance(filament::Material *material) {
        return material->getDefaultInstance();
    }

    filament::Material * material_build(filament::Engine & engine, void *buffer, size_t len) {
        return filament::Material::Builder().package(buffer, len).build(engine);
    }

    void material_instance_setParameter_texture(filament::MaterialInstance * instance, const char *name,
                                                filament::Texture const* texture, filament::TextureSampler const& sampler) {
        instance->setParameter(name, texture, sampler);
    }
}