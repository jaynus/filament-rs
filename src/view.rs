use filament_sys::{ffi, Viewport};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use std::sync::Arc;

use crate::{
    camera::Camera,
    engine::{Engine, EngineError},
    impl_handle,
    scene::Scene,
};

pub use ffi::filament_View_AmbientOcclusionOptions as AmbientOcclusionOptions;
pub use ffi::filament_View_BloomOptions as BloomOptions;
pub use ffi::filament_View_DynamicResolutionOptions as DynamicResolutionOptions;
pub use ffi::filament_View_FogOptions as FogOptions;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive)]
#[repr(u8)]
pub enum AmbientOcclusion {
    None = ffi::filament_View_AmbientOcclusion_NONE,
    SSAO = ffi::filament_View_AmbientOcclusion_SSAO,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive)]
#[repr(u8)]
pub enum AntiAliasing {
    NONE = ffi::filament_View_AntiAliasing_NONE,
    FXAA = ffi::filament_View_AntiAliasing_FXAA,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive)]
#[repr(u8)]
pub enum Dithering {
    NONE = ffi::filament_View_Dithering_NONE,
    TEMPORAL = ffi::filament_View_Dithering_TEMPORAL,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive)]
#[repr(u8)]
pub enum ToneMapping {
    LINEAR = ffi::filament_View_ToneMapping_LINEAR,
    AC = ffi::filament_View_ToneMapping_ACES,
}

impl_handle!(View, ffi::filament_View);
impl Drop for View {
    fn drop(&mut self) {
        if let Some(ptr) = Arc::get_mut(&mut self.ptr) {
            log::trace!(target: "drop", "drop {}", std::any::type_name::<Self>());
            unsafe { self.engine.as_raw_mut().destroy15(*ptr) }
        }
    }
}

impl View {
    pub fn set_ambient_occlusion(&mut self, ao: AmbientOcclusion) {
        unsafe { self.as_raw_mut().setAmbientOcclusion(ao.to_u8().unwrap()) }
    }
    pub fn ambient_occlusion(&self) -> AmbientOcclusion {
        AmbientOcclusion::from_u8(unsafe { self.as_raw_ref().getAmbientOcclusion() }).unwrap()
    }

    pub fn set_ambient_occlusion_options(&mut self, options: AmbientOcclusionOptions) {
        unsafe { self.as_raw_mut().setAmbientOcclusionOptions(&options as _) }
    }
    pub fn ambient_occlusion_options(&self) -> AmbientOcclusionOptions {
        unsafe { *self.as_raw_ref().getAmbientOcclusionOptions() }
    }

    pub fn set_antialiasing(&mut self, ao: AntiAliasing) {
        unsafe { self.as_raw_mut().setAntiAliasing(ao.to_u8().unwrap()) }
    }
    pub fn antialiasing(&self) -> AntiAliasing {
        AntiAliasing::from_u8(unsafe { self.as_raw_ref().getAntiAliasing() }).unwrap()
    }

    pub fn set_dithering(&mut self, ao: Dithering) {
        unsafe { self.as_raw_mut().setDithering(ao.to_u8().unwrap()) }
    }
    pub fn dithering(&self) -> Dithering {
        Dithering::from_u8(unsafe { self.as_raw_ref().getDithering() }).unwrap()
    }

    pub fn set_tone_mapping(&mut self, ao: ToneMapping) {
        unsafe { self.as_raw_mut().setToneMapping(ao.to_u8().unwrap()) }
    }
    pub fn tone_mapping(&self) -> ToneMapping {
        ToneMapping::from_u8(unsafe { self.as_raw_ref().getToneMapping() }).unwrap()
    }

    pub fn set_name<S>(&mut self, name: S)
    where
        S: AsRef<str>,
    {
        unsafe {
            self.as_raw_mut()
                .setName(std::ffi::CString::new(name.as_ref()).unwrap().as_ptr())
        }
    }
    pub fn name(&self) -> String {
        unsafe {
            std::ffi::CString::from_raw(self.as_raw_ref().getName() as *mut _)
                .into_string()
                .unwrap()
        }
    }

    pub fn set_scene(&mut self, scene: &Scene) {
        unsafe { self.as_raw_mut().setScene(scene.as_raw_ptr()) }
    }
    pub fn set_camera(&mut self, camera: &Camera) {
        unsafe { self.as_raw_mut().setCamera(camera.as_raw_ptr()) }
    }
    pub fn set_viewport(&mut self, viewport: Viewport) {
        unsafe { self.as_raw_mut().setViewport(&viewport as *const _ as _) }
    }
    // TODO: getters, how do we return managed handle objects that dont destruct?
    // TODO: This may require the engine to store an index of all created objects

    pub fn new(engine: &mut Engine) -> Result<Self, EngineError> {
        let ptr = unsafe { engine.as_raw_mut().createView() };

        if ptr.is_null() {
            Err(EngineError::CreationFailed)
        } else {
            Ok(Self {
                ptr: Arc::new(ptr),
                engine: engine.clone(),
            })
        }
    }
}
