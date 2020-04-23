//
// Created by jaynus on 4/21/20.
//

#include "helpers.h"

namespace helpers {
    filament::backend::SamplerParams make_SamplerParams(
            filament::backend::SamplerMagFilter filterMag,
            filament::backend::SamplerMinFilter filterMin,
            filament::backend::SamplerWrapMode wrapS,
            filament::backend::SamplerWrapMode wrapT,
            filament::backend::SamplerWrapMode wrapR,
            uint8_t anisotropyLog2,
            filament::backend::SamplerCompareMode compareMode,
            filament::backend::SamplerCompareFunc compareFunc)
    {
        filament::backend::SamplerParams params;
        params.filterMag = filterMag;
        params.filterMin = filterMin;
        params.wrapS = wrapS;
        params.wrapT = wrapT;
        params.wrapR = wrapR;
        params.anisotropyLog2 = anisotropyLog2;
        params.compareMode = compareMode;
        params.padding0 = 0;
        params.compareFunc = compareFunc;
        params.padding1 = 0;
        params.padding2 = 0;
        return params;
    }

    filament::backend::PixelBufferDescriptor make_PixelBufferDescriptor(
            void const *buffer, size_t size, filament::backend::BufferDescriptor::Callback callback, void *user,
            filament::backend::PixelDataFormat format,
            filament::backend::PixelDataType ty
    ) {
        return filament::backend::PixelBufferDescriptor(buffer, size, format, ty, callback, user);
    }
}