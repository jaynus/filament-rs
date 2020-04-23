//
// Created by jaynus on 4/21/20.
//

#ifndef FILAMENT_RS_HELPERS_H
#define FILAMENT_RS_HELPERS_H

#include <math/vec3.h>
#include <math/vec4.h>
#include <backend/DriverEnums.h>
#include <backend/BufferDescriptor.h>
#include <backend/PixelBufferDescriptor.h>

#include "renderable_manager.h"

namespace helpers {
    filament::backend::SamplerParams make_SamplerParams(
            filament::backend::SamplerMagFilter filterMag,
            filament::backend::SamplerMinFilter filterMin,
            filament::backend::SamplerWrapMode wrapS,
            filament::backend::SamplerWrapMode wrapT,
            filament::backend::SamplerWrapMode wrapR,
            uint8_t anisotropyLog2,
            filament::backend::SamplerCompareMode compareMode,
            filament::backend::SamplerCompareFunc compareFunc);

    filament::backend::PixelBufferDescriptor make_PixelBufferDescriptor(
            void const *buffer, size_t size, filament::backend::BufferDescriptor::Callback callback, void *user,
            filament::backend::PixelDataFormat format,
            filament::backend::PixelDataType ty
            );

}
#endif //FILAMENT_RS_HELPERS_H
