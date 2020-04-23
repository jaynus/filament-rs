use filament_sys::ffi;
use std::{convert::TryInto, sync::Arc};

use crate::{
    engine::{Engine, EngineError},
    impl_handle,
};

pub use ffi::{
    filament_TextureSampler as TextureSampler,
    filament_TextureSampler_CompareFunc as SamplerCompareFunc,
    filament_TextureSampler_CompareMode as SamplerCompareMode,
    filament_TextureSampler_MagFilter as SamplerMagFilter,
    filament_TextureSampler_MinFilter as SamplerMinFilter,
    filament_TextureSampler_WrapMode as SamplerWrapMode,
    filament_Texture_CompressedType as CompressedType, filament_Texture_CubemapFace as CubeMapFace,
    filament_Texture_Format as TextureFormat, filament_Texture_InternalFormat as InternalFormat,
    filament_Texture_Sampler as SamplerType, filament_Texture_Swizzle as TextureSwizzle,
    filament_Texture_Type as TextureType, filament_Texture_Usage as TextureUsage,
    filament_backend_PixelBufferDescriptor as PixelBufferDescriptor,
    filament_backend_PixelDataFormat as PixelDataFormat,
    filament_backend_PixelDataType as PixelDataType,
};

impl_handle!(Texture, ffi::filament_Texture);
impl Drop for Texture {
    fn drop(&mut self) {
        if let Some(ptr) = Arc::get_mut(&mut self.ptr) {
            log::trace!(target: "drop", "drop {}", std::any::type_name::<Self>());
            unsafe { self.engine.as_raw_mut().destroy13(*ptr) }
        }
    }
}
impl Texture {
    pub fn is_format_supported(engine: &Engine, format: InternalFormat) -> bool {
        unsafe { ffi::filament_Texture::isTextureFormatSupported(engine.as_raw_ptr(), format) }
    }

    pub fn compute_data_size(
        format: TextureFormat,
        ty: TextureType,
        stride: usize,
        height: usize,
        alignment: usize,
    ) -> usize {
        unsafe {
            ffi::filament_Texture::computeTextureDataSize(
                format,
                ty,
                stride.try_into().unwrap(),
                height.try_into().unwrap(),
                alignment.try_into().unwrap(),
            )
            .try_into()
            .unwrap()
        }
    }

    #[inline]
    pub fn width(&self, level: usize) -> usize {
        unsafe {
            self.as_raw_ref()
                .getWidth(level.try_into().unwrap())
                .try_into()
                .unwrap()
        }
    }

    #[inline]
    pub fn height(&self, level: usize) -> usize {
        unsafe {
            self.as_raw_ref()
                .getHeight(level.try_into().unwrap())
                .try_into()
                .unwrap()
        }
    }

    #[inline]
    pub fn depth(&self, level: usize) -> usize {
        unsafe {
            self.as_raw_ref()
                .getDepth(level.try_into().unwrap())
                .try_into()
                .unwrap()
        }
    }

    #[inline]
    pub fn levels(&self) -> usize {
        unsafe { self.as_raw_ref().getLevels().try_into().unwrap() }
    }

    #[inline]
    pub fn target(&self) -> SamplerType {
        unsafe { self.as_raw_ref().getTarget() }
    }

    #[inline]
    pub fn format(&self) -> InternalFormat {
        unsafe { self.as_raw_ref().getFormat() }
    }

    pub fn set<T: Sized>(
        &mut self,
        level: usize,
        buffer: Vec<T>,
        format: PixelDataFormat,
        ty: PixelDataType,
    ) {
        let mut desc = make_pixel_buffer_descriptor(buffer, format, ty);
        unsafe {
            let engine = self.engine.as_raw_ptr();
            self.as_raw_mut()
                .setImage(engine, level.try_into().unwrap(), &mut desc as *mut _)
        };
        std::mem::forget(desc);
    }

    #[inline]
    pub fn set_external_image<T: Sized>(&mut self, engine: &mut Engine, data: &[T]) {
        unsafe {
            self.as_raw_mut()
                .setExternalImage(engine.as_raw_ptr(), data.as_ptr() as *mut _)
        }
    }

