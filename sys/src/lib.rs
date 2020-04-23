#![allow(dead_code)]

use std::convert::TryInto;

pub mod ffi {
    #![allow(
        non_upper_case_globals,
        non_snake_case,
        non_camel_case_types,
        dead_code
    )]
    include!("bindings.rs");

    // TODO: Not sure why bindgen gets this wrong
    pub type filament_math_mat4f = [u32; 16];
}

impl Default for ffi::filament_Renderer_ClearOptions {
    fn default() -> Self {
        Self {
            clearColor: [0.0, 0.0, 1.0, 1.0].into_filament(),
            clear: true,
            discard: true,
        }
    }
}

pub use ffi::filament_backend_Viewport as Viewport;

impl Viewport {
    #[inline]
    pub fn new(left: i32, bottom: i32, width: u32, height: u32) -> Self {
        Self {
            left,
            bottom,
            width,
            height,
        }
    }

    #[inline]
    pub fn right(&self) -> i32 {
        let width: i32 = self.width.try_into().unwrap();
        self.left + width
    }

    #[inline]
    pub fn top(&self) -> i32 {
        let height: i32 = self.height.try_into().unwrap();
        self.bottom + height
    }
}

pub trait IntoFilament<D> {
    fn into_filament(self) -> D;
}

pub trait FromFilament<D> {
    fn into_array(self) -> D;
}

impl IntoFilament<ffi::filament_math_float3> for [f32; 3] {
    #[inline(always)]
    fn into_filament(self) -> ffi::filament_math_float3 {
        let mut r: [u32; 3] = [0; 3];

        unsafe { std::ptr::copy_nonoverlapping(self.as_ptr(), r.as_mut_ptr() as *mut _, 3) }

        r
    }
}

impl FromFilament<[f32; 3]> for ffi::filament_math_float3 {
    fn into_array(self) -> [f32; 3] {
        unsafe { std::slice::from_raw_parts(self.as_ptr() as *const _, 3) }
            .try_into()
            .unwrap()
    }
}

impl IntoFilament<ffi::filament_math_float4> for [f32; 4] {
    #[inline(always)]
    fn into_filament(self) -> ffi::filament_math_float4 {
        let mut r: [u32; 4] = [0; 4];

        unsafe { std::ptr::copy_nonoverlapping(self.as_ptr(), r.as_mut_ptr() as *mut _, 4) }

        r
    }
}
impl FromFilament<[f32; 4]> for ffi::filament_math_float4 {
    fn into_array(self) -> [f32; 4] {
        unsafe { std::slice::from_raw_parts(self.as_ptr() as *const _, 4) }
            .try_into()
            .unwrap()
    }
}

impl IntoFilament<ffi::filament_math_mat4f> for [f32; 16] {
    #[inline(always)]
    fn into_filament(self) -> ffi::filament_math_mat4f {
        let mut r: [u32; 16] = [0; 16];

        unsafe { std::ptr::copy_nonoverlapping(self.as_ptr(), r.as_mut_ptr() as *mut _, 16) }

        r
    }
}
impl FromFilament<[f32; 16]> for ffi::filament_math_mat4f {
    fn into_array(self) -> [f32; 16] {
        unsafe { std::slice::from_raw_parts(self.as_ptr() as *const _, 16) }
            .try_into()
            .unwrap()
    }
}

#[no_mangle]
/// A callback from Filament to de-allocate a buffer (after it has been copied to the GPU).
pub unsafe extern "C" fn deallocate_rust_buffer(
    ptr: *mut std::ffi::c_void,
    size: u64,
    _user: *mut std::ffi::c_void,
) {
    log::trace!("deallocate_rust_buffer @ {:?}, size={}", ptr, size);
    let size = size as usize;
    std::mem::drop(Vec::from_raw_parts(ptr, size, size));
}

// This is implemented in-case the user forgets to call forget
impl Drop for ffi::filament_backend_BufferDescriptor {
    fn drop(&mut self) {
        log::trace!(target: "drop", "drop {}", std::any::type_name::<Self>());
        unsafe { crate::deallocate_rust_buffer(self.buffer, self.size, self.user) }
    }
}

