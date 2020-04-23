//
// Created by jaynus on 4/22/20.
//

#ifndef FILAMENT_RS_MATERIALS_H
#define FILAMENT_RS_MATERIALS_H

#include <filament/Engine.h>

namespace filament {
    class Material;
    class MaterialInstance;
    class TextureSampler;
    class Texture;
}

namespace helpers {
    filament::Material * material_build(filament::Engine & engine, void *buffer, size_t len);
    filament::MaterialInstance * material_getDefaultInstance(filament::Material *material);

    void material_instance_setParameter_texture(filament::MaterialInstance * instance, const char *name,
                                                filament::Texture const* texture, filament::TextureSampler const& sampler);
}



#endif //FILAMENT_RS_MATERIALS_H