    #[inline]
    pub fn builder() -> Result<TextureBuilder, EngineError> {
        TextureBuilder::new()
    }
}

pub struct TextureBuilder {
    inner: ffi::filament_Texture_Builder,
}
impl Drop for TextureBuilder {
    fn drop(&mut self) {
        log::trace!(target: "drop", "drop {}", std::any::type_name::<Self>());
        unsafe { self.inner.destruct() }
    }
}
impl TextureBuilder {
    pub fn build(mut self, engine: &mut Engine) -> Result<Texture, EngineError> {
        let ptr = unsafe { self.inner.build(engine.as_raw_ptr()) };
        if ptr.is_null() {
            Err(EngineError::CreationFailed)
        } else {
            Ok(Texture {
                ptr: Arc::new(ptr),
                engine: engine.clone(),
            })
        }
    }

    #[inline]
    pub fn width(mut self, width: usize) -> Self {
        unsafe { self.inner.width(width.try_into().unwrap()) };
        self
    }

    #[inline]
    pub fn height(mut self, height: usize) -> Self {
        unsafe { self.inner.height(height.try_into().unwrap()) };
        self
    }

    #[inline]
    pub fn depth(mut self, depth: usize) -> Self {
        unsafe { self.inner.depth(depth.try_into().unwrap()) };
        self
    }

    #[inline]
    pub fn levels(mut self, levels: usize) -> Self {
        unsafe { self.inner.levels(levels.try_into().unwrap()) };
        self
    }

    #[inline]
    pub fn sampler(mut self, sampler: SamplerType) -> Self {
        unsafe { self.inner.sampler(sampler) };
        self
    }

    #[inline]
    pub fn format(mut self, internal_format: InternalFormat) -> Self {
        unsafe { self.inner.format(internal_format) };
        self
    }

    #[inline]
    pub fn usage(mut self, usage: TextureUsage) -> Self {
        unsafe { self.inner.usage(usage) };
        self
    }

    #[inline]
    pub fn swizzle(
        mut self,
        r: TextureSwizzle,
        g: TextureSwizzle,
        b: TextureSwizzle,
        a: TextureSwizzle,
    ) -> Self {
        unsafe { self.inner.swizzle(r, g, b, a) };
        self
    }

    #[inline]
    pub fn import(mut self, id: isize) -> Self {
        unsafe { self.inner.import(id) };
        self
    }

    #[inline]
    pub fn new() -> Result<Self, EngineError> {
        Ok(Self {
            inner: unsafe { ffi::filament_Texture_Builder::new() },
        })
    }
}

pub fn make_pixel_buffer_descriptor<T: Sized>(
    mut data: Vec<T>,
    format: PixelDataFormat,
    ty: PixelDataType,
) -> PixelBufferDescriptor {
    let desc = unsafe {
        ffi::helpers_make_PixelBufferDescriptor(
            data.as_mut_ptr() as *mut _,
            (data.len() * std::mem::size_of::<T>()).try_into().unwrap(),
            Some(filament_sys::deallocate_rust_buffer),
            std::ptr::null_mut(),
            format,
            ty,
        )
    };
    std::mem::forget(data);
    desc
}

#[cfg(test)]
mod tests {
    use super::{InternalFormat, Texture, TextureSampler};
    use crate::{engine::Engine, sys::ffi, Backend};

    #[test]
    fn texture_creation() {
        let mut engine = Engine::new(Backend::NOOP).unwrap();
        let sampler = TextureSampler::default();
        let mut texture = Texture::builder()
            .unwrap()
            .width(123)
            .height(456)
            .format(InternalFormat::RGB8)
            .build(&mut engine)
            .unwrap();

        assert_eq!(texture.width(0), 123);
        assert_eq!(texture.height(0), 456);
        assert_eq!(texture.format(), InternalFormat::RGB8);

        unsafe { assert!(ffi::test_texture_eq(texture.as_raw_ptr())) };
    }
}