// This is implemented in-case the user forgets to call forget
impl Drop for ffi::filament_backend_PixelBufferDescriptor {
    fn drop(&mut self) {
        log::trace!(target: "drop", "drop {}", std::any::type_name::<Self>());
        unsafe {
            crate::deallocate_rust_buffer(self._base.buffer, self._base.size, self._base.user)
        }
    }
}

impl Default for ffi::utils_Entity {
    fn default() -> Self {
        Self { mIdentity: 0 }
    }
}

pub struct TextureSamplerBuilder {
    pub filter_mag: ffi::filament_backend_SamplerMagFilter,
    pub filter_min: ffi::filament_backend_SamplerMinFilter,
    pub wrap_s: ffi::filament_backend_SamplerWrapMode,
    pub wrap_t: ffi::filament_backend_SamplerWrapMode,
    pub wrap_r: ffi::filament_backend_SamplerWrapMode,
    pub anisotropy_log2: u8,
    pub compare_mode: ffi::filament_backend_SamplerCompareMode,
    pub compare_func: ffi::filament_backend_SamplerCompareFunc,
}
impl TextureSamplerBuilder {
    pub fn filter_mag(mut self, filter: ffi::filament_backend_SamplerMagFilter) -> Self {
        self.filter_mag = filter;
        self
    }
    pub fn filter_min(mut self, filter: ffi::filament_backend_SamplerMinFilter) -> Self {
        self.filter_min = filter;
        self
    }
    pub fn wrap_s(mut self, mode: ffi::filament_backend_SamplerWrapMode) -> Self {
        self.wrap_s = mode;
        self
    }
    pub fn wrap_t(mut self, mode: ffi::filament_backend_SamplerWrapMode) -> Self {
        self.wrap_t = mode;
        self
    }
    pub fn wrap_r(mut self, mode: ffi::filament_backend_SamplerWrapMode) -> Self {
        self.wrap_r = mode;
        self
    }
    pub fn anisotropy_log2(mut self, anisotropy_log2: u8) -> Self {
        self.anisotropy_log2 = anisotropy_log2;
        self
    }
    pub fn compare_mode(mut self, mode: ffi::filament_backend_SamplerCompareMode) -> Self {
        self.compare_mode = mode;
        self
    }
    pub fn compare_func(mut self, mode: ffi::filament_backend_SamplerCompareFunc) -> Self {
        self.compare_func = mode;
        self
    }

    fn build(self) -> ffi::filament_TextureSampler {
        ffi::filament_TextureSampler {
            mSamplerParams: unsafe {
                ffi::helpers_make_SamplerParams(
                    self.filter_mag,
                    self.filter_min,
                    self.wrap_s,
                    self.wrap_t,
                    self.wrap_r,
                    self.anisotropy_log2,
                    self.compare_mode,
                    self.compare_func,
                )
            },
        }
    }
}
impl Default for TextureSamplerBuilder {
    fn default() -> Self {
        Self {
            filter_mag: ffi::filament_backend_SamplerMagFilter::NEAREST,
            filter_min: ffi::filament_backend_SamplerMinFilter::NEAREST,
            wrap_s: ffi::filament_backend_SamplerWrapMode::CLAMP_TO_EDGE,
            wrap_t: ffi::filament_backend_SamplerWrapMode::CLAMP_TO_EDGE,
            wrap_r: ffi::filament_backend_SamplerWrapMode::CLAMP_TO_EDGE,
            anisotropy_log2: 0,
            compare_mode: ffi::filament_backend_SamplerCompareMode::NONE,
            compare_func: ffi::filament_backend_SamplerCompareFunc::LE,
        }
    }
}
impl ffi::filament_TextureSampler {
    pub fn builder() -> TextureSamplerBuilder {
        TextureSamplerBuilder::default()
    }
}
impl Default for ffi::filament_TextureSampler {
    fn default() -> Self {
        Self::builder().build()
    }
}
